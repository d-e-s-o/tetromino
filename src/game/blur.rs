// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::Context as _;
use anyhow::Result;

use xgl::Framebuffer;
use xgl::Texture;
use xgl::TextureInfo;
use xgl::VertexArray;
use xgl::sys;
use xgl::sys::Gl as _;

use crate::gl::BlurRenderState;
use crate::gl::ColorExt as _;
use crate::gl::ObjectRenderState;


/// The width of the framebuffer rendered to for blur effects.
const BLUR_FB_WIDTH: u32 = 512;
/// The height of the framebuffer rendered to for blur effects.
const BLUR_FB_HEIGHT: u32 = 512;


#[derive(Debug)]
pub(super) struct Blur {
  /// An off-screen framebuffer that we render to to later facilitate
  /// blur post-processing.
  framebuffer: Framebuffer,
  /// The color attachment.
  color: Texture,
  /// An empty vertex array object.
  empty_vao: VertexArray,
}

impl Blur {
  pub fn new(context: &sys::Context) -> Result<Self> {
    let color_info = TextureInfo {
      width: BLUR_FB_WIDTH,
      height: BLUR_FB_WIDTH,
      intern_format: sys::TextureInternalFormat::RGBA8,
      pixel_format: sys::TexturePixelFormat::RGBA,
      color_format: sys::Type::UnsignedByte,
    };

    let color = Texture::builder()
      .set_wrap_mode(sys::TextureWrap::ClampToEdge)
      .set_context(context)
      .empty(&color_info)?;
    let framebuffer = Framebuffer::builder()
      .set_color_attachment(&color)
      .build(context)
      .context("failed to create framebuffer object")?;

    let empty_vao = VertexArray::empty(context)?;

    let slf = Self {
      framebuffer,
      color,
      empty_vao,
    };
    Ok(slf)
  }

  /// Render the scene to an off-screen framebuffer for later
  /// post-processing.
  pub fn render_scene<F>(&self, state: &mut ObjectRenderState, clear_color: (f32, f32, f32), f: F)
  where
    F: FnOnce(&mut ObjectRenderState),
  {
    // Switch to linear color output (disabling sRGB encoding), because
    // we render to an off-screen target and want to work with linear
    // values until we finally hit the screen.
    let () = state.set_linear_colors(true);
    let () = self.framebuffer.bind();
    let (r, g, b) = clear_color.to_linear();
    let () = state.set_clear_color(r, g, b, 1.0);
    let () = state.clear(sys::ClearMask::ColorBuffer);
    let () = state.set_viewport(0, 0, BLUR_FB_WIDTH as _, BLUR_FB_HEIGHT as _);
    let () = f(state);
    let () = self.framebuffer.unbind();
    let () = state.set_linear_colors(false);
  }

  /// Blur the previously rendered image and render it to the screen.
  pub fn render_blur(&self, state: &mut BlurRenderState) {
    let pos = 1.2;
    let sigma = 5.0;
    let () = state.set_blur_opts(pos, sigma);

    // NB: No need to clear the screen here as we are drawing to the
    //     entire screen anyway.
    let () = state.set_texture(&self.color);
    let () = self.empty_vao.bind();
    let () = state.draw_arrays(sys::Primitive::Triangles, 3);
  }
}
