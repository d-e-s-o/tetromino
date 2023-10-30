// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later


/// A enumeration of the actions to be performed on a stone.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Action {
  Merge,
  MoveDown,
  MoveLeft,
  MoveRight,
  RotateLeft,
  RotateRight,
}


#[cfg(test)]
mod tests {
  use std::mem::size_of;

  use super::*;


  /// Make sure that our [`Action`] type is sufficiently small and
  /// contains at least one niche bit.
  #[test]
  fn action_size() {
    assert_eq!(size_of::<Option<Action>>(), 1);
  }
}
