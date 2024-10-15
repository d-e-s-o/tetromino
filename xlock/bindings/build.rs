// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(clippy::let_unit_value)]

use std::borrow::Cow;
use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::ops::Deref as _;
use std::path::Path;

const XLOCK_SRC_ARCHIVE_URL: &str = "XLOCK_SRC_ARCHIVE_URL";
#[cfg(feature = "generate-xlock-bindings")]
const XLOCK_SRC_ROOT: &str = "XLOCK_SRC_ROOT";
const XLOCK_VERSION: &str = "XLOCK_VERSION";


/// Unpack an xz compressed file.
#[cfg(all(feature = "tar", feature = "xz2"))]
fn unpack_tar_xz(src_file: &File, dst: &Path) {
  use std::fs::create_dir_all;
  use tar::Archive;
  use xz2::read::XzDecoder;

  println!("cargo:rerun-if-changed={}", dst.display());

  let () = create_dir_all(dst).unwrap();

  let decoder = XzDecoder::new_multi_decoder(src_file);
  let mut extracter = Archive::new(decoder);
  let () = extracter.set_overwrite(true);
  let () = extracter.unpack(dst).unwrap();
}

#[cfg(not(all(feature = "tar", feature = "xz2")))]
fn unpack_tar_xz(_src_file: &File, _dst: &Path) {
  unimplemented!()
}


#[cfg(feature = "reqwest")]
fn download_file(url: &str) -> Option<File> {
  use std::io::Seek as _;
  use std::io::Write as _;
  use std::time::Duration;

  use reqwest::blocking::Client;
  use reqwest::StatusCode;
  use reqwest::Url;
  use tempfile::tempfile;

  let mut dst = tempfile().expect("failed to create temporary file");
  let client = Client::builder()
    // Some servers are braindead and report nonsensical HTTP errors
    // that mask otherwise useful ones if no user agent is provided.
    .user_agent("reqwest")
    // Only have a connect timeout here, because the "normal" timeout
    // seems to guard the *entire* transfer?!
    .timeout(None)
    .connect_timeout(Duration::from_secs(60))
    .build()
    .unwrap();
  let result = client
    .get(Url::parse(url).unwrap())
    .send()
    .unwrap()
    .error_for_status();
  match result {
    Ok(response) => {
      let _count = dst.write(&response.bytes().unwrap()).unwrap();
      let () = dst.rewind().unwrap();
      Some(dst)
    },
    Err(err) if err.status() == Some(StatusCode::NOT_FOUND) => None,
    Err(err) => panic!("failed to download {url}: {err}"),
  }
}

#[cfg(not(feature = "reqwest"))]
fn download_file(_url: &str) -> Option<File> {
  unimplemented!()
}


/// Download and extract the xlock source code.
fn download_xlock_source(xlock_src: &Path) {
  let url1;
  let url2;
  let url3;
  let urls1;
  let urls2;

  println!("cargo:rerun-if-env-changed={}", XLOCK_SRC_ARCHIVE_URL);
  let url = env::var_os(XLOCK_SRC_ARCHIVE_URL);
  let urls = if let Some(url) = &url {
    url1 = url
      .to_str()
      .unwrap_or_else(|| panic!("{XLOCK_SRC_ARCHIVE_URL} variable does not contain valid Unicode"));
    urls1 = [url1];
    urls1.as_slice()
  } else {
    println!("cargo:rerun-if-env-changed={}", XLOCK_VERSION);

    let version = env::var_os(XLOCK_VERSION)
      .map(Cow::from)
      .unwrap_or_else(|| Cow::Borrowed(OsStr::new("5.77")));
    let version = version
      .to_str()
      .unwrap_or_else(|| panic!("{XLOCK_VERSION} variable does not contain valid Unicode"));
    // We cannot know in advance whether this version is *a*
    // recent release or *the* recent release. So we have two URL
    // candidates that we need to try. Sigh.
    url2 = format!("https://www.sillycycle.com/xlock/xlockmore-{version}.tar.xz");
    url3 = format!("https://www.sillycycle.com/xlock/recent-releases/xlockmore-{version}.tar.xz");
    urls2 = [url2.deref(), url3.deref()];
    urls2.as_slice()
  };

  let mut downloaded = false;

  for url in urls {
    if let Some(file) = download_file(url) {
      let () = unpack_tar_xz(&file, xlock_src);
      downloaded = true;
      break
    }
  }

  if !downloaded {
    panic!("failed to download xlock source code from: {urls:?}");
  }
}


fn main() {
  let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
  let xlock_src = crate_dir.join("xlock-src");

  if cfg!(feature = "download-xlock-source") {
    let () = download_xlock_source(&xlock_src);
  }

  #[cfg(feature = "generate-xlock-bindings")]
  {
    use std::fs::read_dir;
    use std::path::PathBuf;

    println!("cargo:rerun-if-env-changed={}", XLOCK_SRC_ROOT);

    let xlock_src_root = if let Some(xlock_src_root) = env::var_os(XLOCK_SRC_ROOT) {
      PathBuf::from(xlock_src_root)
    } else {
      println!("cargo:rerun-if-changed={}", xlock_src.display());
      // Check if the xlock-src/ is present. If it is, find the
      // lexicographically last item in there. Assuming one or more
      // entries of the form xlockmore-5.32, xlockmore-5.73, ..., we
      // shall end up picking the one with the highest version number.
      let mut entries = read_dir(&xlock_src)
        .unwrap_or_else(|err| {
          panic!(
            "{XLOCK_SRC_ROOT} environment variable not found and failed \
             to read {}: {err}; cannot continue",
            xlock_src.display(),
          )
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

      let () = entries.sort_by_key(|entry| entry.file_name());
      let xlock_src_root = entries.last().map(|entry| entry.path()).unwrap_or_else(|| {
        panic!(
          "no xlock source candidates found in {}",
          xlock_src.display()
        )
      });
      xlock_src_root
    };

    let xlock_src_root = xlock_src_root.canonicalize().unwrap();
    println!(
      "cargo:warning=using xlock sources at {}",
      xlock_src_root.display()
    );

    let xlock_h = xlock_src_root.join("xlock").join("xlock.h");
    println!("cargo:rerun-if-changed={}", xlock_h.display());

    let () = bindgen::Builder::default()
      .clang_args(["-D", "USE_MODULES"])
      .header(xlock_h.to_str().unwrap())
      .derive_copy(false)
      .allowlist_type("LockStruct")
      .allowlist_type("ModeInfo")
      .allowlist_type("ModStruct")
      .layout_tests(false)
      .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
      .generate()
      .expect("failed to generate bindings")
      .write_to_file(crate_dir.join("bindings.rs"))
      .expect("failed to write bindings");
  }
}
