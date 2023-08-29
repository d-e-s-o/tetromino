// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Index;
use std::ops::IndexMut;

use crate::Point;


/// A 2D matrix.
#[derive(Debug)]
pub(crate) struct Matrix<T> {
  /// The actual matrix.
  matrix: Box<[Option<T>]>,
  /// The width of the matrix.
  width: u16,
  /// The height of the matrix.
  height: u16,
}

impl<T> Matrix<T> {
  /// # Panics
  /// This constructor panics if either dimension is 0.
  pub(crate) fn new(width: u16, height: u16) -> Self {
    assert!(width > 0);
    assert!(height > 0);

    let mut matrix = Vec::new();
    let () = matrix.resize_with(usize::from(width) * usize::from(height), Option::default);

    Self {
      width,
      height,
      matrix: matrix.into_boxed_slice(),
    }
  }

  #[inline]
  pub(crate) fn width(&self) -> u16 {
    self.width
  }

  #[inline]
  pub(crate) fn height(&self) -> u16 {
    self.height
  }

  fn calculate_index(&self, (x, y): (u16, u16)) -> usize {
    usize::from(x) + usize::from(y) * usize::from(self.width)
  }
}

impl<T> Index<(u16, u16)> for Matrix<T> {
  type Output = Option<T>;

  fn index(&self, index: (u16, u16)) -> &Self::Output {
    let index = self.calculate_index(index);
    &self.matrix[index]
  }
}

impl<T> Index<Point<u16>> for Matrix<T> {
  type Output = Option<T>;

  fn index(&self, index: Point<u16>) -> &Self::Output {
    self.index((index.x, index.y))
  }
}

impl<T> IndexMut<(u16, u16)> for Matrix<T> {
  fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
    let index = self.calculate_index(index);
    &mut self.matrix[index]
  }
}

impl<T> IndexMut<Point<u16>> for Matrix<T> {
  fn index_mut(&mut self, index: Point<u16>) -> &mut Self::Output {
    self.index_mut((index.x, index.y))
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Check that indexing into a `Matrix` object works as it should.
  #[test]
  fn index_access() {
    let mut matrix = Matrix::<usize>::new(3, 6);
    assert_eq!(matrix.width(), 3);
    assert_eq!(matrix.height(), 6);

    // Set lower-left corner element without using our custom `Index`
    // impl.
    matrix.matrix[0] = Some(42);
    assert_eq!(matrix[(0, 0)], Some(42));

    // Set lower-right corner element.
    matrix.matrix[2] = Some(43);
    assert_eq!(matrix[(2, 0)], Some(43));

    // Set upper-left corner element.
    matrix.matrix[15] = Some(44);
    assert_eq!(matrix[(0, 5)], Some(44));

    // Set upper-right corner element.
    matrix.matrix[17] = Some(45);
    assert_eq!(matrix[(2, 5)], Some(45));
  }
}
