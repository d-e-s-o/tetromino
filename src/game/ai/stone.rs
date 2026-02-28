// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::iter;
use std::slice;
use std::vec;

use crate::Point;

use super::super::Stonelike;
#[cfg(test)]
use super::super::matrix::Matrix;
use super::super::stonelike::rotate;
use super::Orientation;


#[derive(Clone, Debug)]
pub(crate) struct Stone {
  /// The pieces making up the stone and their locations.
  pieces: Box<[((), Point<i16>)]>,
  /// The stone's orientation, relative to its initial state.
  orientation: Orientation,
}

impl Stone {
  /// Create an AI stone from the locations of individual pieces.
  pub fn from_pieces<P>(pieces: P) -> Self
  where
    P: Iterator<Item = Point<i16>>,
  {
    Self {
      pieces: pieces.map(|location| ((), location)).collect(),
      orientation: Orientation::Rotated0,
    }
  }

  #[cfg(test)]
  pub(super) fn from_matrix<P>(matrix: &Matrix<Option<P>>) -> Self {
    Self::from_pieces(
      matrix
        .iter()
        .filter_map(|(p, location)| p.as_ref().map(|_p| location)),
    )
  }

  /// Retrieve the stone's orientation relative to its default.
  #[inline]
  pub(super) fn orientation(&self) -> Orientation {
    self.orientation
  }
}

impl Stonelike for Stone {
  type Piece = ();
  type PieceIter<'slf> =
    iter::Map<slice::Iter<'slf, ((), Point<i16>)>, fn(&'slf ((), Point<i16>)) -> Point<i16>>;
  type PieceIterMut<'slf> = iter::Map<
    slice::IterMut<'slf, ((), Point<i16>)>,
    fn(&'slf mut ((), Point<i16>)) -> &'slf mut Point<i16>,
  >;
  type IntoPiecesIter = vec::IntoIter<((), Point<i16>)>;

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

  fn rotate(&mut self, left: bool) {
    if left {
      self.orientation = self.orientation.rotate_left();
    } else {
      self.orientation = self.orientation.rotate_right();
    }

    rotate(self, left)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[cfg(feature = "nightly")]
  use std::hint::black_box;

  #[cfg(feature = "nightly")]
  use test::Bencher;

  use super::super::util::assert_stones_eq;
  use super::super::util::stone;


  /// Check that stone rotation works as expected.
  #[test]
  fn rotation_left() {
    let mut stone = stone! {"
      #.
      #.
      ##
    "};

    let () = stone.rotate_left();
    let expected = stone! {"
      ..#
      ###
      ...
    "};
    assert_stones_eq(&stone, &expected);

    let () = stone.rotate_left();
    let expected = stone! {"
      ##
      .#
      .#
    "};
    assert_stones_eq(&stone, &expected);

    let () = stone.rotate_left();
    let expected = stone! {"
      ###
      #..
      ...
    "};
    assert_stones_eq(&stone, &expected);

    let () = stone.rotate_left();
    let expected = stone! {"
      #.
      #.
      ##
    "};
    assert_stones_eq(&stone, &expected);
  }


  /// Check that stone rotation works as expected.
  #[test]
  fn rotation_right() {
    // We need to position the stone at (0,1) here because rotation
    // makes one of the pieces have a negative y-coordinate.
    let mut stone = stone! {"
      .##
      ##.
      ...
    "};

    let () = stone.rotate_right();
    let expected = stone! {"
      #.
      ##
      .#
    "};
    assert_stones_eq(&stone, &expected);

    let () = stone.rotate_right();
    let expected = stone! {"
      .##
      ##.
      ...
    "};
    assert_stones_eq(&stone, &expected);

    let () = stone.rotate_right();
    let expected = stone! {"
      #.
      ##
      .#
    "};
    assert_stones_eq(&stone, &expected);
  }


  /// Benchmark rotation of a stone.
  #[cfg(feature = "nightly")]
  #[bench]
  fn bench_stone_rotation(b: &mut Bencher) {
    let pieces = [
      Point::new(0, 0),
      Point::new(0, 1),
      Point::new(0, 2),
      Point::new(0, 3),
    ];
    let mut stone = Stone::from_pieces(pieces.into_iter());

    let () = b.iter(|| {
      let () = black_box(&mut stone).rotate(black_box(false));
    });
  }
}
