// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg_attr(target_arch = "wasm32", expect(unused_imports))]

use std::fs::read_to_string;
use std::io::ErrorKind;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;

use serde::Deserialize;
use serde::Serialize;

use toml_edit::de::from_str as from_toml_str;

use crate::game;
use crate::keys;


/// Retrieve the default path to the program's configuration file.
#[cfg(not(target_arch = "wasm32"))]
fn default_config_path() -> Result<PathBuf> {
  use dirs::config_dir;

  let config = config_dir()
    .context("unable to determine config directory")?
    .join("tetromino")
    .join("config.toml");

  Ok(config)
}


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

impl Config {
  /// Load the configuration from its default path on the file system.
  #[cfg(not(target_arch = "wasm32"))]
  pub fn load() -> Result<Self> {
    let path = default_config_path().context("failed to retrieve program config directory path")?;
    let contents = match read_to_string(&path) {
      Ok(contents) => contents,
      Err(err) if err.kind() == ErrorKind::NotFound => return Ok(Config::default()),
      e @ Err(..) => e.with_context(|| {
        format!(
          "failed to load program configuration at `{}`",
          path.display()
        )
      })?,
    };
    let config = from_toml_str(&contents)
      .with_context(|| format!("failed to parse TOML configuration at `{}`", path.display()))?;
    Ok(config)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use toml_edit::de::from_str as from_toml_str;
  use toml_edit::ser::to_string_pretty as to_toml_string;


  /// Make sure that we can successfully deserialize various
  /// configurations.
  #[test]
  fn deserialization() {
    // Complete config with everything set.
    let config = Config::default();
    let config = to_toml_string(&config).unwrap();
    assert!(from_toml_str::<Config>(&config).is_ok());

    // Config without keyboard data.
    let config = r#"
[game]
start_level = 1
lines_for_level = 10
field_width = 10
field_height = 20
preview_stone_count = 1
enable_ai = false
enable_dark_mode = false
    "#;
    assert!(from_toml_str::<Config>(config).is_ok());

    // Partial game config.
    let config = r#"
[game]
start_level = 1
lines_for_level = 10
    "#;
    assert!(from_toml_str::<Config>(config).is_ok());
  }
}
