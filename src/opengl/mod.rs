// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later


#[allow(clippy::all)]
mod gl {
  #[link(name = "GL")]
  extern "system" {}

  include!("bindings.rs");

  pub use self::types::*;
}
