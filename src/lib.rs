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
mod winit;

use std::cmp::Ordering;
use std::time::Instant;

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
pub use crate::winit::Context;
pub use crate::winit::Window;


/// An enumeration describing when the next program "tick" should occur.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tick {
  /// The next tick should happen at the given instant.
  At(Instant),
  /// No additional tick is necessary at this point.
  None,
}

impl From<Option<Instant>> for Tick {
  fn from(other: Option<Instant>) -> Self {
    match other {
      Some(instant) => Tick::At(instant),
      None => Tick::None,
    }
  }
}

impl PartialOrd<Tick> for Tick {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Tick {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::None, Self::None) => Ordering::Equal,
      (Self::At(_instant), Self::None) => Ordering::Less,
      (Self::None, Self::At(_instant)) => Ordering::Greater,
      (Self::At(instant1), Self::At(instant2)) => instant1.cmp(instant2),
    }
  }
}


// This function is really only meant to be used by the main program.
#[doc(hidden)]
pub fn run() -> Result<()> {
  winit::run_app()
}
