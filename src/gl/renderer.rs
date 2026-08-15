// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::Cell;
use std::cell::RefCell;
use std::mem::needs_drop;
use std::mem::offset_of;
use std::mem::replace;
use std::ops::DerefMut as _;
use std::rc::Rc;

use anyhow::Result;

use xgl::VertexArray;
use xgl::VertexBuffer;
use xgl::sys;
use xgl::sys::Gl as _;
use xgl::vertex::Attrib;
use xgl::vertex::AttribType;
use xgl::vertex::Attribs;

use crate::Point;
use crate::Rect;
use crate::guard::Guard;

use super::Color;
use super::Mat4f;
use super::ObjectRenderState;
use super::Texture;
use super::empty_texture;


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
  fn ensure_bound(&mut self, object: &mut ObjectRenderState) {
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
          let () = object.set_texture(unbound);
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
pub(crate) struct ActiveRenderer<'state> {
  /// The GL render state.
  object: RefCell<&'state mut ObjectRenderState>,
  /// The `Renderer` this object belongs to.
  renderer: &'state Renderer,
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

impl<'state> ActiveRenderer<'state> {
  fn new(object: &'state mut ObjectRenderState, renderer: &'state Renderer) -> Self {
    Self {
      object: RefCell::new(object),
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
      let () = texture.ensure_bound(*self.object.borrow_mut());
      let () = self.renderer.vertices_vbo.update(&buffer, 0);
      let () = self.renderer.vertices_vao.bind();
      let () = self.object.borrow().draw_arrays(self.primitive.get(), size);

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
  }
}


/// A type enabling the rendering of graphics.
#[derive(Debug)]
pub(crate) struct Renderer {
  /// The modelview matrix we use.
  modelview: Mat4f,
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
  pub fn new(state: &mut ObjectRenderState) -> Result<Self> {
    let vertices_vbo = VertexBuffer::from_vertices(
      &[Vertex::default(); VERTEX_BUFFER_CAPACITY],
      sys::VertexBufferUsage::DynamicDraw,
      state,
    )?;
    let vertices_vao = VertexArray::new(&vertices_vbo, state.attrib_indices(), state)?;
    let empty_texture = Rc::new(empty_texture(state)?);

    let () = Self::set_global_gl_state(state);

    let slf = Self {
      modelview: Mat4f::identity(),
      empty_texture,
      vertices_vbo,
      vertices_vao,
    };
    Ok(slf)
  }

  /// Set global GL state that we treat as invariant between frames.
  ///
  /// If an object changes this state temporarily, said change has to be
  /// reverted when no longer needed.
  fn set_global_gl_state(context: &sys::Context) {
    let () = context.disable(sys::Capability::ScissorTest);
    let () = context.disable(sys::Capability::DepthTest);

    let () = context.enable(sys::Capability::CullFace);
    let () = context.set_front_face(sys::FrontFace::CounterClockWise);
    let () = context.set_cull_face(sys::CullFace::Back);
  }

  /// Activate the renderer with the given [`sys::Context`] in
  /// preparation for rendering to take place.
  pub fn on_pre_render<'state>(
    &'state self,
    state: &'state mut ObjectRenderState,
  ) -> ActiveRenderer<'state> {
    let () = state.set_modelview(&self.modelview);

    ActiveRenderer::new(state, self)
  }
}
