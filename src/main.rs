// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::env::args_os;

use anyhow::bail;
use anyhow::Result;

use tetromino::run;


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
",
        name = env!("CARGO_CRATE_NAME"),
        version = env!("VERSION"),
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
