// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use serde::Serialize;

use crate::game;
use crate::keys;


/// A type representing the configuration of the program.
#[derive(Default, Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Config {
  /// Settings pertaining keyboard handling.
  #[serde(default)]
  pub keyboard: keys::Config,
  /// Configuration of the game itself.
  #[serde(default)]
  pub game: game::Config,
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Make sure that we can successfully deserialize various
  /// configurations.
  #[test]
  fn deserialization() {
    // Complete config with everything set.
    let config = Config::default();
    let config = toml::to_string_pretty(&config).unwrap();
    assert!(toml::from_str::<Config>(&config).is_ok());

    // Config without keyboard data.
    let config = r#"
[game]
start_level = 1
lines_for_level = 10
field_width = 10
field_height = 20
preview_stone_count = 1
enable_ai = false
    "#;
    assert!(toml::from_str::<Config>(config).is_ok());

    // Partial game config.
    let config = r#"
[game]
start_level = 1
lines_for_level = 10
    "#;
    assert!(toml::from_str::<Config>(config).is_ok());
  }
}
