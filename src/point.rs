// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Add;
use std::ops::AddAssign;


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

impl<T> Add<Point<T>> for Point<T>
where
  T: AddAssign<T>,
{
  type Output = Point<T>;

  #[inline]
  fn add(mut self, other: Point<T>) -> Self::Output {
    self += other;
    self
  }
}

impl<T> AddAssign<Point<T>> for Point<T>
where
  T: AddAssign<T>,
{
  #[inline]
  fn add_assign(&mut self, rhs: Point<T>) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}
