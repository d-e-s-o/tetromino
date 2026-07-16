// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg_attr(target_arch = "wasm32", expect(dead_code, unused_imports))]

use std::fs::create_dir_all;
use std::fs::read_to_string;
use std::fs::write;
use std::io::ErrorKind;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr as _;

use anyhow::Context as _;
use anyhow::Result;

use serde::Deserialize;
use serde::Serialize;

use toml_edit::DocumentMut;
use toml_edit::de::from_document as from_toml_doc;

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

fn load_config_doc(path: &Path) -> Result<Option<DocumentMut>> {
  let contents = match read_to_string(path) {
    Ok(contents) => contents,
    Err(err) if err.kind() == ErrorKind::NotFound => return Ok(None),
    e @ Err(..) => e.with_context(|| {
      format!(
        "failed to load program configuration at `{}`",
        path.display()
      )
    })?,
  };

  let doc = DocumentMut::from_str(&contents)
    .with_context(|| format!("failed to parse TOML configuration at `{}`", path.display()))?;
  Ok(Some(doc))
}

fn load_config(path: &Path) -> Result<Config> {
  let doc = load_config_doc(path)?;

  if let Some(doc) = doc {
    from_toml_doc(doc).with_context(|| {
      format!(
        "failed to deserialize configuration from `{}`",
        path.display()
      )
    })
  } else {
    Ok(Config::default())
  }
}


fn save_config(config: &Config, path: &Path) -> Result<()> {
  macro_rules! update {
    ($doc:expr, $field:expr, as int) => {{
      $doc[stringify!($field)] = toml_edit::value(i64::from($field));
    }};
    ($doc:expr, $field:expr) => {{
      $doc[stringify!($field)] = toml_edit::value($field);
    }};
  }


  let mut doc = load_config_doc(path)?.unwrap_or_default();
  let current_result = from_toml_doc(doc.clone());

  if let Some(dir) = path.parent() {
    let () = create_dir_all(dir)
      .with_context(|| format!("failed to create directory `{}`", dir.display()))?;
  }

  // We only want to write out the config if any of the values changed.
  if current_result.as_ref() != Ok(config) {
    let Config {
      keyboard:
        keys::Config {
          auto_repeat_timeout_ms,
          auto_repeat_interval_ms,
        },
      game:
        game::Config {
          start_level,
          lines_for_level,
          field_width,
          field_height,
          preview_stone_count,
          enable_ai,
          enable_dark_mode,
        },
    } = config.clone();

    let keyboard = &mut doc["keyboard"];
    update!(keyboard, auto_repeat_timeout_ms, as int);
    update!(keyboard, auto_repeat_interval_ms, as int);

    let game = &mut doc["game"];
    update!(game, start_level, as int);
    update!(game, lines_for_level, as int);
    update!(game, field_width, as int);
    update!(game, field_height, as int);
    update!(game, preview_stone_count, as int);
    update!(game, enable_ai);
    update!(game, enable_dark_mode);

    let () = write(path, doc.to_string())?;
  }

  Ok(())
}


/// A type representing the configuration of the program.
#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Config {
  /// Settings pertaining keyboard handling.
  #[serde(default)]
  pub keyboard: keys::Config,
  /// Configuration of the game itself.
  #[serde(default)]
  pub game: game::Config,
}

#[cfg(not(target_arch = "wasm32"))]
impl Config {
  /// Load the configuration from its default path on the file system.
  pub fn load() -> Result<Self> {
    let path = default_config_path().context("failed to retrieve program config directory path")?;
    let config = load_config(&path)?;
    Ok(config)
  }

  /// Save the configuration to its default path on the file system.
  pub fn save(&self) -> Result<()> {
    let path = default_config_path().context("failed to retrieve program config directory path")?;
    let () = save_config(self, &path)?;
    Ok(())
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::fs;
  use std::thread::sleep;
  use std::time::Duration;

  use tempfile::NamedTempFile;

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

  /// Check that saving a [`Config`] does nothing if its fields didn't
  /// change.
  #[test]
  fn save_config_no_change() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path();

    let config = Config::default();
    let () = save_config(&config, path).unwrap();

    let before = fs::metadata(path).unwrap().modified().unwrap();
    // Sleep a tiny bit, because file systems may use coarse grained
    // time stamps.
    let () = sleep(Duration::from_millis(100));

    let loaded = load_config(path).unwrap();
    let () = save_config(&loaded, path).unwrap();

    let after = fs::metadata(path).unwrap().modified().unwrap();
    assert_eq!(before, after);
  }

  /// Verify that we write out a [`Config`] if it changed.
  #[test]
  fn save_config_with_change() {
    let file = NamedTempFile::new().unwrap();
    let path = file.path();

    let mut config = Config::default();
    let () = save_config(&config, path).unwrap();

    let before = fs::metadata(path).unwrap().modified().unwrap();
    let () = sleep(Duration::from_millis(100));

    let loaded = load_config(path).unwrap();
    assert_eq!(loaded, config);

    config.game.enable_ai = !config.game.enable_ai;
    let () = save_config(&config, path).unwrap();

    let after = fs::metadata(path).unwrap().modified().unwrap();
    assert!(after > before);

    let loaded = load_config(path).unwrap();
    assert_eq!(loaded, config);
  }
}
