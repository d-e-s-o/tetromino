// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::BitOr;
use std::ops::BitOrAssign;


/// An enumeration of possible state changes performed/desired by lower
/// level parts of the program.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Change {
  /// Some state was changed that necessitates a redraw.
  Changed,
  /// A desire to quite the program has been made.
  Quit,
  /// No visible state has changed.
  #[default]
  Unchanged,
}

impl BitOr<Change> for Change {
  type Output = Change;

  fn bitor(self, rhs: Change) -> Self::Output {
    match (self, rhs) {
      (Self::Quit, _) | (_, Self::Quit) => Self::Quit,
      (Self::Changed, _) | (_, Self::Changed) => Self::Changed,
      (Self::Unchanged, Self::Unchanged) => Self::Unchanged,
    }
  }
}

impl BitOrAssign<Change> for Change {
  fn bitor_assign(&mut self, rhs: Change) {
    *self = *self | rhs;
  }
}
