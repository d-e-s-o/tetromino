// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;


pub fn smoothstep<T>(min: T, max: T, ratio: f32) -> T
where
  T: Copy + Sub<Output = T> + Mul<f32, Output = T> + Add<Output = T>,
{
  debug_assert!((0.0..=1.0).contains(&ratio), "{ratio}");
  let x = ratio * ratio * (3.0 - 2.0 * ratio);
  min + (max - min) * x
}
