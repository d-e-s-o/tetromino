// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

// Note: Most of the font functionality is independent of the OpenGL
//       backend entirely and could be factored out (presumably with
//       minor changes) should we ever add more backends.
mod font;
mod renderer;
mod texture;

pub(crate) use font::Font;
pub(crate) use renderer::ActiveRenderer;
pub(crate) use renderer::Color;
pub(crate) use texture::Texture;
pub(crate) use texture::TextureBuilderExt;
pub(crate) use texture::empty_texture;

pub(crate) type Mat4f = vema::Matrix<f32, 4>;

pub use renderer::Renderer;
