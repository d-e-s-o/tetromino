// Copyright (C) 2024-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::Color;


/// The color mode currently in use.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) enum ColorMode {
  /// Light mode is currently active.
  #[default]
  Light,
  /// Dark mode is currently active.
  Dark,
}

impl ColorMode {
  /// Toggle the currently active color.
  #[inline]
  pub fn toggle(&mut self) {
    match self {
      Self::Light => *self = Self::Dark,
      Self::Dark => *self = Self::Light,
    }
  }
}


/// A set of two colors, one for light mode and another for dark mode.
pub(crate) struct ColorSet<T = Color> {
  pub light: T,
  pub dark: T,
}

impl<T> ColorSet<T> {
  #[inline]
  pub const fn new(light: T, dark: T) -> Self {
    Self { light, dark }
  }

  /// Retrieve the appropriate color based on the provided mode.
  #[inline]
  pub fn select(&self, mode: ColorMode) -> T
  where
    T: Copy,
  {
    match mode {
      ColorMode::Light => self.light,
      ColorMode::Dark => self.dark,
    }
  }
}
