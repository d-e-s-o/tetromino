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
mod guard;
mod keys;
mod mode;
mod opengl;
mod point;
mod rand;
mod rect;
mod tick;
mod winit;

use anyhow::Result;

use crate::mode::ColorMode;
use crate::mode::ColorSet;
use crate::opengl::ActiveRenderer;
use crate::opengl::Color;
use crate::opengl::Font;
use crate::opengl::Texture;
use crate::opengl::TextureBuilderExt;
use crate::point::Point;
use crate::rand::Rng;
use crate::rect::Rect;

pub use crate::change::Change;
#[doc(hidden)]
pub use crate::config::Config;
pub use crate::game::Config as GameConfig;
pub use crate::game::Game;
pub use crate::opengl::Renderer;
pub use crate::tick::Tick;
pub use crate::winit::Context;
pub use crate::winit::Window;


// This function is really only meant to be used by the main program.
#[doc(hidden)]
pub fn run() -> Result<()> {
  winit::run_app()
}
