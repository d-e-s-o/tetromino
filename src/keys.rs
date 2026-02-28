// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem::MaybeUninit;
use std::os::raw::c_uint;
use std::ptr::null;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::ensure;

use serde::Deserialize;
use serde::Serialize;

use x11_dl::xlib;

pub use keypeat::KeyRepeat;
pub use keypeat::Keys;


mod imp {
  pub(crate) use winit::keyboard::KeyCode as Key;

  pub(crate) const KEY_ROTATE_LEFT: Key = Key::Digit1;
  pub(crate) const KEY_ROTATE_RIGHT: Key = Key::Digit2;
  pub(crate) const KEY_MOVE_LEFT: Key = Key::KeyH;
  pub(crate) const KEY_MOVE_DOWN: Key = Key::KeyJ;
  pub(crate) const KEY_MOVE_RIGHT: Key = Key::KeyL;
  pub(crate) const KEY_DROP: Key = Key::Space;
  pub(crate) const KEY_RESTART: Key = Key::Backspace;
  pub(crate) const KEY_AUTO_PLAY: Key = Key::F2;
  pub(crate) const KEY_PAUSE: Key = Key::F3;
  pub(crate) const KEY_MODE: Key = Key::F4;
  #[cfg(feature = "debug")]
  pub(crate) const KEY_DEBUG: Key = Key::F11;
  pub(crate) const KEY_QUIT: Key = Key::KeyQ;
}

pub(crate) use imp::*;


#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Config {
  /// The auto-repeat timeout, in milliseconds.
  pub(crate) auto_repeat_timeout_ms: u32,
  /// The auto-repeat interval, in milliseconds.
  pub(crate) auto_repeat_interval_ms: u32,
}

impl Config {
  /// Instantiate a `Config` object using system defaults.
  pub(crate) fn with_system_defaults() -> Result<Self> {
    let mut timeout = MaybeUninit::<c_uint>::uninit();
    let mut interval = MaybeUninit::<c_uint>::uninit();
    // Value XkbUseCoreKbd constant defined somewhere in XKB.h and not
    // exported anywhere. Gorgeous!
    let xkb_use_core_kbd = 0x0100;

    let xlib = xlib::Xlib::open().context("failed to open xlib")?;
    // We could conceivably get the display passed in from our window or
    // some such, but the reality is that it's a royal mess to convert
    // one into the other.
    let display = unsafe { (xlib.XOpenDisplay)(null()) };
    ensure!(!display.is_null(), "failed to open X display");

    let result = unsafe {
      (xlib.XkbGetAutoRepeatRate)(
        display,
        xkb_use_core_kbd,
        timeout.as_mut_ptr(),
        interval.as_mut_ptr(),
      )
    };
    ensure!(result != 0, "failed to query keyboard auto repeat settings");

    let timeout = unsafe { timeout.assume_init() };
    let interval = unsafe { interval.assume_init() };

    let slf = Self {
      auto_repeat_timeout_ms: timeout,
      auto_repeat_interval_ms: interval,
    };
    Ok(slf)
  }
}

impl Default for Config {
  fn default() -> Self {
    match Self::with_system_defaults() {
      Ok(config) => config,
      Err(..) => {
        // We may not be able to query system settings in all cases. For
        // example, if no display is available we will likely fail.
        // However, for better user experience we still want to provide
        // somewhat sensible defaults. So just come up with something.
        Self {
          auto_repeat_timeout_ms: 100,
          auto_repeat_interval_ms: 50,
        }
      },
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Make sure that we can create a `Config` object using system
  /// defaults.
  #[test]
  fn config_instantiation() {
    match Config::with_system_defaults() {
      Ok(_config) => (),
      // In something like a CI environment we may not have an X
      // display. Let's just ignore those rare setups for the time
      // being...
      Err(err) if err.to_string() == "failed to open X display" => (),
      Err(err) => panic!("{}", err),
    }
  }
}
