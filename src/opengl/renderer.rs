// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::Cell;
use std::cell::RefCell;
use std::mem::needs_drop;
use std::num::NonZeroU16;
use std::num::NonZeroU32;

use anyhow::Context as _;
use anyhow::Result;

use glutin::context::PossiblyCurrentContext;
use glutin::surface::GlSurface;
use glutin::surface::Surface;
use glutin::surface::WindowSurface;

use crate::Point;

use super::gl;


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
  #[inline]
  pub(crate) const fn black() -> Self {
    Self {
      r: 0,
      g: 0,
      b: 0,
      a: gl::GLubyte::MAX,
    }
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


pub(crate) struct Renderer {
  /// The OpenGL surface that is used for rendering.
  surface: Surface<WindowSurface>,
  /// The OpenGL context used for double buffering.
  context: PossiblyCurrentContext,
  /// The physical width of the window to which this renderer belongs.
  phys_w: gl::GLsizei,
  /// The physical height of the window to which this renderer belongs.
  phys_h: gl::GLsizei,
  /// The logical width of the view maintained by this renderer.
  logic_w: gl::GLfloat,
  /// The logical height of the view maintained by this renderer.
  logic_h: gl::GLfloat,
  /// The currently set color.
  color: Cell<Color>,
  /// The vertex buffer we use.
  vertices: RefCell<Vec<Vertex>>,
  /// The type of primitive currently active for rendering.
  primitive: Cell<Primitive>,
}

impl Renderer {
  pub(super) fn new(
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) -> Self {
    let (logic_w, logic_h) = Self::calculate_view(phys_w, phys_h, logic_w, logic_h);

    Self {
      surface,
      context,
      phys_w: gl::GLsizei::try_from(phys_w.get()).unwrap_or(gl::GLsizei::MAX),
      phys_h: gl::GLsizei::try_from(phys_w.get()).unwrap_or(gl::GLsizei::MAX),
      logic_w,
      logic_h,
      color: Cell::new(Color::black()),
      vertices: RefCell::new(Vec::with_capacity(VERTEX_BUFFER_CAPACITY)),
      primitive: Cell::new(Primitive::Quad),
    }
  }

  /// Set the color with which subsequent vertices are to be rendered.
  #[inline]
  pub(crate) fn set_color(&self, color: Color) {
    let _prev = self.color.replace(color);
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

  pub(crate) fn update_view(
    &mut self,
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) {
    let () = self.surface.resize(&self.context, phys_w, phys_h);
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

  pub(crate) fn on_pre_render(&self) -> Result<()> {
    let () = self.push_states();
    let () = self.push_matrizes();

    unsafe {
      // Approximation of 0xeeeeee.
      // TODO: Make color configurable.
      gl::ClearColor(0.93, 0.93, 0.93, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT);

      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
    Ok(())
  }

  pub(crate) fn on_post_render(&self) -> Result<()> {
    let () = self.flush_vertex_buffer();
    let () = self.pop_matrizes();
    let () = self.pop_states();

    let () = self
      .surface
      .swap_buffers(&self.context)
      .context("failed to swap OpenGL buffers")?;
    Ok(())
  }

  /// Render a line.
  pub(crate) fn render_line(&self, p1: Point<u16>, p2: Point<u16>) {
    const VERTEX_COUNT_LINE: usize = 2;

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

  /// Set the type of primitive that we currently render and ensure that
  /// there is space for at least `vertex_cnt` vertices in our vertex
  /// buffer.
  fn set_primitive(&self, primitive: Primitive, vertex_cnt: usize) {
    if primitive != self.primitive.get()
      || self.vertices.borrow_mut().spare_capacity_mut().len() < vertex_cnt
    {
      let () = self.flush_vertex_buffer();
      let () = self.primitive.set(primitive);
    }
  }

  /// Send the cached data to the graphics device for rendering.
  fn flush_vertex_buffer(&self) {
    let mut buffer = self.vertices.borrow_mut();
    let size = buffer.len() as _;
    if size > 0 {
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
}
