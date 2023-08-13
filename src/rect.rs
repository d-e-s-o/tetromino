// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later


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
}

impl<T> From<(T, T, T, T)> for Rect<T> {
  #[inline]
  fn from((x, y, w, h): (T, T, T, T)) -> Self {
    Self::new(x, y, w, h)
  }
}
