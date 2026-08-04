// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::ColorMode;
use crate::ColorSet;
use crate::Point;
use crate::Rect;


/// The representation of a single "piece" of a stone.
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub(crate) struct Piece {
  /// The index of the color that the piece has.
  color_idx: u8,
}

impl Piece {
  /// The set of colors we use for pieces.
  pub(super) const COLORS: &'static [ColorSet] = &[
    ColorSet::new(Color::red(), Color::red()),
    ColorSet::new(Color::green(), Color::green()),
    ColorSet::new(Color::yellow(), Color::yellow()),
    ColorSet::new(Color::violet(), Color::violet()),
    ColorSet::new(Color::blue(), Color::blue()),
    ColorSet::new(Color::cyan(), Color::cyan()),
    ColorSet::new(Color::gray(), Color::orange()),
  ];

  pub fn new(color_idx: u8) -> Self {
    debug_assert!(usize::from(color_idx) < Self::COLORS.len());
    Self { color_idx }
  }

  /// # Notes
  /// This method assumes that the piece texture to use is already
  /// bound.
  pub fn render(&self, renderer: &Renderer, location: Point<i16>, color_mode: ColorMode) {
    // Perhaps counter-intuitively, the color black acts as a neutral
    // component here.
    self.render_with_overlay(renderer, location, color_mode, Color::black())
  }

  /// Render the piece with the provided color as "overlay".
  pub fn render_with_overlay(
    &self,
    renderer: &Renderer,
    location: Point<i16>,
    color_mode: ColorMode,
    overlay: Color,
  ) {
    let color = Self::COLORS[usize::from(self.color_idx)].select(color_mode);
    let _guard = renderer.set_color(color + overlay);

    let () = renderer.render_rect(Rect::new(location.x, location.y, 1, 1));
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::mem::size_of;


  /// Check that objects of our [`Piece`] type have the expected size.
  #[test]
  fn piece_size() {
    assert_eq!(size_of::<Piece>(), 1);
  }
}
