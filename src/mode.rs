// Copyright (C) 2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::Color;


/// The color mode currently in use.
#[derive(Clone, Copy, Debug)]
pub(crate) enum ColorMode<T = Color> {
  /// Light mode is currently active.
  Light(T),
  /// Dark mode is currently active.
  Dark(T),
}

impl<T> ColorMode<T> {
  /// Retrieve the currently active color.
  #[inline]
  pub fn color(&self) -> T
  where
    T: Copy,
  {
    match self {
      Self::Light(c) | Self::Dark(c) => *c,
    }
  }
}

impl ColorMode<Color> {
  /// Toggle the currently active color.
  #[inline]
  pub fn toggle_with(&mut self, colors: &ColorSet) {
    match self {
      Self::Light(..) => *self = Self::Dark(colors.dark),
      Self::Dark(..) => *self = Self::Light(colors.light),
    }
  }
}


/// A set of two colors, one for light mode and another for dark mode.
pub(crate) struct ColorSet {
  pub light: Color,
  pub dark: Color,
}

impl ColorSet {
  #[inline]
  pub const fn new(light: Color, dark: Color) -> Self {
    Self { light, dark }
  }
}
