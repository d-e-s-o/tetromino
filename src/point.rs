// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later


#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T> {
  #[inline]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T> From<(T, T)> for Point<T> {
  #[inline]
  fn from(other: (T, T)) -> Self {
    Self::new(other.0, other.1)
  }
}
