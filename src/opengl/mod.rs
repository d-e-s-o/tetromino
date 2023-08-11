// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

mod renderer;
mod window;

pub(crate) use renderer::Color;
pub(crate) use renderer::Renderer;
pub(crate) use window::Window;

#[allow(clippy::all)]
mod gl {
  #[link(name = "GL")]
  extern "system" {}

  include!("bindings.rs");

  pub use self::types::*;
}
