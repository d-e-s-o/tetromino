// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Index;
use std::ops::IndexMut;

use crate::Point;


/// A 2D matrix.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Matrix<T> {
  /// The actual matrix.
  matrix: Box<[T]>,
  /// The width of the matrix.
  width: i16,
  /// The height of the matrix.
  height: i16,
}

impl<T> Matrix<T> {
  /// # Panics
  /// This constructor panics if either dimension is 0.
  pub(crate) fn new(width: i16, height: i16) -> Self
  where
    T: Default,
  {
    assert!(width > 0);
    assert!(height > 0);

    let mut matrix = Vec::new();
    let () = matrix.resize_with((width * height) as usize, T::default);

    Self {
      width,
      height,
      matrix: matrix.into_boxed_slice(),
    }
  }

  /// Remove the given line, moving all elements above one line down.
  pub(super) fn remove_line(&mut self, line: i16)
  where
    T: Copy + Default,
  {
    let src_index = self.calculate_index((0, line + 1));
    let dst_index = self.calculate_index((0, line));
    let src_range = src_index..;
    let () = self.matrix.copy_within(src_range, dst_index);

    // Now clear the very top line, as everything was copied one line
    // down.
    let src_index = self.calculate_index((0, self.height - 1));
    let src_range = src_index..src_index + self.width as usize;
    let () = self.matrix.get_mut(src_range).unwrap().fill(T::default());
  }

  /// Clear the matrix, removing all elements from it.
  #[inline]
  pub(super) fn clear(&mut self)
  where
    T: Clone + Default,
  {
    let () = self.matrix.fill(T::default());
  }

  /// Create an iterator over all elements, along with their positions.
  pub(crate) fn iter(&self) -> impl Iterator<Item = (&T, Point<i16>)> {
    let width = self.width as usize;

    self.matrix.iter().enumerate().map(move |(i, t)| {
      let x = i % width;
      let y = i / width;
      (t, Point::new(x as i16, y as i16))
    })
  }

  /// Create an iterator over all elements in the given line.
  pub(crate) fn iter_line(&self, line: i16) -> impl Iterator<Item = &T> {
    let index = self.calculate_index((0, line));
    self.matrix[index..index + self.width as usize].iter()
  }

  /// Convert the `Matrix` into one with a different `T`, using the
  /// provided function, `f`, to to convert individual items.
  pub(crate) fn to_other<U, F>(&self, f: F) -> Matrix<U>
  where
    F: Fn(&T) -> U,
  {
    Matrix {
      width: self.width,
      height: self.height,
      matrix: self.matrix.iter().map(f).collect(),
    }
  }

  #[inline]
  pub(crate) fn width(&self) -> i16 {
    self.width
  }

  #[inline]
  pub(crate) fn height(&self) -> i16 {
    self.height
  }

  fn calculate_index(&self, (x, y): (i16, i16)) -> usize {
    (x + y * self.width) as _
  }
}

impl<T> Index<(i16, i16)> for Matrix<T> {
  type Output = T;

  fn index(&self, index: (i16, i16)) -> &Self::Output {
    let index = self.calculate_index(index);
    &self.matrix[index]
  }
}

impl<T> Index<Point<i16>> for Matrix<T> {
  type Output = T;

  fn index(&self, index: Point<i16>) -> &Self::Output {
    self.index((index.x, index.y))
  }
}

impl<T> IndexMut<(i16, i16)> for Matrix<T> {
  fn index_mut(&mut self, index: (i16, i16)) -> &mut Self::Output {
    let index = self.calculate_index(index);
    &mut self.matrix[index]
  }
}

impl<T> IndexMut<Point<i16>> for Matrix<T> {
  fn index_mut(&mut self, index: Point<i16>) -> &mut Self::Output {
    self.index_mut((index.x, index.y))
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Check that indexing into a `Matrix` object works as it should.
  #[test]
  fn index_access() {
    let mut matrix = Matrix::<Option<usize>>::new(3, 6);
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

  /// Make sure that we can remove a line from the matrix.
  #[test]
  fn line_removal() {
    let mut matrix = Matrix::<Option<usize>>::new(2, 4);
    let mut x = 0;
    let () = matrix.matrix.fill_with(|| {
      x += 1;
      Some(x)
    });

    // |7,8|
    // |5,6|
    // |3,4|
    // |1,2|
    // -----
    #[rustfmt::skip]
    let expected = [
      Some(1), Some(2),
      Some(3), Some(4),
      Some(5), Some(6),
      Some(7), Some(8),
    ];
    assert_eq!(&*matrix.matrix, expected.as_slice());

    let () = matrix.remove_line(0);
    // |   |
    // |7,8|
    // |5,6|
    // |3,4|
    // -----
    #[rustfmt::skip]
    let expected = [
      Some(3), Some(4),
      Some(5), Some(6),
      Some(7), Some(8),
      None, None,
    ];
    assert_eq!(&*matrix.matrix, expected.as_slice());
  }
}
