// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use grev::git_revision_auto;

// OpenGL 1.3 is guaranteed to be available on Linux, so it's fine
// for us to use static bindings.
const OPENGL_MAJOR: u8 = 1;
const OPENGL_MINOR: u8 = 3;


fn main() {
  println!("cargo:rustc-env=OPENGL_MAJOR={OPENGL_MAJOR}");
  println!("cargo:rustc-env=OPENGL_MINOR={OPENGL_MINOR}");

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

    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let dst = Path::new(&crate_dir)
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

  let dir = env!("CARGO_MANIFEST_DIR");
  if let Some(git_rev) = git_revision_auto(dir).unwrap() {
    println!(
      "cargo:rustc-env=VERSION={} (@ {})",
      env!("CARGO_PKG_VERSION"),
      git_rev
    );
  } else {
    println!("cargo:rustc-env=VERSION={}", env!("CARGO_PKG_VERSION"));
  }
}
