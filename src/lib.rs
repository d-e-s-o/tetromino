// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A graphical Tetris clone.

#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

mod app;
mod change;
mod config;
mod game;
mod gl;
mod guard;
mod instant;
mod keys;
mod mode;
mod point;
mod rand;
mod rect;
mod tick;
#[cfg(not(target_arch = "wasm32"))]
mod winit;

#[cfg(not(target_arch = "wasm32"))]
use anyhow::Result;

use crate::gl::ActiveRenderer;
use crate::gl::Color;
use crate::gl::Font;
use crate::gl::Texture;
use crate::gl::TextureBuilderExt;
use crate::mode::ColorMode;
use crate::mode::ColorSet;
use crate::point::Point;
use crate::rand::Rng;
use crate::rect::Rect;

pub use crate::change::Change;
#[doc(hidden)]
pub use crate::config::Config;
pub use crate::game::Config as GameConfig;
pub use crate::game::Game;
pub use crate::gl::Renderer;
pub use crate::instant::Instant;
pub use crate::tick::Tick;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::winit::Context;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::winit::Window;


// This function is really only meant to be used by the main program.
#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
pub fn run() -> Result<()> {
  winit::run_app()
}
