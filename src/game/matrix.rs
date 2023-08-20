// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later


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
}
