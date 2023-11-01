// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::Ordering;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FmtResult;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::SubAssign;


#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub(super) struct Cost(f32);

impl Cost {
  /// Instantiate no or neutral cost.
  #[inline]
  pub fn none() -> Cost {
    Self(0.0)
  }

  /// Instantiate the maximum cost.
  #[inline]
  pub fn max() -> Cost {
    Self(f32::MAX)
  }
}

impl Display for Cost {
  #[inline]
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.0, f)
  }
}

impl From<f32> for Cost {
  #[inline]
  fn from(other: f32) -> Self {
    Cost(other)
  }
}

impl From<i16> for Cost {
  #[inline]
  fn from(other: i16) -> Self {
    Cost(f32::from(other))
  }
}

impl From<u8> for Cost {
  #[inline]
  fn from(other: u8) -> Self {
    Cost(f32::from(other))
  }
}

impl From<u16> for Cost {
  #[inline]
  fn from(other: u16) -> Self {
    Cost(f32::from(other))
  }
}

impl<T> Add<T> for Cost
where
  Cost: From<T>,
{
  type Output = Cost;

  #[inline]
  fn add(mut self, other: T) -> Self::Output {
    self.0 += Cost::from(other).0;
    self
  }
}

impl<T> AddAssign<T> for Cost
where
  Cost: From<T>,
{
  #[inline]
  fn add_assign(&mut self, other: T) {
    self.0 += Cost::from(other).0;
  }
}

impl<T> SubAssign<T> for Cost
where
  Cost: From<T>,
{
  #[inline]
  fn sub_assign(&mut self, other: T) {
    self.0 -= Cost::from(other).0;
  }
}

impl PartialEq for Cost {
  #[inline]
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl Eq for Cost {}

impl PartialOrd for Cost {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Cost {
  #[inline]
  fn cmp(&self, other: &Self) -> Ordering {
    self.0.total_cmp(&other.0)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::mem::size_of;


  /// Make sure that our [`Cost`] type is sufficiently small.
  #[test]
  fn cost_size() {
    assert_eq!(size_of::<Cost>(), 4);
  }


  /// Make sure some trivial comparisons between well-defined costs
  /// work as they should.
  #[test]
  fn comparisons_and_ops() {
    let mut small = Cost::none();
    let mut large = Cost::max();
    assert!(small < large);

    large -= 4u8;
    assert!(small < large);

    small += 1.5;
    assert!(small < large);

    small = small + small;
    assert!(small < large);
  }
}
