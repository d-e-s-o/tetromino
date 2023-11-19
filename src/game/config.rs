// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use serde::Serialize;


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  /// The level the game starts at.
  pub start_level: u16,
  /// The number of lines required to level up.
  pub lines_for_level: u16,
  /// The width of the tetris field.
  pub field_width: i16,
  /// The height of the tetris field.
  pub field_height: i16,
  /// The number of upcoming stones to show.
  pub preview_stone_count: u8,
  /// Whether or not the AI is enabled initially.
  pub enable_ai: bool,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      start_level: 1,
      lines_for_level: 10,
      field_width: 10,
      field_height: 20,
      preview_stone_count: 1,
      enable_ai: false,
    }
  }
}
