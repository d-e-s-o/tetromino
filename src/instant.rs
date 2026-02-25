// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later


#[cfg(target_arch = "wasm32")]
mod imp {
  use std::cmp::Ordering;
  use std::ops::Add;
  use std::ops::AddAssign;
  use std::ops::Sub;
  use std::time::Duration;

  use wasm_bindgen::prelude::wasm_bindgen;


  #[wasm_bindgen]
  extern "C" {
    #[wasm_bindgen(js_namespace = performance)]
    fn now() -> f64;
  }

  /// A Wasm compatible `Instant` type, with an interface similar to
  /// [`std::time::Instant`].
  #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
  #[repr(transparent)]
  pub struct Instant {
    millis: f64,
  }

  impl Instant {
    /// Retrieve an instant corresponding to "now".
    #[inline]
    pub fn now() -> Self {
      Self { millis: now() }
    }

    /// Calculate the amount of time elapsed from another instant to
    /// this one.
    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
      *self - earlier
    }

    /// Retrieve the time stamp represented by this [`Instant`] as
    /// milliseconds.
    #[inline]
    pub(crate) fn as_millis(&self) -> f64 {
      self.millis
    }
  }

  impl Add<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn add(mut self, other: Duration) -> Self::Output {
      self += other;
      self
    }
  }

  impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, other: Duration) {
      self.millis += other.as_secs_f64() * 1000.0;
    }
  }

  impl Sub<Instant> for Instant {
    type Output = Duration;

    #[inline]
    fn sub(self, other: Instant) -> Self::Output {
      Duration::from_secs_f64((self.millis - other.millis) / 1000.0)
    }
  }

  impl Eq for Instant {}

  impl Ord for Instant {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
      self.millis.total_cmp(&other.millis)
    }
  }
}

#[cfg(not(target_arch = "wasm32"))]
#[expect(clippy::disallowed_types)]
mod imp {
  use std::time::Instant as StdInstant;

  /// Our `Instant` type.
  pub type Instant = StdInstant;
}

pub use imp::Instant;
