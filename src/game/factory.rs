// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::max;
use std::cmp::min;

use crate::Color;
use crate::Point;
use crate::Rng;
use crate::Texture;

use super::Stone;
use super::StoneProducer;


type StoneTemplate = Vec<Point<i8>>;


/// The set of colors we use for stones.
const COLORS: &[Color] = &[
  Color::red(),
  Color::green(),
  Color::yellow(),
  Color::violet(),
  Color::blue(),
  Color::cyan(),
  Color::gray(),
];


#[derive(Debug)]
pub(super) struct StoneFactory {
  /// The texture to use for each piece.
  piece_texture: Texture,
  /// The known stone types.
  templates: Vec<StoneTemplate>,
  /// The random number generator we use when creating new stones.
  rng: Rng,
}

impl StoneFactory {
  pub(super) fn with_default_stones(piece_texture: Texture) -> Self {
    #[rustfmt::skip]
    let templates = vec![
      vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1), Point::new(0, 1)], // O
      vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1), Point::new(2, 1)], // S
      vec![Point::new(0, 1), Point::new(1, 1), Point::new(1, 0), Point::new(2, 0)], // Z
      vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)], // I
      vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1), Point::new(2, 0)], // T
      vec![Point::new(0, 0), Point::new(1, 0), Point::new(0, 1), Point::new(0, 2)], // J
      vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1), Point::new(1, 2)], // L
    ];

    Self {
      templates,
      piece_texture,
      rng: Rng::new(),
    }
  }
}

impl StoneProducer for StoneFactory {
  /// Create a new random stone, based on one of the known templates.
  fn create_stone(&self) -> Stone {
    let index = self.rng.rand_u32() as usize % self.templates.len();
    let template = &self.templates[index];
    let color = COLORS[index % COLORS.len()];

    Stone::new(self.piece_texture.clone(), template, color)
  }

  // TODO: Loose copy of logic from `Stone`. Should think about deduplicating.
  fn max_dimensions(&self) -> (i16, i16) {
    let mut locations = self.templates.iter().flatten();
    // SANITY: Our stone always has at least one template.
    let location = locations.next().unwrap();
    let mut x_min = location.x;
    let mut x_max = location.x;
    let mut y_min = location.y;
    let mut y_max = location.y;

    for location in locations.skip(1) {
      x_min = min(x_min, location.x);
      x_max = max(x_max, location.x);
      y_min = min(y_min, location.y);
      y_max = max(y_max, location.y);
    }

    let w = x_max + 1 - x_min;
    let h = y_max + 1 - y_min;
    (w.into(), h.into())
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Check that we report the correct upper bound dimensions for the
  /// default set of stones.
  #[test]
  fn stone_dimensions() {
    let texture = Texture::invalid();
    let factory = StoneFactory::with_default_stones(texture);

    let (w, h) = factory.max_dimensions();
    assert_eq!(w, 3);
    assert_eq!(h, 4);
  }
}
