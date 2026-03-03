// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg(target_arch = "wasm32")]


/// Redefinition of the standard `println` macro for WASM Web targets.
#[macro_export]
macro_rules! println {
  ($($t:tt)*) => {
    ::web_sys::console::log_1(&format!($($t)*).into())
  };
}

/// Redefinition of the standard `eprintln` macro for WASM Web targets.
#[macro_export]
macro_rules! eprintln {
  ($($t:tt)*) => {
    ::web_sys::console::error_1(&format!($($t)*).into())
  };
}

/// Redefinition of the standard `dbg` macro for WASM Web targets.
#[macro_export]
macro_rules! dbg {
  ($($t:tt)*) => {
    ::web_sys::console::log_1(&format!("{:?}", $($t)*).into())
  };
}
