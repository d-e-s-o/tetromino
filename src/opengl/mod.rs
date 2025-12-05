// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

// Note: Most of the font functionality is independent of the OpenGL
//       backend entirely and could be factored out (presumably with
//       minor changes) should we ever add more backends.
mod font;
mod renderer;
mod texture;
mod window;

pub(crate) use font::Font;
pub(crate) use renderer::ActiveRenderer;
pub(crate) use renderer::Color;
pub(crate) use texture::Texture;
#[cfg(test)]
pub(crate) use window::with_opengl_context;

pub(crate) type Mat4f = vema::Matrix<f32, 4>;

pub use renderer::Renderer;
pub use window::Context;
pub use window::Window;


#[expect(unused_imports)]
#[allow(clippy::allow_attributes, clippy::all)]
mod gl {
  #[link(name = "GL")]
  extern "system" {}

  include!("bindings.rs");

  pub use self::types::*;
}
