// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::iter;
use std::slice;
use std::vec;

use crate::Point;

#[cfg(test)]
use super::super::matrix::Matrix;
use super::super::stonelike::rotate;
use super::super::Stonelike;
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
