// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use serde::Serialize;


fn default_start_level() -> u16 {
  1
}

fn default_lines_for_level() -> u16 {
  10
}

fn default_field_width() -> i16 {
  10
}

fn default_field_height() -> i16 {
  20
}

fn default_preview_stone_count() -> u8 {
  1
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
  /// The level the game starts at.
  #[serde(default = "default_start_level")]
  pub start_level: u16,
  /// The number of lines required to level up.
  #[serde(default = "default_lines_for_level")]
  pub lines_for_level: u16,
  /// The width of the tetris field.
  #[serde(default = "default_field_width")]
  pub field_width: i16,
  /// The height of the tetris field.
  #[serde(default = "default_field_height")]
  pub field_height: i16,
  /// The number of upcoming stones to show.
  #[serde(default = "default_preview_stone_count")]
  pub preview_stone_count: u8,
  /// Whether or not the AI is enabled initially.
  #[serde(default)]
  pub enable_ai: bool,
}

impl Default for Config {
  fn default() -> Self {
    Self {
      start_level: default_start_level(),
      lines_for_level: default_lines_for_level(),
      field_width: default_field_width(),
      field_height: default_field_height(),
      preview_stone_count: default_preview_stone_count(),
      enable_ai: Default::default(),
    }
  }
}
