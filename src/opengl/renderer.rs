// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::Cell;
use std::cell::RefCell;
use std::mem::needs_drop;
use std::mem::offset_of;
use std::mem::replace;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::ops::Add;
use std::ops::DerefMut as _;
use std::ops::Sub;
use std::rc::Rc;

use anyhow::Context as _;
use anyhow::Result;

use xgl::sys;
use xgl::sys::Gl as _;
use xgl::vertex::Attrib;
use xgl::vertex::AttribType;
use xgl::vertex::Attribs;
use xgl::MatrixStack;
use xgl::Program;
use xgl::Shader;
use xgl::VertexArray;
use xgl::VertexBuffer;

use crate::guard::Guard;
use crate::winit::Context;
use crate::Point;
use crate::Rect;

use super::empty_texture;
use super::Mat4f;
use super::Texture;


/// The capacity of our vertex buffer.
// TODO: We should consider sizing it more dynamically and just making
//       this an upper limit instead.
const VERTEX_BUFFER_CAPACITY: usize = 1024;


#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
struct Vertex {
  // texture coordinates
  u: i16,
  v: i16,

  // color
  r: u8,
  g: u8,
  b: u8,
  a: u8,

  // position
  x: f32,
  y: f32,
}

impl Attribs for Vertex {
  const ATTRIBS: &'static [(AttribType, Attrib)] = &[
    (
      AttribType::Texture,
      Attrib {
        size: 2,
        type_: sys::Type::Short,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: 0,
      },
    ),
    (
      AttribType::Color,
      Attrib {
        size: 4,
        type_: sys::Type::UnsignedByte,
        // By performing normalization we effectively map:
        // 0   -> 0.0
        // ... -> ...
        // 255 -> 1.0
        normalize: true,
        stride: size_of::<Self>() as _,
        offset: offset_of!(Self, r) as _,
      },
    ),
    (
      AttribType::Position,
      Attrib {
        size: 2,
        type_: sys::Type::Float,
        normalize: false,
        stride: size_of::<Self>() as _,
        offset: offset_of!(Self, x) as _,
      },
    ),
  ];
}


#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C, packed)]
pub(crate) struct Color {
  pub(crate) r: u8,
  pub(crate) g: u8,
  pub(crate) b: u8,
  pub(crate) a: u8,
}

impl Color {
  /// A `const` version of `Add::add`.
  const fn cadd(self, other: Color) -> Self {
    Self {
      r: self.r.saturating_add(other.r),
      g: self.g.saturating_add(other.g),
      b: self.b.saturating_add(other.b),
      a: self.a.saturating_add(other.a),
    }
  }

  /// A `const` version of `Sub::sub`.
  pub const fn csub(self, other: Color) -> Self {
    Self {
      r: self.r.saturating_sub(other.r),
      g: self.g.saturating_sub(other.g),
      b: self.b.saturating_sub(other.b),
      a: self.a.saturating_sub(other.a),
    }
  }


  #[inline]
  pub(crate) const fn black() -> Self {
    Self {
      r: u8::MIN,
      g: u8::MIN,
      b: u8::MIN,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn white() -> Self {
    Self {
      r: u8::MAX,
      g: u8::MAX,
      b: u8::MAX,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn red() -> Self {
    Self {
      r: u8::MAX,
      g: u8::MIN,
      b: u8::MIN,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn green() -> Self {
    Self {
      r: u8::MIN,
      g: u8::MAX,
      b: u8::MIN,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn blue() -> Self {
    Self {
      r: u8::MIN,
      g: u8::MIN,
      b: u8::MAX,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn yellow() -> Self {
    Self::red().cadd(Self::green())
  }

  #[inline]
  pub(crate) const fn violet() -> Self {
    Self::red().cadd(Self::blue())
  }

  #[inline]
  pub(crate) const fn cyan() -> Self {
    Self::green().cadd(Self::blue())
  }

  #[inline]
  pub(crate) const fn orange() -> Self {
    Self {
      r: u8::MAX,
      g: u8::MAX / 2,
      b: u8::MIN,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn gray() -> Self {
    Self {
      r: u8::MAX / 2,
      g: u8::MAX / 2,
      b: u8::MAX / 2,
      a: u8::MAX,
    }
  }
}

impl Add<Color> for Color {
  type Output = Color;

  fn add(self, other: Color) -> Self::Output {
    self.cadd(other)
  }
}

impl Sub<Color> for Color {
  type Output = Color;

  fn sub(self, other: Color) -> Self::Output {
    self.csub(other)
  }
}


#[derive(Clone, Debug)]
enum TextureState {
  /// The texture has been bound and will be used when rendering.
  Bound { bound: Rc<Texture> },
  /// The texture is active but has not yet been bound. The most
  /// convenient way for ensuring that it is in fact bound once that is
  /// required is via the [`ensure_bound`][Self::ensure_bound] method.
  Unbound {
    unbound: Rc<Texture>,
    still_bound: Option<Rc<Texture>>,
  },
}

impl TextureState {
  /// Mark the provided texture as active, but don't bind it yet.
  fn activate(&mut self, texture: Rc<Texture>) -> Rc<Texture> {
    match self {
      Self::Bound { bound } if Rc::ptr_eq(&texture, bound) => texture,
      Self::Bound { bound } => {
        let state = Self::Unbound {
          unbound: texture,
          still_bound: Some(Rc::clone(bound)),
        };
        replace(self, state).into_texture()
      },
      Self::Unbound { unbound, .. } => replace(unbound, texture),
    }
  }

  /// Make sure that the "active" texture is bound.
  fn ensure_bound(&mut self) {
    match self {
      Self::Bound { .. } => (),
      Self::Unbound {
        unbound,
        still_bound,
      } => {
        if still_bound
          .as_ref()
          .map(|still_bound| !Rc::ptr_eq(unbound, still_bound))
          .unwrap_or(true)
        {
          let () = unbound.bind();
        }

        // The clone is reasonably cheap, but also entirely unnecessary at
        // a conceptual level. We just want to flip the enum variant from
        // `Unbound` to `Bound`. Thanks Rust...
        let bound = Rc::clone(unbound);
        let _prev = replace(self, Self::Bound { bound });
      },
    }
  }

  /// Retrieve the "active" texture.
  fn texture(&self) -> &Rc<Texture> {
    match self {
      Self::Bound { bound: texture }
      | Self::Unbound {
        unbound: texture, ..
      } => texture,
    }
  }

  /// Destruct the object into the "active" texture.
  fn into_texture(self) -> Rc<Texture> {
    match self {
      Self::Bound { bound: texture }
      | Self::Unbound {
        unbound: texture, ..
      } => texture,
    }
  }
}


/// A type directly usable to render graphics primitives.
#[derive(Debug)]
pub struct ActiveRenderer<'renderer> {
  /// The `Renderer` this object belongs to.
  renderer: &'renderer Renderer,
  /// The origin relative to which rendering happens.
  origin: Cell<Point<i16>>,
  /// The currently set color.
  color: Cell<Color>,
  /// The currently set texture.
  texture: RefCell<TextureState>,
  /// The vertex buffer we use.
  vertices: RefCell<Vec<Vertex>>,
  /// The type of primitive currently active for rendering.
  primitive: Cell<sys::Primitive>,
}

impl<'renderer> ActiveRenderer<'renderer> {
  fn new(renderer: &'renderer Renderer) -> Self {
    Self {
      renderer,
      origin: Cell::new(Point::default()),
      color: Cell::new(Color::black()),
      texture: RefCell::new(TextureState::Unbound {
        unbound: Rc::clone(&renderer.empty_texture),
        still_bound: None,
      }),
      vertices: RefCell::new(Vec::with_capacity(VERTEX_BUFFER_CAPACITY)),
      primitive: Cell::new(sys::Primitive::Triangles),
    }
  }

  /// Set the origin relative to which rendering happens.
  #[inline]
  pub(crate) fn set_origin(&self, origin: Point<i16>) -> Guard<'_, impl FnOnce() + '_> {
    let new_origin = self.origin.get() + origin;
    let prev_origin = self.origin.replace(new_origin);
    Guard::new(move || self.origin.set(prev_origin))
  }

  /// Set the color with which subsequent vertices are to be rendered.
  #[inline]
  pub(crate) fn set_color(&self, color: Color) -> Guard<'_, impl FnOnce() + '_> {
    let prev_color = self.color.replace(color);
    Guard::new(move || self.color.set(prev_color))
  }

  #[inline]
  pub(crate) fn set_texture(&self, texture: &Rc<Texture>) -> Guard<'_, impl FnOnce() + '_> {
    fn set(renderer: &ActiveRenderer, texture: Rc<Texture>) -> Rc<Texture> {
      let mut state = renderer.texture.borrow_mut();
      let state = state.deref_mut();

      if !Rc::ptr_eq(&texture, state.texture()) {
        let () = renderer.flush_vertex_buffer(state);
      }

      state.activate(texture)
    }

    let texture = Rc::clone(texture);
    let prev_texture = set(self, texture);

    Guard::new(move || {
      let _prev = set(self, prev_texture);
    })
  }

  #[inline]
  pub(crate) fn set_no_texture(&self) -> Guard<'_, impl FnOnce() + '_> {
    self.set_texture(&self.renderer.empty_texture)
  }

  /// Clear the screen using the given color.
  pub(crate) fn clear_screen(&self, color: (f32, f32, f32)) {
    let (r, g, b) = color;
    let () = self.renderer.context.set_clear_color(r, g, b, 1.0);
    let () = self.renderer.context.clear(sys::ClearMask::ColorBuffer);
  }

  /// Render a line.
  pub(crate) fn render_line(&self, mut p1: Point<i16>, mut p2: Point<i16>) {
    const VERTEX_COUNT_LINE: usize = 2;

    let origin = self.origin.get();
    p1 += origin;
    p2 += origin;

    let () = self.set_primitive(sys::Primitive::Lines, VERTEX_COUNT_LINE);
    let Color { r, g, b, a } = self.color.get();

    let mut vertex = Vertex {
      u: 0,
      v: 0,
      r,
      g,
      b,
      a,
      x: p1.x.into(),
      y: p1.y.into(),
    };

    let mut buffer = self.vertices.borrow_mut();
    let vertices = buffer.spare_capacity_mut();
    vertices[0].write(vertex);

    // second point
    vertex.x = p2.x.into();
    vertex.y = p2.y.into();
    vertices[1].write(vertex);

    let len = buffer.len();
    let () = unsafe { buffer.set_len(len + VERTEX_COUNT_LINE) };
  }

  /// Render a rectangle.
  pub(crate) fn render_rect(&self, rect: Rect<i16>) {
    let () = self.render_rect_f32(rect.into_other());
  }

  /// Render a rectangle.
  pub(crate) fn render_rect_f32(&self, rect: Rect<f32>) {
    // Texture coordinates for the quad. We always map the complete
    // texture on it.
    let coords = Rect::new(0, 0, 1, 1);
    let () = self.render_rect_with_tex_coords(rect, coords);
  }

  /// Render a rectangle.
  pub(crate) fn render_rect_with_tex_coords(&self, mut rect: Rect<f32>, coords: Rect<i16>) {
    const VERTEX_COUNT_QUAD: usize = 6;

    let origin = self.origin.get();
    rect += origin.into_other();

    let () = self.set_primitive(sys::Primitive::Triangles, VERTEX_COUNT_QUAD);
    let Color { r, g, b, a } = self.color.get();

    let mut vertex = Vertex {
      u: coords.x,
      v: coords.y,
      r,
      g,
      b,
      a,
      x: rect.x,
      y: rect.y,
    };

    let mut buffer = self.vertices.borrow_mut();
    let vertices = buffer.spare_capacity_mut();

    // lower left
    vertices[0].write(vertex);
    vertices[5].write(vertex);

    // lower right
    vertex.u += coords.w;
    vertex.x += rect.w;
    vertices[1].write(vertex);

    // upper right
    vertex.v += coords.h;
    vertex.y += rect.h;
    vertices[2].write(vertex);

    // upper right
    vertices[3].write(vertex);

    // upper left
    vertex.u = coords.x;
    vertex.x = rect.x;
    vertices[4].write(vertex);

    let len = buffer.len();
    let () = unsafe { buffer.set_len(len + VERTEX_COUNT_QUAD) };
  }

  /// Set the type of primitive that we currently render and ensure that
  /// there is space for at least `vertex_cnt` vertices in our vertex
  /// buffer.
  fn set_primitive(&self, primitive: sys::Primitive, vertex_cnt: usize) {
    if primitive != self.primitive.get()
      || self.vertices.borrow_mut().spare_capacity_mut().len() < vertex_cnt
    {
      let () = self.flush_vertex_buffer(self.texture.borrow_mut().deref_mut());
      let () = self.primitive.set(primitive);
    }
  }

  /// Send the cached data to the graphics device for rendering.
  fn flush_vertex_buffer(&self, texture: &mut TextureState) {
    let mut buffer = self.vertices.borrow_mut();
    let size = buffer.len() as _;
    if size > 0 {
      let () = texture.ensure_bound();
      let () = self.renderer.vertices_vbo.update(&buffer, 0);
      let () = self.renderer.vertices_vao.bind();
      let () = self
        .renderer
        .context
        .draw_arrays(self.primitive.get(), size);

      debug_assert!(const { !needs_drop::<Vertex>() });
      // SAFETY: We are strictly decreasing size and our vertices are
      //         plain old data. No need to drop them properly.
      unsafe { buffer.set_len(0) };
    }
  }
}

impl Drop for ActiveRenderer<'_> {
  fn drop(&mut self) {
    let () = self.flush_vertex_buffer(self.texture.borrow_mut().deref_mut());
    let () = self.renderer.on_post_render();
  }
}


/// A type enabling the rendering of graphics.
#[derive(Debug)]
pub struct Renderer {
  /// The physical width of the window to which this renderer belongs.
  phys_w: u32,
  /// The physical height of the window to which this renderer belongs.
  phys_h: u32,
  /// The logical width of the view maintained by this renderer.
  logic_w: f32,
  /// The logical height of the view maintained by this renderer.
  logic_h: f32,
  /// The OpenGL context.
  context: sys::Context,
  /// The program.
  _program: Program,
  /// The model-view matrix stack.
  modelview: RefCell<MatrixStack<Mat4f, 2, Box<dyn Fn(&Mat4f)>>>,
  /// The projection matrix stack.
  projection: RefCell<MatrixStack<Mat4f, 2, Box<dyn Fn(&Mat4f)>>>,
  /// An "empty" texture.
  empty_texture: Rc<Texture>,
  /// Vertices for rendering the scene.
  vertices_vbo: VertexBuffer<Vertex>,
  /// The vertex array object capturing the VBO state.
  vertices_vao: VertexArray,
}

impl Renderer {
  /// Create a new [`Renderer`] object assuming the provide "physical"
  /// and logical view dimensions.
  pub fn new(
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) -> Result<Self> {
    let context = sys::Context::default();

    // The name of the model-view matrix uniform in the vertex shader.
    let modelview_uniform = "modelview";
    // The name of the projection matrix uniform in the vertex shader.
    let projection_uniform = "projection";
    // The name of the texture uniform in the fragment shader.
    let texture_unit_uniform = "texture_unit";
    // The name of the color input attribute in the vertex shader.
    let color_attrib = "color";
    // The name of the position input attribute in the vertex shader.
    let position_attrib = "position";
    // The name of the texture coordinate input attribute in the vertex shader.
    let texture_coord_attrib = "texture_coord";
    // The name of the color attribute transferred between vertex and
    // fragment shader.
    let color_in_out = "color_in_out";
    // The name of the texture coordinate attribute transferred between
    // vertex and fragment shader.
    let texture_coord_in_out = "texture_coord_in_out";

    let vertex_shader_file = format!(
      r#"#version {glsl_version} core

      uniform mat4 {modelview_uniform};
      uniform mat4 {projection_uniform};

      in vec3 {position_attrib};
      in vec4 {color_attrib};
      in vec2 {texture_coord_attrib};

      out vec4 {color_in_out};
      out vec2 {texture_coord_in_out};

      void main() {{
        gl_Position = {projection_uniform} * {modelview_uniform} * vec4({position_attrib}, 1.0);

        {color_in_out} = {color_attrib};
        {texture_coord_in_out} = {texture_coord_attrib};
      }}
      "#,
      glsl_version = Shader::glsl_version(),
    );

    let fragment_shader_file = format!(
      r#"#version {glsl_version} core

      uniform sampler2D {texture_unit_uniform};

      in vec4 {color_in_out};
      in vec2 {texture_coord_in_out};

      out vec4 fragment_color;

      void main() {{
        fragment_color = texture({texture_unit_uniform}, {texture_coord_in_out}) * {color_in_out};
      }}
      "#,
      glsl_version = Shader::glsl_version(),
    );

    let vertex_shader = Shader::new(sys::ShaderType::Vertex, &vertex_shader_file, &context)
      .context("failed to create vertex shader")?;
    let fragment_shader = Shader::new(sys::ShaderType::Fragment, &fragment_shader_file, &context)
      .context("failed to create fragment shader")?;
    let program = Program::new(&[vertex_shader, fragment_shader], &context)?;
    let texture_unit_loc = program.query_uniform_location(texture_unit_uniform)?;
    let modelview_loc = program.query_uniform_location(modelview_uniform)?;
    let projection_loc = program.query_uniform_location(projection_uniform)?;
    let color_idx = program.query_attrib_location(color_attrib)?;
    let position_idx = program.query_attrib_location(position_attrib)?;
    let texture_coord_idx = program.query_attrib_location(texture_coord_attrib)?;

    // Bind the program so that we can set uniforms below.
    let () = program.bind();

    // All our texturing uses a single unit. Activate it.
    let unit = 0;
    let () = context.set_active_texture_unit(unit);
    let () = context.set_uniform_1i(&texture_unit_loc, unit as _);

    let context_clone1 = context.clone();
    let context_clone2 = context.clone();

    let (logic_w, logic_h) = Self::calculate_view(phys_w, phys_h, logic_w, logic_h);

    let attrib_indices = [
      (texture_coord_idx, AttribType::Texture),
      (color_idx, AttribType::Color),
      (position_idx, AttribType::Position),
    ];
    let vertices_vbo = VertexBuffer::from_vertices(
      &[Vertex::default(); VERTEX_BUFFER_CAPACITY],
      sys::VertexBufferUsage::DynamicDraw,
      &context,
    )?;
    let vertices_vao = VertexArray::new(&vertices_vbo, &attrib_indices, &context)?;
    let empty_texture = Rc::new(empty_texture(&context)?);

    let slf = Self {
      phys_w: phys_w.get(),
      phys_h: phys_h.get(),
      logic_w,
      logic_h,
      context,
      _program: program,
      modelview: RefCell::new(MatrixStack::new(Box::new(move |matrix| {
        context_clone1.set_uniform_matrix(&modelview_loc, matrix.as_array())
      }))),
      projection: RefCell::new(MatrixStack::new(Box::new(move |matrix| {
        context_clone2.set_uniform_matrix(&projection_loc, matrix.as_array())
      }))),
      empty_texture,
      vertices_vbo,
      vertices_vao,
    };
    Ok(slf)
  }

  fn calculate_view(
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) -> (f32, f32) {
    let phys_w = phys_w.get() as f32;
    let phys_h = phys_h.get() as f32;
    let logic_w = logic_w.get() as f32;
    let logic_h = logic_h.get() as f32;

    let phys_ratio = phys_w / phys_h;
    let logic_ratio = logic_w / logic_h;

    let mut width = logic_w;
    let mut height = logic_h;

    // Our goal is to make the two ratios equal, that means:
    //
    // phys_w   logic_w + x
    // ------ = -----------
    // phys_h   logic_h + y
    //
    // where `x` is zero if `logic_ratio` > `phys_ratio`, otherwise `y`
    // is zero. Resolve it to `x` or `y` to get the equation from above.
    if logic_ratio > phys_ratio {
      height += logic_w * phys_h / phys_w - logic_h;
    } else {
      width += logic_h * phys_w / phys_h - logic_w;
    }
    (width, height)
  }

  /// Update the view after the containing window or contained logical
  /// dimensions have changed.
  pub fn update_view(
    &mut self,
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) {
    let (logic_w, logic_h) = Self::calculate_view(phys_w, phys_h, logic_w, logic_h);

    self.phys_w = phys_w.get();
    self.phys_h = phys_h.get();
    self.logic_w = logic_w;
    self.logic_h = logic_h;
  }

  fn push_states(&self) {
    let () = self
      .context
      .set_viewport(0, 0, self.phys_w as _, self.phys_h as _);
  }

  fn pop_states(&self) {}

  fn push_matrizes(&self) {
    // We create an orthogonal projection matrix with bounds
    // sufficient to contain the logical view.
    let () = self.projection.borrow_mut().push(|p| {
      // Our renderer will render everything with z-coordinate of 0.0f,
      // this must lie inside the range [zNear, zFar] (last two
      // parameters).
      *p = Mat4f::orthographic(0.0, self.logic_w, 0.0, self.logic_h, -0.5, 0.5);
    });
    let () = self.modelview.borrow_mut().push(|m| {
      *m = Mat4f::identity();
    });
  }

  fn pop_matrizes(&self) {
    let () = self.modelview.borrow_mut().pop();
    let () = self.projection.borrow_mut().pop();
  }

  /// Activate the renderer with the given [`Context`] in preparation
  /// for rendering to take place.
  // This method requires an exclusive `Context` reference do ensure
  // that while a renderer is active the context can't swap buffers, for
  // example.
  pub fn on_pre_render<'ctx>(&'ctx self, context: &'ctx mut Context) -> ActiveRenderer<'ctx> {
    let _ = context;
    let () = self.push_states();
    let () = self.push_matrizes();

    ActiveRenderer::new(self)
  }

  fn on_post_render(&self) {
    let () = self.pop_matrizes();
    let () = self.pop_states();
  }
}
