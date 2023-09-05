// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Cursor;
use std::num::NonZeroU16;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;

use crate::ActiveRenderer as Renderer;
use crate::Point;
use crate::State;
use crate::Texture;

use super::data;
use super::Field;
use super::StoneFactory;

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
  /// The time of the next tick, i.e., the next downward movement.
  next_tick: Instant,
}

impl Game {
  /// Instantiate a new game of Tetris.
  pub(crate) fn new() -> Result<Self> {
    let reader = Cursor::new(data::TETRIS_FIELD_PIECE_TEXTURE);
    let piece = image::io::Reader::with_format(reader, image::ImageFormat::Png).decode()?;
    let piece = Texture::with_image(piece)?;

    let factory = Rc::new(StoneFactory::with_default_stones(piece.clone()));

    let reader = Cursor::new(data::TETRIS_FIELD_BACK_TEXTURE);
    let field_back = image::io::Reader::with_format(reader, image::ImageFormat::Png).decode()?;
    let field_back = Texture::with_image(field_back)?;
    // TODO: Make dimensions configurable.
    let field_width = 10;
    let field_height = 20;
    let field_location = Point::new(LEFT_SPACE, BOTTOM_SPACE);
    let field = Field::new(
      field_location,
      field_width,
      field_height,
      factory.clone(),
      piece,
      field_back,
    );

    let slf = Self {
      field,
      next_tick: Self::next_tick(Instant::now()),
    };
    Ok(slf)
  }

  /// Calculate the time of the next tick, given the current one.
  fn next_tick(current_tick: Instant) -> Instant {
    // TODO: Need to use actual level.
    const LEVEL: u16 = 10;

    // The current stone drop speed, in units per second.
    let units_per_sec = 1.0 + 0.2 * LEVEL as f32;
    current_tick + Duration::from_secs_f32(1.0 / units_per_sec)
  }

  /// Fast-forward the game to the current time.
  ///
  /// This includes moving the currently active stone according to the
  /// elapsed time since the last update.
  pub(crate) fn tick(&mut self, now: Instant) -> (State, Instant) {
    let mut state = State::Unchanged;

    while now >= self.next_tick {
      state |= self.field.move_stone_down();
      self.next_tick = Self::next_tick(self.next_tick);
    }

    (state, self.next_tick)
  }

  #[inline]
  pub(crate) fn on_move_down(&mut self) -> State {
    self.field.move_stone_down()
  }

  #[inline]
  pub(crate) fn on_drop(&mut self) -> State {
    self.field.drop_stone()
  }

  #[inline]
  pub(crate) fn on_move_left(&mut self) -> State {
    self.field.move_stone_left()
  }

  #[inline]
  pub(crate) fn on_move_right(&mut self) -> State {
    self.field.move_stone_right()
  }

  #[inline]
  pub(crate) fn on_rotate_left(&mut self) -> State {
    self.field.rotate_stone_left()
  }

  #[inline]
  pub(crate) fn on_rotate_right(&mut self) -> State {
    self.field.rotate_stone_right()
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
