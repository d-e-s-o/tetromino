// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

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
}

impl Renderer {
  pub(super) fn new(surface: Surface<WindowSurface>, context: PossiblyCurrentContext) -> Self {
    Self { surface, context }
  }

  pub(crate) fn on_pre_render(&self) -> Result<()> {
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
    let () = self
      .surface
      .swap_buffers(&self.context)
      .context("failed to swap OpenGL buffers")?;
    Ok(())
  }
}
