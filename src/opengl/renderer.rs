// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::Cell;
use std::cell::RefCell;
use std::mem::needs_drop;
use std::mem::replace;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::ops::Add;
use std::ops::DerefMut as _;

use crate::guard::Guard;
use crate::Point;
use crate::Rect;

use super::gl;
use super::Context;
use super::Texture;


/// The capacity of our vertex buffer.
// TODO: We should consider sizing it more dynamically and just making
//       this an upper limit instead.
const VERTEX_BUFFER_CAPACITY: usize = 1024;


#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
#[repr(packed)]
struct Vertex {
  // texture coordinates
  u: gl::GLfloat,
  v: gl::GLfloat,

  // color
  r: gl::GLubyte,
  g: gl::GLubyte,
  b: gl::GLubyte,
  a: gl::GLubyte,

  // position
  x: gl::GLfloat,
  y: gl::GLfloat,
  z: gl::GLfloat,
}


#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub(crate) struct Color {
  r: gl::GLubyte,
  g: gl::GLubyte,
  b: gl::GLubyte,
  a: gl::GLubyte,
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

  #[inline]
  pub(crate) const fn black() -> Self {
    Self {
      r: gl::GLubyte::MIN,
      g: gl::GLubyte::MIN,
      b: gl::GLubyte::MIN,
      a: gl::GLubyte::MAX,
    }
  }

  #[inline]
  pub(crate) const fn white() -> Self {
    Self {
      r: gl::GLubyte::MAX,
      g: gl::GLubyte::MAX,
      b: gl::GLubyte::MAX,
      a: gl::GLubyte::MAX,
    }
  }

  #[inline]
  pub(crate) const fn red() -> Self {
    Self {
      r: gl::GLubyte::MAX,
      g: gl::GLubyte::MIN,
      b: gl::GLubyte::MIN,
      a: gl::GLubyte::MAX,
    }
  }

  #[inline]
  pub(crate) const fn green() -> Self {
    Self {
      r: gl::GLubyte::MIN,
      g: gl::GLubyte::MAX,
      b: gl::GLubyte::MIN,
      a: gl::GLubyte::MAX,
    }
  }

  #[inline]
  pub(crate) const fn blue() -> Self {
    Self {
      r: gl::GLubyte::MIN,
      g: gl::GLubyte::MIN,
      b: gl::GLubyte::MAX,
      a: gl::GLubyte::MAX,
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
      r: gl::GLubyte::MAX,
      g: gl::GLubyte::MAX / 2,
      b: gl::GLubyte::MIN,
      a: gl::GLubyte::MAX,
    }
  }

  #[inline]
  pub(crate) const fn gray() -> Self {
    Self {
      r: gl::GLubyte::MAX / 2,
      g: gl::GLubyte::MAX / 2,
      b: gl::GLubyte::MAX / 2,
      a: gl::GLubyte::MAX,
    }
  }
}

impl Add<Color> for Color {
  type Output = Color;

  fn add(self, other: Color) -> Self::Output {
    self.cadd(other)
  }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
enum Primitive {
  Line = gl::LINES,
  Quad = gl::QUADS,
}

impl Primitive {
  fn as_glenum(&self) -> gl::GLenum {
    *self as _
  }
}


#[derive(Clone, Debug)]
enum TextureState {
  /// The texture has been bound and will be used when rendering.
  Bound { bound: Texture },
  /// The texture is active but has not yet been bound. The most
  /// convenient way for ensuring that it is in fact bound once that is
  /// required is via the [`ensure_bound`][Self::ensure_bound] method.
  Unbound {
    unbound: Texture,
    still_bound: Texture,
  },
}

impl TextureState {
  /// Mark the provided texture as active, but don't bind it yet.
  fn activate(&mut self, texture: Texture) -> Texture {
    match self {
      Self::Bound { bound } if texture == *bound => texture,
      Self::Bound { bound } => {
        let state = Self::Unbound {
          unbound: texture,
          still_bound: bound.clone(),
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
        if unbound != still_bound {
          let () = unbound.bind();
        }

        // The clone is reasonably cheap, but also entirely unnecessary at
        // a conceptual level. We just want to flip the enum variant from
        // `Unbound` to `Bound`. Thanks Rust...
        let bound = unbound.clone();
        let _prev = replace(self, Self::Bound { bound });
      },
    }
  }

  /// Retrieve the "active" texture.
  fn texture(&self) -> &Texture {
    match self {
      Self::Bound { bound: texture }
      | Self::Unbound {
        unbound: texture, ..
      } => texture,
    }
  }

  /// Destruct the object into the "active" texture.
  fn into_texture(self) -> Texture {
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
  /// An invalid texture.
  invalid_texture: Texture,
  /// The origin relative to which rendering happens.
  origin: Cell<Point<i16>>,
  /// The currently set color.
  color: Cell<Color>,
  /// The currently set texture.
  texture: RefCell<TextureState>,
  /// The vertex buffer we use.
  vertices: RefCell<Vec<Vertex>>,
  /// The type of primitive currently active for rendering.
  primitive: Cell<Primitive>,
}

impl<'renderer> ActiveRenderer<'renderer> {
  fn new(renderer: &'renderer Renderer) -> Self {
    let invalid_texture = Texture::invalid();
    Self {
      renderer,
      invalid_texture: invalid_texture.clone(),
      origin: Cell::new(Point::default()),
      color: Cell::new(Color::black()),
      // We know that no texture is active, because we are called on the
      // `Renderer::on_pre_render` path and it just cleared a bunch of
      // state. So it's fine for us to claim that an "invalid" texture
      // is bound already.
      texture: RefCell::new(TextureState::Bound {
        bound: invalid_texture,
      }),
      vertices: RefCell::new(Vec::with_capacity(VERTEX_BUFFER_CAPACITY)),
      primitive: Cell::new(Primitive::Quad),
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
  pub(crate) fn set_texture(&self, texture: &Texture) -> Guard<'_, impl FnOnce() + '_> {
    fn set(renderer: &ActiveRenderer, texture: Texture) -> Texture {
      let mut state = renderer.texture.borrow_mut();
      let state = state.deref_mut();

      if texture != *state.texture() {
        let () = renderer.flush_vertex_buffer(state);
      }

      state.activate(texture)
    }

    let texture = texture.clone();
    let prev_texture = set(self, texture);

    Guard::new(move || {
      let _prev = set(self, prev_texture);
    })
  }

  #[inline]
  pub(crate) fn set_no_texture(&self) -> Guard<'_, impl FnOnce() + '_> {
    self.set_texture(&self.invalid_texture)
  }

  /// Render a line.
  pub(crate) fn render_line(&self, mut p1: Point<i16>, mut p2: Point<i16>) {
    const VERTEX_COUNT_LINE: usize = 2;

    let origin = self.origin.get();
    p1 += origin;
    p2 += origin;

    let () = self.set_primitive(Primitive::Line, VERTEX_COUNT_LINE);
    let color = self.color.get();

    let mut vertex = Vertex {
      u: 0.0,
      v: 0.0,
      r: color.r,
      g: color.g,
      b: color.b,
      a: color.a,
      x: p1.x.into(),
      y: p1.y.into(),
      z: 0.0,
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
    let coords = Rect::new(0.0, 0.0, 1.0, 1.0);
    let () = self.render_rect_with_tex_coords_f32(rect, coords);
  }

  /// Render a rectangle.
  pub(crate) fn render_rect_with_tex_coords(&self, rect: Rect<i16>, coords: Rect<f32>) {
    self.render_rect_with_tex_coords_f32(rect.into_other(), coords)
  }

  /// Render a rectangle.
  pub(crate) fn render_rect_with_tex_coords_f32(&self, mut rect: Rect<f32>, coords: Rect<f32>) {
    const VERTEX_COUNT_QUAD: usize = 4;

    let origin = self.origin.get();
    rect += origin.into_other();

    let () = self.set_primitive(Primitive::Quad, VERTEX_COUNT_QUAD);
    let color = self.color.get();

    let mut vertex = Vertex {
      u: coords.x,
      v: coords.y,
      r: color.r,
      g: color.g,
      b: color.b,
      a: color.a,
      x: rect.x,
      y: rect.y,
      z: 0.0,
    };

    let mut buffer = self.vertices.borrow_mut();
    let vertices = buffer.spare_capacity_mut();
    vertices[0].write(vertex);

    // lower right
    vertex.u += coords.w;
    vertex.x += rect.w;
    vertices[1].write(vertex);

    // upper right
    vertex.v += coords.h;
    vertex.y += rect.h;
    vertices[2].write(vertex);

    // upper left
    vertex.u = coords.x;
    vertex.x = rect.x;
    vertices[3].write(vertex);

    let len = buffer.len();
    let () = unsafe { buffer.set_len(len + VERTEX_COUNT_QUAD) };
  }

  /// Set the type of primitive that we currently render and ensure that
  /// there is space for at least `vertex_cnt` vertices in our vertex
  /// buffer.
  fn set_primitive(&self, primitive: Primitive, vertex_cnt: usize) {
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

      unsafe {
        gl::InterleavedArrays(gl::T2F_C4UB_V3F, 0, buffer.as_ptr().cast());
        gl::DrawArrays(self.primitive.get().as_glenum(), 0, size);

        debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
      }

      debug_assert!(!needs_drop::<Vertex>());
      // SAFETY: We are strictly decreasing size and our vertices are
      //         plain old data. No need to drop them properly.
      unsafe { buffer.set_len(0) };
    }
  }

  /// Retrieve the physical width of the rendering area, in pixels.
  #[inline]
  pub(crate) fn phys_width(&self) -> u32 {
    self.renderer.phys_w as _
  }

  /// Retrieve the physical height of the rendering area, in pixels.
  #[inline]
  pub(crate) fn phys_height(&self) -> u32 {
    self.renderer.phys_h as _
  }

  /// Retrieve the logical width of the rendering area, in game units,
  /// but already expressed as a floating point value.
  #[inline]
  pub(crate) fn logic_width(&self) -> f32 {
    self.renderer.logic_w as _
  }

  /// Retrieve the logical height of the rendering area, in game units,
  /// but already expressed as a floating point value.
  #[inline]
  pub(crate) fn logic_height(&self) -> f32 {
    self.renderer.logic_h as _
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
  phys_w: gl::GLsizei,
  /// The physical height of the window to which this renderer belongs.
  phys_h: gl::GLsizei,
  /// The logical width of the view maintained by this renderer.
  logic_w: gl::GLfloat,
  /// The logical height of the view maintained by this renderer.
  logic_h: gl::GLfloat,
}

impl Renderer {
  /// Create a new [`Renderer`] object assuming the provide "physical"
  /// and logical view dimensions.
  pub fn new(
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) -> Self {
    let (logic_w, logic_h) = Self::calculate_view(phys_w, phys_h, logic_w, logic_h);

    Self {
      phys_w: gl::GLsizei::try_from(phys_w.get()).unwrap_or(gl::GLsizei::MAX),
      phys_h: gl::GLsizei::try_from(phys_h.get()).unwrap_or(gl::GLsizei::MAX),
      logic_w,
      logic_h,
    }
  }

  fn calculate_view(
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) -> (gl::GLfloat, gl::GLfloat) {
    let phys_w = phys_w.get() as gl::GLfloat;
    let phys_h = phys_h.get() as gl::GLfloat;
    let logic_w = logic_w.get() as gl::GLfloat;
    let logic_h = logic_h.get() as gl::GLfloat;

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

    self.phys_w = gl::GLsizei::try_from(phys_w.get()).unwrap_or(gl::GLsizei::MAX);
    self.phys_h = gl::GLsizei::try_from(phys_h.get()).unwrap_or(gl::GLsizei::MAX);
    self.logic_w = logic_w;
    self.logic_h = logic_h;
  }

  fn push_states(&self) {
    unsafe {
      gl::PushAttrib(
        gl::CURRENT_BIT
          | gl::COLOR_BUFFER_BIT
          | gl::DEPTH_BUFFER_BIT
          | gl::ENABLE_BIT
          | gl::FOG_BIT
          | gl::LIGHTING_BIT
          | gl::LINE_BIT
          | gl::POINT_BIT
          | gl::SCISSOR_BIT
          | gl::STENCIL_BUFFER_BIT
          | gl::TEXTURE_BIT
          | gl::TRANSFORM_BIT
          | gl::VIEWPORT_BIT,
      );

      gl::Disable(gl::FOG);
      gl::Disable(gl::LIGHTING);
      gl::Disable(gl::COLOR_MATERIAL);
      gl::Disable(gl::DEPTH_TEST);
      gl::Disable(gl::SCISSOR_TEST);
      gl::Disable(gl::CULL_FACE);

      gl::Enable(gl::TEXTURE_2D);

      gl::PointSize(1.0);
      gl::LineWidth(1.0);

      gl::Viewport(0, 0, self.phys_w, self.phys_h);

      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
  }

  fn pop_states(&self) {
    unsafe {
      gl::PopAttrib();

      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
  }

  fn push_matrizes(&self) {
    unsafe {
      // We create an orthogonal projection matrix with bounds
      // sufficient to contain the logical view.
      gl::MatrixMode(gl::PROJECTION);
      gl::PushMatrix();
      gl::LoadIdentity();
      // Our renderer will render everything with z-coordinate of 0.0f,
      // this must lie inside the range [zNear, zFar] (last two
      // parameters).
      gl::Ortho(
        0.0,
        self.logic_w.into(),
        0.0,
        self.logic_h.into(),
        -0.5,
        0.5,
      );

      gl::MatrixMode(gl::TEXTURE);
      gl::PushMatrix();
      gl::LoadIdentity();

      gl::MatrixMode(gl::MODELVIEW);
      gl::PushMatrix();
      gl::LoadIdentity();

      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
  }

  fn pop_matrizes(&self) {
    unsafe {
      gl::MatrixMode(gl::MODELVIEW);
      gl::PopMatrix();

      gl::MatrixMode(gl::TEXTURE);
      gl::PopMatrix();

      gl::MatrixMode(gl::PROJECTION);
      gl::PopMatrix();

      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
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

    unsafe {
      // Approximation of 0xeeeeee.
      // TODO: Make color configurable.
      gl::ClearColor(0.93, 0.93, 0.93, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);

      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
    ActiveRenderer::new(self)
  }

  fn on_post_render(&self) {
    let () = self.pop_matrizes();
    let () = self.pop_states();
  }
}
