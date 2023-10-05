// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Cursor;
use std::num::NonZeroU16;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;

use crate::ActiveRenderer as Renderer;
use crate::Change;
use crate::Font;
use crate::Point;
use crate::Texture;
use crate::Tick;

use super::data;
use super::field::State;
use super::Config;
use super::Field;
use super::MoveResult;
use super::PreviewStones;
use super::Score;
use super::StoneFactory;

/// Space between the left screen side and the field.
const LEFT_SPACE: i16 = 1;
/// Space between the bottom of the screen and the field.
const BOTTOM_SPACE: i16 = 1;
/// Space between the right side of the screen and the preview stones.
const RIGHT_SPACE: i16 = 1;
/// Space between the upper screen side and the field.
const TOP_SPACE: i16 = 1;
/// Space between the field and the preview stones.
const PREVIEW_FIELD_SPACE: i16 = 1;
/// Space between the preview stones and the score board.
const PREVIEW_SCORE_SPACE: i16 = 1;
/// The time for which we highlight any completed lines while not
/// responding to any input.
// TODO: Make configurable.
const CLEAR_TIME: Duration = Duration::from_millis(200);


/// A type representing a game of Tetris.
pub(crate) struct Game {
  /// The Tetris field.
  field: Field,
  /// The preview stones.
  preview: Rc<PreviewStones>,
  /// The current score.
  score: Score,
  /// The time of the next tick, i.e., the next downward movement.
  ///
  /// The attribute is `None` if the game is not running (e.g., paused).
  next_tick: Option<Instant>,
}

impl Game {
  /// Instantiate a new game of Tetris with the given configuration.
  pub(crate) fn with_config(config: &Config) -> Result<Self> {
    let reader = Cursor::new(data::TETRIS_FIELD_PIECE_TEXTURE);
    let piece = image::io::Reader::with_format(reader, image::ImageFormat::Png).decode()?;
    let piece = Texture::with_image(piece)?;

    let factory = Rc::new(StoneFactory::with_default_stones(piece.clone()));

    let field_location = Point::new(LEFT_SPACE, BOTTOM_SPACE);
    let preview_location = field_location
      + Point::new(
        Field::total_width(config.field_width),
        Field::total_height(config.field_height),
      )
      + Point::new(RIGHT_SPACE, 0);
    let preview = Rc::new(PreviewStones::new(
      preview_location,
      config.preview_stone_count,
      factory,
    ));

    let reader = Cursor::new(data::TETRIS_FIELD_BACK_TEXTURE);
    let field_back = image::io::Reader::with_format(reader, image::ImageFormat::Png).decode()?;
    let field_back = Texture::with_image(field_back)?;
    let field = Field::new(
      field_location,
      config.field_width,
      config.field_height,
      CLEAR_TIME,
      preview.clone(),
      piece.clone(),
      field_back,
    );

    let font = Font::builtin(piece);
    let score_location = preview_location - Point::new(0, preview.height() + PREVIEW_SCORE_SPACE);
    let score = Score::new(
      score_location,
      config.start_level,
      config.lines_for_level,
      font,
    );

    let slf = Self {
      field,
      preview,
      next_tick: Some(Self::next_tick(Instant::now(), score.level())),
      score,
    };
    Ok(slf)
  }

  /// Calculate the time of the next tick, given the current one.
  fn next_tick(current_tick: Instant, level: u16) -> Instant {
    // The current stone drop speed, in units per second.
    let units_per_sec = 1.0 + 0.2 * level as f32;
    current_tick + Duration::from_secs_f32(1.0 / units_per_sec)
  }

  /// Fast-forward the game to the current time.
  ///
  /// This includes moving the currently active stone according to the
  /// elapsed time since the last update.
  pub(crate) fn tick(&mut self, now: Instant) -> (Change, Tick) {
    let mut change = Change::Unchanged;

    match self.field.state() {
      State::Moving { .. } => (),
      State::Clearing { until, .. } => {
        if now > *until {
          self.next_tick = Some(Self::next_tick(*until, self.score.level()));
          let () = self.field.clear_complete_lines();

          change = Change::Changed;
        } else {
          return (change, Tick::At(*until))
        }
      },
      State::Colliding => {
        debug_assert_eq!(self.next_tick, None);
        self.next_tick = None
      },
    }

    while let Some(next_tick) = &mut self.next_tick {
      if now >= *next_tick {
        let result = self.field.move_stone_down();
        change |= result.0;

        match result.1 {
          MoveResult::None | MoveResult::Moved => (),
          MoveResult::Merged(lines) => {
            let () = Self::handle_merged_lines(&mut self.score, lines);
          },
          MoveResult::Conflict => {
            let () = self.end();
            break
          },
        }

        *next_tick = Self::next_tick(*next_tick, self.score.level());
      } else {
        break
      }
    }

    let tick = match self.next_tick {
      None => Tick::None,
      Some(next_tick) => Tick::At(next_tick),
    };

    (change, tick)
  }

  /// Restart the game.
  pub(crate) fn restart(&mut self) -> Change {
    let () = self.score.reset();
    let () = if self.field.reset() {
      self.next_tick = Some(Self::next_tick(Instant::now(), self.score.level()));
    } else {
      self.end()
    };

    Change::Changed
  }

  /// End the current game.
  fn end(&mut self) {
    self.next_tick = None;

    println!(
      "{} points @ level {}; total {} lines cleared (game over)",
      self.score.points(),
      self.score.level(),
      self.score.lines()
    );
  }

  /// Pause or unpause the game.
  #[inline]
  pub(crate) fn pause(&mut self, pause: bool) {
    if pause {
      let _next_tick = self.next_tick.take();
    } else {
      if !matches!(self.field.state(), State::Colliding) {
        let _next_tick = self
          .next_tick
          .replace(Self::next_tick(Instant::now(), self.score.level()));
      }
    }
  }

  /// Inquire whether the game is currently paused.
  #[inline]
  pub(crate) fn is_paused(&self) -> Option<bool> {
    if matches!(self.field.state(), State::Colliding) {
      None
    } else {
      Some(self.next_tick.is_none())
    }
  }

  fn handle_merged_lines(score: &mut Score, lines: u16) {
    let level = score.level();
    let () = score.add(lines);
    let new_level = score.level();

    // While we actually render the score in real-time, we also print to
    // stdout on level up, just to have a history in a slightly more
    // persistent location (still visible after the main window got
    // closed).
    if new_level != level {
      println!("{} points @ level {}", score.points(), new_level);
    }
  }

  #[inline]
  pub(crate) fn on_move_down(&mut self) -> Change {
    if self.next_tick.is_some() {
      let (change, result) = self.field.move_stone_down();
      match result {
        MoveResult::None | MoveResult::Moved => (),
        MoveResult::Merged(lines) => {
          let () = Self::handle_merged_lines(&mut self.score, lines);
        },
        MoveResult::Conflict => {
          let () = self.end();
        },
      }

      change
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_drop(&mut self) -> Change {
    if self.next_tick.is_some() {
      let (change, result) = self.field.drop_stone();
      match result {
        MoveResult::None | MoveResult::Moved => (),
        MoveResult::Merged(lines) => {
          let () = Self::handle_merged_lines(&mut self.score, lines);
        },
        MoveResult::Conflict => {
          let () = self.end();
        },
      }

      change
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_move_left(&mut self) -> Change {
    if self.next_tick.is_some() {
      self.field.move_stone_left()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_move_right(&mut self) -> Change {
    if self.next_tick.is_some() {
      self.field.move_stone_right()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_rotate_left(&mut self) -> Change {
    if self.next_tick.is_some() {
      self.field.rotate_stone_left()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_rotate_right(&mut self) -> Change {
    if self.next_tick.is_some() {
      self.field.rotate_stone_right()
    } else {
      Change::Unchanged
    }
  }

  /// Render the game and its components.
  pub(crate) fn render(&self, renderer: &Renderer) {
    let () = self.field.render(renderer);
    let () = self.preview.render(renderer);
    let () = self.score.render(renderer);
  }

  /// Retrieve the game surface's width.
  pub(crate) fn width(&self) -> NonZeroU16 {
    let width =
      LEFT_SPACE + self.field.width() + PREVIEW_FIELD_SPACE + self.preview.width() + RIGHT_SPACE;
    // SAFETY: The provided height is guaranteed to be greater than zero.
    unsafe { NonZeroU16::new_unchecked(width as u16) }
  }

  /// Retrieve the game surface's height.
  pub(crate) fn height(&self) -> NonZeroU16 {
    let height = BOTTOM_SPACE + self.field.height() + TOP_SPACE;
    // SAFETY: The provided height is guaranteed to be greater than zero.
    unsafe { NonZeroU16::new_unchecked(height as u16) }
  }
}
