// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use grev::git_revision_auto;


fn main() {
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

    // OpenGL 1.3 is guaranteed to be available on Linux, so it's fine
    // for us to use static bindings.
    Registry::new(Api::Gl, (1, 3), Profile::Core, Fallbacks::All, [])
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
