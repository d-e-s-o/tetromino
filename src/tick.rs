// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::Ordering;

use crate::Instant;


/// An enumeration describing when the next program "tick" should occur.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tick {
  /// The next tick should happen immediately.
  Now,
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
      (Self::None, Self::At(_instant)) => Ordering::Greater,
      (Self::None, Self::Now) => Ordering::Greater,
      (Self::At(_instant), Self::None) => Ordering::Less,
      (Self::At(instant1), Self::At(instant2)) => instant1.cmp(instant2),
      (Self::At(_instant), Self::Now) => Ordering::Greater,
      (Self::Now, Self::None) => Ordering::Less,
      (Self::Now, Self::At(_instant)) => Ordering::Less,
      (Self::Now, Self::Now) => Ordering::Equal,
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::time::Duration;


  /// Check that [`Tick`] instances order properly.
  #[test]
  fn ordering() {
    let now = Instant::now();

    assert_eq!(Tick::None.cmp(&Tick::None), Ordering::Equal);
    assert_eq!(Tick::None.cmp(&Tick::Now), Ordering::Greater);
    assert_eq!(Tick::None.cmp(&Tick::At(now)), Ordering::Greater);

    assert_eq!(Tick::Now.cmp(&Tick::Now), Ordering::Equal);
    assert_eq!(Tick::Now.cmp(&Tick::None), Ordering::Less);
    assert_eq!(Tick::Now.cmp(&Tick::At(now)), Ordering::Less);

    assert_eq!(Tick::At(now).cmp(&Tick::None), Ordering::Less);
    assert_eq!(Tick::At(now).cmp(&Tick::Now), Ordering::Greater);

    let later = now + Duration::from_secs(1);
    assert_eq!(Tick::At(now).cmp(&Tick::At(now)), Ordering::Equal);
    assert_eq!(Tick::At(now).cmp(&Tick::At(later)), Ordering::Less);
    assert_eq!(Tick::At(later).cmp(&Tick::At(now)), Ordering::Greater);
  }
}
