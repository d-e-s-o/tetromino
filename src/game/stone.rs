// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::max;
use std::cmp::min;

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Point;
use crate::Rect;
use crate::Texture;

use super::Piece;


/// The representation of a Tetris stone.
#[derive(Debug)]
pub(crate) struct Stone {
  /// The texture to use for an individual piece.
  piece_texture: Texture,
  /// The individual pieces making up the stone and their locations.
  /// Typically a stone has four pieces, but that's not set in stone.
  pieces: Vec<(Piece, Point<u16>)>,
}

impl Stone {
  pub(crate) fn new(piece_texture: Texture, template: &[Point<u8>], color: Color) -> Self {
    assert!(!template.is_empty(), "provided stone template is empty");

    Self {
      piece_texture,
      pieces: template
        .iter()
        .map(|p| (Piece::new(color), p.into_other()))
        .collect(),
    }
  }

  pub(crate) fn render(&self, renderer: &Renderer) {
    let _guard = renderer.set_texture(&self.piece_texture);

    let () = self
      .pieces
      .iter()
      .for_each(|(piece, location)| piece.render(renderer, *location));
  }

  /// # Notes
  /// Location manipulation is done in a wrapping manner. If underflow
  /// (or overflow) is a possibility that should be checked beforehand
  /// or accommodated afterwards.
  pub(crate) fn move_by(&mut self, x: i16, y: i16) {
    let () = self.pieces.iter_mut().for_each(|(_piece, location)| {
      location.x = location.x.wrapping_add_signed(x);
      location.y = location.y.wrapping_add_signed(y);
    });
  }

  pub(crate) fn move_to(&mut self, location: Point<u16>) {
    let bounds = self.bounds();
    let x = location.x.wrapping_sub(bounds.x) as _;
    let y = location.y.wrapping_sub(bounds.y) as _;

    self.move_by(x, y)
  }

  pub(crate) fn bounds(&self) -> Rect<u16> {
    // SANITY: Our stone always has at least one piece.
    let (_piece, location) = self.pieces.first().unwrap();
    let mut x_min = location.x;
    let mut x_max = location.x;
    let mut y_min = location.y;
    let mut y_max = location.y;

    for (_piece, location) in self.pieces.iter().skip(1) {
      x_min = min(x_min, location.x);
      x_max = max(x_max, location.x);
      y_min = min(y_min, location.y);
      y_max = max(y_max, location.y);
    }

    Rect {
      x: x_min,
      y: y_min,
      w: x_max + 1 - x_min,
      h: y_max + 1 - y_min,
    }
  }

  pub(crate) fn pieces(
    &self,
  ) -> impl Iterator<Item = Point<u16>> + DoubleEndedIterator + ExactSizeIterator + '_ {
    self.pieces.iter().map(|(_piece, location)| *location)
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  fn new_stone(template: &[Point<u8>]) -> Stone {
    Stone::new(Texture::invalid(), template, Color::black())
  }


  /// Check that the bounds of a `Stone` are calculated correctly.
  #[test]
  fn stone_bounds() {
    let template = [Point::new(1, 2)];
    let bounds = new_stone(&template).bounds();
    assert_eq!(bounds.x, 1);
    assert_eq!(bounds.y, 2);
    assert_eq!(bounds.w, 1);
    assert_eq!(bounds.h, 1);

    let template = [Point::new(1, 2), Point::new(3, 2)];
    let bounds = new_stone(&template).bounds();
    assert_eq!(bounds.x, 1);
    assert_eq!(bounds.y, 2);
    assert_eq!(bounds.w, 3);
    assert_eq!(bounds.h, 1);

    let template = [Point::new(1, 2), Point::new(0, 1)];
    let bounds = new_stone(&template).bounds();
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 1);
    assert_eq!(bounds.w, 2);
    assert_eq!(bounds.h, 2);

    let template = [
      Point::new(0, 0),
      Point::new(0, 1),
      Point::new(1, 0),
      Point::new(1, 1),
    ];
    let bounds = new_stone(&template).bounds();
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 0);
    assert_eq!(bounds.w, 2);
    assert_eq!(bounds.h, 2);
  }

  /// Check that we can move a `Stone` object as expected.
  #[test]
  fn stone_movement() {
    let template = [Point::new(1, 2), Point::new(0, 1)];
    let mut stone = new_stone(&template);
    let bounds = stone.bounds();
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 1);
    assert_eq!(bounds.w, 2);
    assert_eq!(bounds.h, 2);

    let () = stone.move_to(Point::new(3, 4));
    let bounds = stone.bounds();
    assert_eq!(bounds.x, 3);
    assert_eq!(bounds.y, 4);

    let () = stone.move_to(Point::new(0, 0));
    let bounds = stone.bounds();
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 0);
    assert_eq!(bounds.w, 2);
    assert_eq!(bounds.h, 2);
  }
}
