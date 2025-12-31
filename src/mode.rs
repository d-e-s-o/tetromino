// Copyright (C) 2024-2025 Daniel Mueller <deso@posteo.net>
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

  /// Create a new [`ColorMode`] object without any payload.
  #[inline]
  pub fn stripped(&self) -> ColorMode<()> {
    match self {
      Self::Light(..) => ColorMode::Light(()),
      Self::Dark(..) => ColorMode::Dark(()),
    }
  }
}

impl ColorMode<()> {
  /// Toggle the currently active color.
  #[inline]
  pub fn toggle(&mut self) {
    match self {
      Self::Light(()) => *self = Self::Dark(()),
      Self::Dark(()) => *self = Self::Light(()),
    }
  }
}

impl<T> ColorMode<T>
where
  T: Copy,
{
  /// Toggle the currently active color.
  #[inline]
  pub fn toggle_with(&mut self, colors: &ColorSet<T>) {
    match self {
      Self::Light(..) => *self = Self::Dark(colors.dark),
      Self::Dark(..) => *self = Self::Light(colors.light),
    }
  }
}

impl Default for ColorMode<()> {
  fn default() -> Self {
    Self::Light(())
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
  pub fn select(&self, mode: &ColorMode<()>) -> T
  where
    T: Copy,
  {
    match mode {
      ColorMode::Light(()) => self.light,
      ColorMode::Dark(()) => self.dark,
    }
  }
}
