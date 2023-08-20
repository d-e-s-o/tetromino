// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Add;
use std::ops::AddAssign;

use crate::Point;


#[derive(Clone, Copy, Debug)]
pub(crate) struct Rect<T> {
  pub x: T,
  pub y: T,
  pub w: T,
  pub h: T,
}

impl<T> Rect<T> {
  #[inline]
  pub(crate) fn new(x: T, y: T, w: T, h: T) -> Self {
    Self { x, y, w, h }
  }

  /// Convert the `Rect` into one with a different `T`, assuming there
  /// exists a lossless conversion between the two inner types.
  pub(crate) fn into_other<U>(self) -> Rect<U>
  where
    U: From<T>,
  {
    Rect {
      x: U::from(self.x),
      y: U::from(self.y),
      w: U::from(self.w),
      h: U::from(self.h),
    }
  }
}

impl<T> From<(T, T, T, T)> for Rect<T> {
  #[inline]
  fn from((x, y, w, h): (T, T, T, T)) -> Self {
    Self::new(x, y, w, h)
  }
}

impl<T> Add<Point<T>> for Rect<T>
where
  T: AddAssign<T>,
{
  type Output = Rect<T>;

  #[inline]
  fn add(mut self, other: Point<T>) -> Self::Output {
    self += other;
    self
  }
}

impl<T> AddAssign<Point<T>> for Rect<T>
where
  T: AddAssign<T>,
{
  #[inline]
  fn add_assign(&mut self, rhs: Point<T>) {
    self.x += rhs.x;
    self.y += rhs.y;
  }
}
