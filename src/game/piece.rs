// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Point;
use crate::Rect;


/// The representation of a single "piece" of a stone.
#[derive(Clone, Debug)]
pub(crate) struct Piece {
  /// The color the piece has.
  color: Color,
}

impl Piece {
  pub(crate) fn new(color: Color) -> Self {
    Self { color }
  }

  /// # Notes
  /// This method assumes that the piece texture to use is already
  /// bound.
  pub(crate) fn render(&self, renderer: &Renderer, location: Point<u16>) {
    let _guard = renderer.set_color(self.color);

    let () = renderer.render_rect(Rect::new(location.x, location.y, 1, 1));
  }
}
