// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::marker::PhantomData;


/// A type for objects that run a function when dropped.
pub(crate) struct Guard<'guardee, F>
where
  F: FnOnce(),
{
  /// The function to run on "drop".
  drop_fn: Option<F>,
  /// Phantom data
  _phantom: PhantomData<&'guardee ()>,
}

impl<F> Guard<'_, F>
where
  F: FnOnce(),
{
  pub fn new(drop_fn: F) -> Self {
    Self {
      drop_fn: Some(drop_fn),
      _phantom: PhantomData,
    }
  }
}

impl<F> Drop for Guard<'_, F>
where
  F: FnOnce(),
{
  fn drop(&mut self) {
    if let Some(drop_fn) = self.drop_fn.take() {
      (drop_fn)()
    }
  }
}
