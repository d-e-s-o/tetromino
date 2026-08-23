// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Add;
use std::ops::Sub;


const SRGB_GAMMA: f32 = 2.2;

pub(crate) const GLSL_LINEAR_TO_SRGB: &str = r#"
vec3 linear_to_srgb(vec3 color) {
  return pow(color, vec3(1.0 / 2.2));
}
"#;


#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C, packed)]
pub(crate) struct Color {
  pub(crate) r: u8,
  pub(crate) g: u8,
  pub(crate) b: u8,
  pub(crate) a: u8,
}

impl Color {
  /// A `const` version of `Add::add`.
  const fn cadd(self, other: Color) -> Self {
    Self {
      r: self.r.saturating_add(other.r),
      g: self.g.saturating_add(other.g),
      b: self.b.saturating_add(other.b),
      a: self.a.saturating_add(other.a),
    }
  }

  /// A `const` version of `Sub::sub`.
  pub const fn csub(self, other: Color) -> Self {
    Self {
      r: self.r.saturating_sub(other.r),
      g: self.g.saturating_sub(other.g),
      b: self.b.saturating_sub(other.b),
      a: self.a.saturating_sub(other.a),
    }
  }


  #[inline]
  pub(crate) const fn black() -> Self {
    Self {
      r: u8::MIN,
      g: u8::MIN,
      b: u8::MIN,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn white() -> Self {
    Self {
      r: u8::MAX,
      g: u8::MAX,
      b: u8::MAX,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn red() -> Self {
    Self {
      r: u8::MAX,
      g: u8::MIN,
      b: u8::MIN,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn green() -> Self {
    Self {
      r: u8::MIN,
      g: u8::MAX,
      b: u8::MIN,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn blue() -> Self {
    Self {
      r: u8::MIN,
      g: u8::MIN,
      b: u8::MAX,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn yellow() -> Self {
    Self::red().cadd(Self::green())
  }

  #[inline]
  pub(crate) const fn violet() -> Self {
    Self::red().cadd(Self::blue())
  }

  #[inline]
  pub(crate) const fn cyan() -> Self {
    Self::green().cadd(Self::blue())
  }

  #[inline]
  pub(crate) const fn orange() -> Self {
    Self {
      r: u8::MAX,
      g: u8::MAX / 4,
      b: u8::MIN,
      a: u8::MAX,
    }
  }

  #[inline]
  pub(crate) const fn gray() -> Self {
    Self {
      r: u8::MAX / 4,
      g: u8::MAX / 4,
      b: u8::MAX / 4,
      a: u8::MAX,
    }
  }
}

impl Add<Color> for Color {
  type Output = Color;

  fn add(self, other: Color) -> Self::Output {
    self.cadd(other)
  }
}

impl Sub<Color> for Color {
  type Output = Color;

  fn sub(self, other: Color) -> Self::Output {
    self.csub(other)
  }
}


pub(crate) trait ColorExt {
  /// Encode the color (assumed to be in sRGB space) into linear color
  /// space.
  fn to_linear(self) -> Self;
}

impl ColorExt for (f32, f32, f32) {
  fn to_linear(self) -> Self {
    let (r, g, b) = self;

    (r.powf(SRGB_GAMMA), g.powf(SRGB_GAMMA), b.powf(SRGB_GAMMA))
  }
}
