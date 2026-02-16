// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::Ordering;
use std::time::Instant;


/// An enumeration describing when the next program "tick" should occur.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tick {
  /// The next tick should happen at the given instant.
  At(Instant),
  /// No additional tick is necessary at this point.
  None,
}

impl From<Option<Instant>> for Tick {
  fn from(other: Option<Instant>) -> Self {
    match other {
      Some(instant) => Tick::At(instant),
      None => Tick::None,
    }
  }
}

impl PartialOrd<Tick> for Tick {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Tick {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::None, Self::None) => Ordering::Equal,
      (Self::At(_instant), Self::None) => Ordering::Less,
      (Self::None, Self::At(_instant)) => Ordering::Greater,
      (Self::At(instant1), Self::At(instant2)) => instant1.cmp(instant2),
    }
  }
}
