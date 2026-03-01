// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! Build script for `tetromino`.

use std::env;
use std::path::Path;
#[cfg(feature = "deploy")]
use std::path::PathBuf;

use grev::git_revision_auto;


/// Generate the final WASM bindings package in `output_dir` based on
/// the provided .wasm file supplied as `input_wasm`.
#[cfg(feature = "deploy")]
fn generate_pkg(input_wasm: &Path, output_dir: &Path, debug: bool) {
  use wasm_bindgen_cli_support::Bindgen;

  // This invocation roughly maps to the following command (which would
  // require wasm-bindgen-cli installed):
  // $ wasm-bindgen --out-dir <output_dir> --target web <input_wasm>
  Bindgen::new()
    .input_path(input_wasm)
    .web(true)
    .unwrap()
    .browser(false)
    .unwrap()
    .debug(debug)
    .keep_debug(debug)
    .remove_name_section(!debug)
    .remove_producers_section(!debug)
    .typescript(false)
    .generate(output_dir)
    .expect("failed to wasm-bindgen")
}


#[cfg(feature = "deploy")]
fn deploy_package(manifest_dir: &Path) {
  let name = env::var("CARGO_PKG_NAME")
    .expect("failed to read CARGO_PKG_NAME variable")
    .replace("-", "_");
  // Cargo's OUT_DIR is where it stores build artifacts and so that is
  // where we can find the created .*wasm we need.
  let out_dir = env::var_os("OUT_DIR").expect("failed to read `OUT_DIR` variable");

  // OUT_DIR is something like
  // <some-dir>/target/<target>/debug/build/<name>-<hash>/out
  // but the generated .wasm resides in
  // <some-dir>/target/<target>/debug/deps/
  // directly. So pop the last three directories.
  let mut input_wasm = PathBuf::from(out_dir);
  input_wasm.pop();
  input_wasm.pop();
  input_wasm.pop();
  input_wasm.push("deps");
  input_wasm.push(name);
  input_wasm.set_extension("wasm");

  let mut output_dir = manifest_dir.to_path_buf();
  output_dir.push("www");
  output_dir.push("pkg");

  let debug = env::var("DEBUG").expect("failed to read DEBUG variable");
  let debug = match debug.as_ref() {
    "true" => true,
    "false" => false,
    _ => {
      panic!("encountered unexpected value in DEBUG variable: {debug}")
    },
  };

  generate_pkg(&input_wasm, &output_dir, debug);

  println!("cargo:rerun-if-changed={}", input_wasm.as_path().display());
}


#[cfg(not(feature = "deploy"))]
fn deploy_package(_manifest_dir: &Path) {
  unimplemented!()
}


fn main() {
  let manifest_dir =
    env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR variable not set");
  let pkg_version = env::var("CARGO_PKG_VERSION").expect("CARGO_PKG_VERSION variable not set");

  if let Some(git_rev) = git_revision_auto(&manifest_dir).unwrap() {
    println!("cargo:rustc-env=VERSION={pkg_version} ({git_rev})");
  } else {
    println!("cargo:rustc-env=VERSION={pkg_version}");
  }

  if cfg!(feature = "deploy") {
    let () = deploy_package(Path::new(&manifest_dir));
  }

  println!("cargo:rerun-if-env-changed=CARGO_FEATURE_RUN");
  println!("cargo:rerun-if-env-changed=CARGO_MANIFEST_DIR");
  println!("cargo:rerun-if-env-changed=CARGO_PKG_NAME");
  println!("cargo:rerun-if-env-changed=OUT_DIR");
}
