// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Index;
use std::ops::IndexMut;

use crate::point::Point;

use super::super::Fieldlike;
use super::super::Matrix;
use super::Stone;


#[derive(Clone, PartialEq)]
pub(crate) struct Field {
  /// The matrix (2D array) of "pieces".
  matrix: Matrix<Option<()>>,
}

impl Field {
  #[inline]
  pub fn from_matrix<P>(matrix: &Matrix<Option<P>>) -> Self {
    Self {
      matrix: matrix.to_other(|option| option.as_ref().map(|_piece| ())),
    }
  }
}

impl Index<Point<i16>> for Field {
  type Output = Option<()>;

  #[inline]
  fn index(&self, index: Point<i16>) -> &Self::Output {
    &self.matrix[(index.x, index.y)]
  }
}

impl IndexMut<Point<i16>> for Field {
  #[inline]
  fn index_mut(&mut self, index: Point<i16>) -> &mut Self::Output {
    &mut self.matrix[(index.x, index.y)]
  }
}

impl Fieldlike<Stone> for Field {
  #[inline]
  fn width(&self) -> i16 {
    self.matrix.width()
  }

  #[inline]
  fn height(&self) -> i16 {
    self.matrix.height()
  }

  #[inline]
  fn line_complete(&self, line: i16) -> bool {
    self.matrix.iter_line(line).all(|elem| elem.is_some())
  }

  #[inline]
  fn remove_line(&mut self, line: i16) {
    self.matrix.remove_line(line)
  }
}

impl Debug for Field {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    for y in (0..self.height()).rev() {
      for x in 0..self.width() {
        write!(
          f,
          "{}",
          if self.matrix[(x, y)].is_some() {
            '#'
          } else {
            '.'
          }
        )?;
      }
      writeln!(f)?;
    }
    Ok(())
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use super::super::super::Stonelike as _;
  use super::super::util::field;
  use super::super::util::stone;


  /// Check that we can properly "reset" a stone's position within a
  /// [`Field`].
  #[test]
  fn stone_reset() {
    let mut stone = stone! {"
      .#.
      ###
    "};
    let field = field! {"
      .......
      #......
      ##.....
      ##...##
    "};

    assert!(field.reset_stone(&mut stone));

    let bounds = stone.bounds();
    assert_eq!(bounds.x, 2);
    assert_eq!(bounds.y, 2);
  }


  /// Test that we can merge a stone with a field.
  #[test]
  fn stone_merge() {
    let stone = stone! {"
      ...#.
      ..###
    "};
    let mut field = field! {"
      .......
      #......
      ##.....
      ##...##
    "};

    let complete = field.merge_stone(stone);
    assert_eq!(complete, 1);

    let expected = field! {"
      .......
      #......
      ##.#...
      #######
    "};
    assert_eq!(field, expected);
  }


  /// Make sure that removal of completed lines works as it should.
  #[test]
  fn line_clearing() {
    let mut field = field! {"
      .......
      #......
      #######
      ##...##
    "};

    // No lines should get removed because the range does not cover the
    // completed line.
    let removed = field.remove_complete_lines(0..1);
    assert_eq!(removed, 0);

    let removed = field.remove_complete_lines(1..4);
    assert_eq!(removed, 1);

    let expected = field! {"
      .......
      .......
      #......
      ##...##
    "};
    assert_eq!(field, expected);
  }
}
