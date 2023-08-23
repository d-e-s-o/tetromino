// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

// Permuted congruential generator implementation, based on
// https://en.wikipedia.org/w/index.php?title=Permuted_congruential_generator&oldid=1167029503#Example_code

use std::cell::Cell;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

const MULTIPLIER: u64 = 6364136223846793005;
const INCREMENT: u64 = 1442695040888963407;


#[derive(Debug)]
pub(crate) struct Rng {
  state: Cell<u64>,
}

impl Rng {
  pub(crate) fn new() -> Self {
    // SANITY: `UNIX_EPOCH` is earlier than *any* other `SystemTime`.
    let seed = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos() as u64;

    Self::with_seed(seed)
  }

  pub(crate) fn with_seed(seed: u64) -> Self {
    let state = seed.wrapping_add(INCREMENT);

    Self {
      state: Cell::new(state),
    }
  }

  /// Generate a new pseudo random `u32` value.
  pub(crate) fn rand_u32(&self) -> u32 {
    fn rotr32(x: u32, r: usize) -> u32 {
      x >> r | x << (!r & 31)
    }

    let mut x = self.state.get();
    let count = (x >> 59) as usize;

    let () = self
      .state
      .set(x.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT));

    x ^= x >> 18;
    rotr32((x >> 27) as u32, count)
  }
}


#[cfg(test)]
mod tests {
  use std::collections::HashSet;

  use super::*;


  /// Check that our `Rng` is reasonably random.
  #[test]
  fn rng_randomness() {
    let rng = Rng::new();
    let set = (0..10).map(|_| rng.rand_u32()).collect::<HashSet<u32>>();

    // In a draw of ten "random" u32 numbers, we expect to have more
    // than 5 unique ones, which should be an *extremely* conservative
    // assertion.
    assert!(set.len() > 5, "{set:#?}");
  }

  /// Check that different `Rng` instances using the same seed yield the
  /// same sequence of random numbers.
  #[test]
  fn rng_seed_dependence() {
    let rng1 = Rng::with_seed(42);
    let rng2 = Rng::with_seed(42);

    (0..10).for_each(|_| {
      assert_eq!(rng1.rand_u32(), rng2.rand_u32());
    })
  }
}
