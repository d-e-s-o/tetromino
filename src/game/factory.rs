// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::Color;
use crate::Point;
use crate::Rng;
use crate::Texture;

use super::Stone;


type StoneTemplate = Vec<Point<u8>>;


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

  /// Create a new random stone, based on one of the known templates.
  pub(crate) fn create_stone(&self) -> Stone {
    let index = self.rng.rand_u32() as usize % self.templates.len();
    let template = &self.templates[index];
    let color = COLORS[index % COLORS.len()];

    Stone::new(self.piece_texture.clone(), template, color)
  }
}
