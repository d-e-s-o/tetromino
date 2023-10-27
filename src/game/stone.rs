// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::iter;
use std::mem::take;
use std::slice;
use std::vec;

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Point;
use crate::Texture;

use super::Piece;
use super::Stonelike;


/// The representation of a Tetris stone.
#[derive(Debug)]
pub(crate) struct Stone {
  /// The texture to use for an individual piece.
  piece_texture: Texture,
  /// The individual pieces making up the stone and their locations.
  /// Typically a stone has four pieces, but that's not set in stone.
  pieces: Box<[(Piece, Point<i16>)]>,
}

impl Stone {
  pub(crate) fn new(piece_texture: Texture, template: &[Point<i8>], color: Color) -> Self {
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
    self.render_with_overlay(renderer, Color::black())
  }

  pub(crate) fn render_with_overlay(&self, renderer: &Renderer, overlay: Color) {
    let _guard = renderer.set_texture(&self.piece_texture);

    let () = self
      .pieces
      .iter()
      .for_each(|(piece, location)| piece.render_with_overlay(renderer, *location, overlay));
  }

  /// Rip out the object's guts, creating a new stone and leaving this
  /// one effectively empty.
  // This method is a convenience helper for `Stone` usage in enums
  // allowing us to omit unnecessary clones due to limitations of Rust.
  pub(crate) fn take(&mut self) -> Self {
    Self {
      piece_texture: self.piece_texture.clone(),
      pieces: take(&mut self.pieces),
    }
  }
}

impl Stonelike for Stone {
  type Piece = Piece;
  type PieceIter<'slf> =
    iter::Map<slice::Iter<'slf, (Piece, Point<i16>)>, fn(&'slf (Piece, Point<i16>)) -> Point<i16>>;
  type PieceIterMut<'slf> = iter::Map<
    slice::IterMut<'slf, (Piece, Point<i16>)>,
    fn(&'slf mut (Piece, Point<i16>)) -> &'slf mut Point<i16>,
  >;
  type IntoPiecesIter = vec::IntoIter<(Piece, Point<i16>)>;

  #[inline]
  fn pieces(&self) -> Self::PieceIter<'_> {
    self.pieces.iter().map(|(_piece, location)| *location)
  }

  #[inline]
  fn pieces_mut(&mut self) -> Self::PieceIterMut<'_> {
    self.pieces.iter_mut().map(|(_piece, location)| location)
  }

  #[inline]
  fn into_pieces(self) -> Self::IntoPiecesIter {
    Vec::from(self.pieces).into_iter()
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  fn new_stone(template: &[Point<i8>]) -> Stone {
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

  /// Check that we can rotate a `Stone` object as expected.
  #[test]
  fn stone_rotation() {
    // T stone
    let template = [
      Point::new(0, 0),
      Point::new(1, 0),
      Point::new(1, 1),
      Point::new(2, 0),
    ];
    let mut stone = new_stone(&template);
    let () = stone.move_to(Point::new(6, 4));
    let before = stone.pieces().collect::<Vec<_>>();
    let () = stone.rotate_left();
    let after = stone.pieces().collect::<Vec<_>>();

    assert_ne!(before, after);
  }
}
