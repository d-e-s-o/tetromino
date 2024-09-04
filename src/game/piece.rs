// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::ColorMode;
use crate::ColorSet;
use crate::Point;
use crate::Rect;


/// The representation of a single "piece" of a stone.
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub(crate) struct Piece {
  /// The index of the color that the piece has.
  color_idx: u8,
  /// The color mode in use.
  // TODO: For whatever reason not even a `ColorMode<NonZeroU8>` fits
  //       into a single byte, when conceptually it should. Should get
  //       to the bottom of it.
  color_mode: ColorMode<()>,
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

  pub(crate) fn new(color_idx: u8) -> Self {
    debug_assert!(usize::from(color_idx) < Self::COLORS.len());
    Self {
      color_idx,
      color_mode: ColorMode::default(),
    }
  }

  /// # Notes
  /// This method assumes that the piece texture to use is already
  /// bound.
  pub(crate) fn render(&self, renderer: &Renderer, location: Point<i16>) {
    // Perhaps counter-intuitively, the color black acts as a neutral
    // component here.
    self.render_with_overlay(renderer, location, Color::black())
  }

  /// Render the piece with the provided color as "overlay".
  pub(crate) fn render_with_overlay(
    &self,
    renderer: &Renderer,
    location: Point<i16>,
    overlay: Color,
  ) {
    let color = Self::COLORS[usize::from(self.color_idx)].select(&self.color_mode);
    let _guard = renderer.set_color(color + overlay);

    let () = renderer.render_rect(Rect::new(location.x, location.y, 1, 1));
  }

  /// Set the piece's color mode.
  #[inline]
  pub(crate) fn set_color_mode(&mut self, mode: ColorMode<()>) {
    self.color_mode = mode;
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::mem::size_of;


  /// Check that objects of our [`Piece`] type have the expected size.
  #[test]
  fn piece_size() {
    assert_eq!(size_of::<Piece>(), 2);
  }
}
