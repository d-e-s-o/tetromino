// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::num::NonZeroU16;
use std::num::NonZeroU32;

use anyhow::Context as _;
use anyhow::Result;

use glutin::context::PossiblyCurrentContext;
use glutin::surface::GlSurface;
use glutin::surface::Surface;
use glutin::surface::WindowSurface;

use super::gl;


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
    let () = self.pop_matrizes();
    let () = self.pop_states();

    let () = self
      .surface
      .swap_buffers(&self.context)
      .context("failed to swap OpenGL buffers")?;
    Ok(())
  }
}
