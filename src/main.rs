// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A graphical Tetris clone.

use std::env::args_os;

use anyhow::bail;
use anyhow::Result;

use tetromino_impl::run;
use tetromino_impl::Config;


fn default_config() -> String {
  // SANITY: Converting the config object to a string is expected to
  //         always succeed and any failure would be a bug.
  let config =
    toml::to_string_pretty(&Config::default()).expect("failed to serialize Tetromino config");
  let mut lines = config.lines();
  let indent = lines
    .next()
    .map(|first| lines.fold(format!("  {first}"), |s, line| format!("{s}\n  {line}")));
  indent.unwrap_or_default()
}


fn main() -> Result<()> {
  // By convention the 0th argument contains the path to the program;
  // ignore it.
  let args = || args_os().skip(1);

  // Slightly ghetto argument parser helps us avoid unnecessary large
  // dependency on `clap`.
  match args().len() {
    0 => run(),
    _ if args().any(|arg| arg == "--help" || arg == "-h") => {
      print!(
        "{name} {version} -- a graphical Tetris clone

USAGE:
  {name} [OPTIONS]

OPTIONS:
  -h, --help     Print help information
  -V, --version  Print version information

CONFIG:
  The game reads the $XDG_CONFIG_DIR/tetromino/config.toml TOML
  configuration file. The default configuration is:

  ```
{config}
  ```
",
        name = env!("CARGO_CRATE_NAME"),
        version = env!("VERSION"),
        config = default_config(),
      );
      Ok(())
    },
    _ if args().any(|arg| arg == "--version" || arg == "-V") => {
      println!("{} {}", env!("CARGO_CRATE_NAME"), env!("VERSION"));
      Ok(())
    },
    _ => {
      let arg = args().next().unwrap();
      bail!("unexpected argument '{}' found", &arg.to_string_lossy())
    },
  }
}
