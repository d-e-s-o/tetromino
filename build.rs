// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Build script for `tetromino`.

use std::env;

use grev::git_revision_auto;

// OpenGL 1.3 is guaranteed to be available on Linux, so it's fine
// for us to use static bindings.
const OPENGL_MAJOR: u8 = 1;
const OPENGL_MINOR: u8 = 3;


fn main() {
  println!("cargo:rustc-env=OPENGL_MAJOR={OPENGL_MAJOR}");
  println!("cargo:rustc-env=OPENGL_MINOR={OPENGL_MINOR}");

  let manifest_dir =
    env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR variable not set");

  #[cfg(feature = "generate-opengl-bindings")]
  {
    use std::env;
    use std::fs::File;
    use std::path::Path;

    use gl_generator::Api;
    use gl_generator::Fallbacks;
    use gl_generator::Profile;
    use gl_generator::Registry;
    use gl_generator::StaticGenerator;

    let dst = Path::new(&manifest_dir)
      .join("src")
      .join("opengl")
      .join("bindings.rs");
    let mut file = File::create(dst).unwrap();

    Registry::new(
      Api::Gl,
      (OPENGL_MAJOR, OPENGL_MINOR),
      Profile::Core,
      Fallbacks::All,
      [],
    )
    .write_bindings(StaticGenerator, &mut file)
    .unwrap();
  }

  let pkg_version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION variable not set");

  if let Some(git_rev) = git_revision_auto(manifest_dir).unwrap() {
    println!("cargo:rustc-env=VERSION={pkg_version} ({git_rev})");
  } else {
    println!("cargo:rustc-env=VERSION={pkg_version}");
  }
}
