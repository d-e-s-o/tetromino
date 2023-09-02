// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;


#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct Point<T> {
  pub x: T,
  pub y: T,
}

impl<T> Point<T> {
  #[inline]
  pub const fn new(x: T, y: T) -> Self {
    Self { x, y }
  }

  /// Convert the `Point` into one with a different `T`, assuming there
  /// exists a lossless conversion between the two inner types.
  pub(crate) fn into_other<U>(self) -> Point<U>
  where
    U: From<T>,
  {
    Point {
      x: U::from(self.x),
      y: U::from(self.y),
    }
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

impl<T> Sub<Point<T>> for Point<T>
where
  T: SubAssign<T>,
{
  type Output = Point<T>;

  #[inline]
  fn sub(mut self, other: Point<T>) -> Self::Output {
    self -= other;
    self
  }
}

impl<T> SubAssign<Point<T>> for Point<T>
where
  T: SubAssign<T>,
{
  #[inline]
  fn sub_assign(&mut self, rhs: Point<T>) {
    self.x -= rhs.x;
    self.y -= rhs.y;
  }
}
