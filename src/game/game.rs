// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Cursor;
use std::num::NonZeroU16;

use anyhow::Result;

use crate::ActiveRenderer as Renderer;
use crate::Point;
use crate::Texture;

use super::data;
use super::Field;

/// Space between the left screen side and the field.
const LEFT_SPACE: u16 = 1;
/// Space between the bottom of the screen and the field.
const BOTTOM_SPACE: u16 = 1;
/// Space between the right side of the screen and the preview stones.
const RIGHT_SPACE: u16 = 1;
/// Space between the upper screen side and the field.
const TOP_SPACE: u16 = 1;


/// A type representing a game of Tetris.
pub(crate) struct Game {
  /// The Tetris field.
  field: Field,
}

impl Game {
  /// Instantiate a new game of Tetris.
  pub(crate) fn new() -> Result<Self> {
    let reader = Cursor::new(data::TETRIS_FIELD_PIECE_TEXTURE);
    let piece = image::io::Reader::with_format(reader, image::ImageFormat::Png).decode()?;
    let piece = Texture::with_image(piece)?;

    let reader = Cursor::new(data::TETRIS_FIELD_BACK_TEXTURE);
    let field_back = image::io::Reader::with_format(reader, image::ImageFormat::Png).decode()?;
    let field_back = Texture::with_image(field_back)?;
    // TODO: Make dimensions configurable.
    let field_width = 10;
    let field_height = 20;
    let field_location = Point::new(LEFT_SPACE, BOTTOM_SPACE);

    let slf = Self {
      field: Field::new(field_location, field_width, field_height, piece, field_back),
    };
    Ok(slf)
  }

  /// Render the game and its components.
  pub(crate) fn render(&self, renderer: &Renderer) {
    let () = self.field.render(renderer);
  }

  /// Retrieve the game surface's width.
  pub(crate) fn width(&self) -> NonZeroU16 {
    // SAFETY: The provided height is guaranteed to be greater than zero.
    unsafe { NonZeroU16::new_unchecked(LEFT_SPACE + self.field.width() + RIGHT_SPACE) }
  }

  /// Retrieve the game surface's height.
  pub(crate) fn height(&self) -> NonZeroU16 {
    // SAFETY: The provided height is guaranteed to be greater than zero.
    unsafe { NonZeroU16::new_unchecked(BOTTOM_SPACE + self.field.height() + TOP_SPACE) }
  }
}