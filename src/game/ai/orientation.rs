// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem::transmute;
use std::num::NonZeroU8;
use std::ops::BitOr;
use std::ops::BitOrAssign;


#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum Orientation {
  // Note: Values are assigned in such a way that bit rotation direction
  //       matches logical rotation direction.
  Rotated0 = 0b1000,
  Rotated90 = 0b0100,
  Rotated180 = 0b0010,
  Rotated270 = 0b0001,
}

impl Orientation {
  #[inline]
  pub fn rotate_left(self) -> Self {
    let value = rotate_left::<4>(self as usize, 1) as u8;
    debug_assert!(0 < value && value <= 8, "{value:b}");
    // SAFETY: `Self` is `repr(u8)` and assuming our rotation code is
    //         not buggy this "cast" is fine.
    unsafe { transmute::<u8, Self>(value) }
  }

  #[inline]
  pub fn rotate_right(self) -> Self {
    let value = rotate_right::<4>(self as usize, 1) as u8;
    debug_assert!(0 < value && value <= 8, "{value:b}");
    // SAFETY: `Self` is `repr(u8)` and assuming our rotation code is
    //         not buggy this "cast" is fine.
    unsafe { transmute::<u8, Self>(value) }
  }
}


// We use `NonZeroU8` here to keep size to a minimum. However, that
// comes at the expense of us having to bit shift stuff a bit to
// actually never use zero. That could be simplified if Rust ever gets a
// `NonMaxU8` or stabilizes niche annotations or so.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub(super) struct Orientations(NonZeroU8);

impl Orientations {
  #[inline]
  pub fn contains(&self, orientation: Orientation) -> bool {
    (self.0.get() >> 1 & orientation as u8) != 0
  }
}

impl BitOr<Orientation> for Orientations {
  type Output = Orientations;

  fn bitor(self, other: Orientation) -> Self::Output {
    let mut result = self;
    result |= other;
    result
  }
}

impl BitOrAssign<Orientation> for Orientations {
  fn bitor_assign(&mut self, other: Orientation) {
    self.0 |= (other as u8) << 1;
  }
}

impl Default for Orientations {
  fn default() -> Self {
    // SAFETY: 1 != 0.
    Self(unsafe { NonZeroU8::new_unchecked(1) })
  }
}


#[inline]
const fn mask<const BITS: usize>() -> usize {
  //
  // Calculation of the mask for the given number of bits by example
  // BITS = 5 => goal is mask 011111b
  // - BITS - 1 => 4
  // - 1 << 4   => 16 = 010000b
  // - 16 - 1   => 15 = 001111b
  // - 15 << 1  => 30 = 011110b
  // - 30 | 1   => 31 = 011111b
  // Note: 'BITS' might equal the number of bits of the given data type;
  //       this means we cannot safely assume that we can always perform
  //       'Bit' shifts, so the algorithm is slightly more complicated
  //       and we only shift by 'Bit'-1 initially.
  ((1usize << (BITS - 1)).wrapping_sub(1) << 1) | 1
}

#[inline]
const fn rotate_left<const BITS: usize>(value: usize, shift: usize) -> usize {
  ((value << shift) | (value >> (BITS - shift))) & mask::<BITS>()
}

#[inline]
const fn rotate_right<const BITS: usize>(value: usize, shift: usize) -> usize {
  ((value >> shift) | (value << (BITS - shift))) & mask::<BITS>()
}


#[cfg(test)]
mod tests {
  use std::mem::size_of;

  use super::*;


  /// Make sure that our [`Orientation`] type is sufficiently small and
  /// contains at least one niche bit.
  #[test]
  fn orientation_size() {
    assert_eq!(size_of::<Option<Orientation>>(), 1);
  }

  /// Make sure that our [`Orientations`] type is sufficiently small and
  /// contains at least one niche bit.
  #[test]
  fn orientations_size() {
    assert_eq!(size_of::<Option<Orientations>>(), 1);
  }

  /// Test that we can create a bit mask correctly.
  #[test]
  fn raw_mask_calculation() {
    assert_eq!(mask::<1>(), 0b0001);
    assert_eq!(mask::<2>(), 0b0011);
    assert_eq!(mask::<3>(), 0b0111);
    assert_eq!(mask::<4>(), 0b1111);
  }

  /// Check that we can correctly rotate bits left in a non-standard
  /// size integer.
  #[test]
  fn raw_left_rotation() {
    assert_eq!(rotate_left::<32>(1, 32), 1);
    assert_eq!(rotate_left::<32>(2, 32), 2);
    assert_eq!(rotate_left::<20>(1, 20), 1);
    assert_eq!(rotate_left::<20>(2, 20), 2);

    assert_eq!(rotate_left::<20>(0, 1), 0);
    assert_eq!(rotate_left::<20>(0, 2), 0);

    assert_eq!(rotate_left::<20>(7, 2), 28);
    assert_eq!(rotate_left::<20>(7, 2), 28);
    assert_eq!(rotate_left::<20>(1 << 19, 1), 1);
  }

  /// Check that we can correctly rotate bits right in a non-standard
  /// size integer.
  #[test]
  fn raw_right_rotation() {
    assert_eq!(rotate_right::<32>(1, 32), 1);
    assert_eq!(rotate_right::<32>(2, 32), 2);

    assert_eq!(rotate_right::<19>(1, 1), 1 << 18);
    assert_eq!(rotate_right::<19>(1, 2), 1 << 17);
  }

  /// Make sure that our [`Orientation`] enum behaves as intended.
  #[test]
  fn orientation_rotation() {
    assert_eq!(Orientation::Rotated0.rotate_left(), Orientation::Rotated270);
    assert_eq!(Orientation::Rotated0.rotate_right(), Orientation::Rotated90);
    assert_eq!(Orientation::Rotated90.rotate_left(), Orientation::Rotated0);
    assert_eq!(
      Orientation::Rotated90.rotate_right(),
      Orientation::Rotated180
    );
    assert_eq!(
      Orientation::Rotated180.rotate_left(),
      Orientation::Rotated90
    );
    assert_eq!(
      Orientation::Rotated180.rotate_right(),
      Orientation::Rotated270
    );
    assert_eq!(
      Orientation::Rotated270.rotate_left(),
      Orientation::Rotated180
    );
    assert_eq!(
      Orientation::Rotated270.rotate_right(),
      Orientation::Rotated0
    );
  }

  /// Check that we can combined [`Orientation`] objects into
  /// [`Orientations`] as expected.
  #[test]
  fn orientation_combination() {
    let mut orientations = Orientations::default();
    assert!(!orientations.contains(Orientation::Rotated0));
    assert!(!orientations.contains(Orientation::Rotated90));
    assert!(!orientations.contains(Orientation::Rotated180));
    assert!(!orientations.contains(Orientation::Rotated270));

    orientations |= Orientation::Rotated90;
    orientations |= Orientation::Rotated90;
    orientations |= Orientation::Rotated90;
    assert!(!orientations.contains(Orientation::Rotated0));
    assert!(orientations.contains(Orientation::Rotated90));
    assert!(!orientations.contains(Orientation::Rotated180));
    assert!(!orientations.contains(Orientation::Rotated270));

    orientations |= Orientation::Rotated180;
    assert!(!orientations.contains(Orientation::Rotated0));
    assert!(orientations.contains(Orientation::Rotated90));
    assert!(orientations.contains(Orientation::Rotated180));
    assert!(!orientations.contains(Orientation::Rotated270));

    orientations |= Orientation::Rotated0;
    assert!(orientations.contains(Orientation::Rotated0));
    assert!(orientations.contains(Orientation::Rotated90));
    assert!(orientations.contains(Orientation::Rotated180));
    assert!(!orientations.contains(Orientation::Rotated270));

    orientations |= Orientation::Rotated270;
    assert!(orientations.contains(Orientation::Rotated0));
    assert!(orientations.contains(Orientation::Rotated90));
    assert!(orientations.contains(Orientation::Rotated180));
    assert!(orientations.contains(Orientation::Rotated270));
  }
}
