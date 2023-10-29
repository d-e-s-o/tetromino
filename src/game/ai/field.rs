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
