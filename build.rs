// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Build script for `tetromino`.

use std::env;

use grev::git_revision_auto;


fn main() {
  let manifest_dir =
    env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR variable not set");
  let pkg_version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION variable not set");

  if let Some(git_rev) = git_revision_auto(manifest_dir).unwrap() {
    println!("cargo:rustc-env=VERSION={pkg_version} ({git_rev})");
  } else {
    println!("cargo:rustc-env=VERSION={pkg_version}");
  }
}
