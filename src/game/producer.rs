// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::fmt::Debug;

use super::Stone;


/// A trait implemented by something that can produce Tetris stones.
pub(crate) trait StoneProducer: Debug {
  /// Create a new random Tetris stone.
  fn create_stone(&self) -> Stone;
}
