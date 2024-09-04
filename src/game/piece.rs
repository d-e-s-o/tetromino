// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Point;
use crate::Rect;


/// The representation of a single "piece" of a stone.
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub(crate) struct Piece {
  /// The index of the color that the piece has.
  color_idx: u8,
}

impl Piece {
  /// The set of colors we use for pieces.
  pub(super) const COLORS: &'static [Color] = &[
    Color::red(),
    Color::green(),
    Color::yellow(),
    Color::violet(),
    Color::blue(),
    Color::cyan(),
    Color::gray(),
  ];

  pub(crate) fn new(color_idx: u8) -> Self {
    debug_assert!(usize::from(color_idx) < Self::COLORS.len());
    Self { color_idx }
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
    let color = Self::COLORS[usize::from(self.color_idx)];
    let _guard = renderer.set_color(color + overlay);

    let () = renderer.render_rect(Rect::new(location.x, location.y, 1, 1));
  }
}
