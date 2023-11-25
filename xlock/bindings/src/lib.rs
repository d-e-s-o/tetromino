// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/bindings.rs"));

// Both types may reasonably be used for static variables where they
// have to be `Sync`.
unsafe impl Sync for ModeSpecOpt {}
unsafe impl Sync for ModStruct {}
