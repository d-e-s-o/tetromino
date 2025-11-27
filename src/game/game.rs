// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Cursor;
use std::num::NonZeroU16;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;

use crate::ActiveRenderer as Renderer;
use crate::Change;
use crate::Color;
use crate::ColorMode;
use crate::ColorSet;
use crate::Font;
use crate::Point;
use crate::Texture;
use crate::Tick;

use super::ai;
use super::data;
use super::field::State;
use super::Config;
use super::Field;
use super::MoveResult;
use super::PreviewStones;
use super::Score;
use super::Stone;
use super::StoneFactory;


/// The color set used when clearing the screen.
const SCREEN_CLEAR_COLOR: ColorSet = ColorSet::new(
  Color {
    r: 0xee,
    g: 0xee,
    b: 0xee,
    a: 0xff,
  },
  Color {
    r: 0x11,
    g: 0x11,
    b: 0x11,
    a: 0xff,
  },
);

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
#[derive(Debug)]
pub struct Game {
  /// The color to use for clearing the screen with.
  screen_clear_color: ColorMode,
  /// The Tetris field.
  field: Field,
  /// The preview stones.
  preview: Rc<PreviewStones>,
  /// The current score.
  score: Score,
  /// The AI playing the game, if any.
  ai: Option<ai::AI>,
  /// The time of the next tick, i.e., the next downward movement.
  ///
  /// The attribute is `None` if the game is not running (e.g., paused).
  next_tick: Option<Instant>,
}

impl Game {
  /// Instantiate a new game of Tetris with the given configuration.
  pub fn with_config(config: &Config) -> Result<Self> {
    let reader = Cursor::new(data::TETRIS_FIELD_PIECE_TEXTURE);
    let piece = image::ImageReader::with_format(reader, image::ImageFormat::Png).decode()?;
    let piece = Texture::with_image(piece)?;

    let factory = Box::new(StoneFactory::with_default_stones(piece.clone()));

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
    let field_back = image::ImageReader::with_format(reader, image::ImageFormat::Png).decode()?;
    let field_back = Texture::with_image(field_back)?;
    let field = Field::new(
      field_location,
      config.field_width,
      config.field_height,
      CLEAR_TIME,
      Rc::clone(&preview) as _,
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

    let ai = if config.enable_ai {
      Self::create_ai(&field, &preview)
    } else {
      None
    };

    let mut slf = Self {
      screen_clear_color: ColorMode::Light(SCREEN_CLEAR_COLOR.light),
      field,
      preview,
      ai,
      next_tick: Some(Self::next_tick(Instant::now(), score.level())),
      score,
    };

    if config.enable_dark_mode {
      let () = slf.toggle_color_mode();
    }
    Ok(slf)
  }

  fn with_ai_data<F, R>(field: &Field, preview: &PreviewStones, f: F) -> Option<R>
  where
    F: FnOnce(&ai::Field, &ai::Stone, &[ai::Stone]) -> R,
  {
    if let Some((field, stone)) = field.to_ai_data() {
      // TODO: Ideally we should use all preview stones (perhaps even
      //       more). But right now our search algorithm is too compute
      //       intensive to make that happen.
      // TODO: Ideally we would not have to allocate here.
      let stones = preview
        .with_stones(move |stones| stones.take(1).map(Stone::to_ai_stone).collect::<Vec<_>>());

      let result = f(&field, &stone, &stones);
      Some(result)
    } else {
      None
    }
  }

  fn advance_ai(ai: &mut ai::AI, field: &Field, preview: &PreviewStones) {
    let _result = Self::with_ai_data(field, preview, |field, stone, next_stones| {
      let () = ai.advance_stone(field, stone, next_stones);
    });
  }

  fn create_ai(field: &Field, preview: &PreviewStones) -> Option<ai::AI> {
    Self::with_ai_data(field, preview, |field, stone, next_stones| {
      ai::AI::new(field, stone, next_stones)
    })
  }

  fn ai_handle_regular_move(ai: &mut Option<ai::AI>, field: &mut Field) -> Change {
    let mut change = Change::Unchanged;

    if let Some(ai) = ai.as_mut() {
      while let Some(action) = ai.peek() {
        change |= match action {
          ai::Action::MoveLeft => field.move_stone_left(),
          ai::Action::MoveRight => field.move_stone_right(),
          ai::Action::RotateLeft => field.rotate_stone_left(),
          ai::Action::RotateRight => field.rotate_stone_right(),
          ai::Action::Merge | ai::Action::MoveDown => return change,
        };

        let _ = ai.next();
      }
    }

    change
  }

  fn ai_remove_down_move(ai: &mut Option<ai::AI>) {
    if let Some(ai) = ai.as_mut() {
      if let Some(ai::Action::MoveDown) = ai.peek() {
        let _ = ai.next();
      }
    }
  }

  fn ai_remove_stone_merge(ai: &mut Option<ai::AI>, field: &Field, preview: &PreviewStones) {
    if let Some(ai) = ai.as_mut() {
      if let Some(ai::Action::Merge) = ai.peek() {
        let _merge = ai.next();
        debug_assert_eq!(ai.next(), None);

        let () = Self::advance_ai(ai, field, preview);
      }
    }
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
  pub fn tick(&mut self, now: Instant) -> (Change, Tick) {
    let mut change = Change::Unchanged;

    match self.field.state() {
      State::Moving { .. } => (),
      State::Clearing { until, .. } => {
        // The game must not be paused while we are clearing. Pausing
        // should always transition the field to "moving" state.
        debug_assert!(self.next_tick.is_some());

        if now > *until {
          self.next_tick = Some(Self::next_tick(*until, self.score.level()));
          let () = self.field.clear_complete_lines();

          change = Change::Changed;
        } else {
          return (change, Tick::At(*until))
        }
      },
      State::Colliding { .. } => {
        debug_assert_eq!(self.next_tick, None);
        self.next_tick = None
      },
    }

    while let Some(next_tick) = &mut self.next_tick {
      change |= Self::ai_handle_regular_move(&mut self.ai, &mut self.field);

      if now >= *next_tick {
        let result = self.field.move_stone_down();
        change |= result.0;

        match result.1 {
          MoveResult::None => (),
          MoveResult::Moved => {
            let () = Self::ai_remove_down_move(&mut self.ai);
          },
          MoveResult::Merged(lines) => {
            let () = Self::handle_merged_lines(&mut self.score, lines);
            let () = Self::ai_remove_down_move(&mut self.ai);
            let () = Self::ai_remove_stone_merge(&mut self.ai, &self.field, &self.preview);
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
  pub fn restart(&mut self) {
    let () = self.score.reset();
    let () = if self.field.reset() {
      if self.ai.is_some() {
        self.ai = Self::create_ai(&self.field, &self.preview);
      }
      self.next_tick = Some(Self::next_tick(Instant::now(), self.score.level()));
    } else {
      self.end()
    };
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
    if !matches!(self.field.state(), State::Colliding { .. }) {
      if pause {
        // Note that strictly speaking the field could change state here
        // (if it was "clearing") and, conceptually, we should cause a
        // redraw (i.e., by returning `Change::Changed`. Practically,
        // though, we do *not* want to do that, because doing so could
        // eagerly remove cleared lines and it just makes more sense to
        // leave them there for the duration of the pause.
        let () = self.field.on_pause();
        let _next_tick = self.next_tick.take();
      } else {
        let _next_tick = self
          .next_tick
          .replace(Self::next_tick(Instant::now(), self.score.level()));
      }
    }
  }

  /// Inquire whether the game is currently paused.
  #[inline]
  pub(crate) fn is_paused(&self) -> bool {
    self.next_tick.is_none()
  }

  /// Enable or disable auto-playing of the game.
  pub(crate) fn auto_play(&mut self, auto_play: bool) {
    if auto_play {
      if self.ai.is_none() {
        self.ai = Self::create_ai(&self.field, &self.preview);
      }
    } else {
      self.ai = None;
    }
  }

  /// Check whether the game is currently controlled by an auto-playing
  /// AI.
  #[inline]
  pub(crate) fn is_auto_playing(&self) -> bool {
    self.ai.is_some()
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

  /// Check whether the game in its current state accepts and reacts to
  /// input.
  ///
  /// It won't accept input if it's currently paused or if the AI is
  /// playing.
  #[inline]
  fn accepts_input(&self) -> bool {
    self.next_tick.is_some() && !self.is_auto_playing()
  }

  #[inline]
  pub(crate) fn on_move_down(&mut self) -> Change {
    if self.accepts_input() {
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
    if self.accepts_input() {
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
    if self.accepts_input() {
      self.field.move_stone_left()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_move_right(&mut self) -> Change {
    if self.accepts_input() {
      self.field.move_stone_right()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_rotate_left(&mut self) -> Change {
    if self.accepts_input() {
      self.field.rotate_stone_left()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_rotate_right(&mut self) -> Change {
    if self.accepts_input() {
      self.field.rotate_stone_right()
    } else {
      Change::Unchanged
    }
  }

  /// Render the game and its components.
  pub fn render(&self, renderer: &Renderer) {
    let () = renderer.clear_screen(self.screen_clear_color.color());
    let () = self.field.render(renderer);
    let () = self.preview.render(renderer);
    let () = self.score.render(renderer);
  }

  /// Toggle the color mode (light/dark) in use.
  pub(crate) fn toggle_color_mode(&mut self) {
    let () = self.screen_clear_color.toggle_with(&SCREEN_CLEAR_COLOR);
    let () = self.field.toggle_color_mode();
    let () = self.preview.toggle_color_mode();
  }

  #[cfg(feature = "debug")]
  pub(crate) fn dump_state(&self) {
    if let Some((stone, field)) = self.field.to_ai_data() {
      if let Some(ai) = self.ai.as_ref() {
        println!("{ai:#?}");
      }
      println!("{stone:?}");
      println!("{field:?}");
    }
  }

  /// Retrieve the game surface's width.
  pub fn width(&self) -> NonZeroU16 {
    let width =
      LEFT_SPACE + self.field.width() + PREVIEW_FIELD_SPACE + self.preview.width() + RIGHT_SPACE;
    // SAFETY: The provided height is guaranteed to be greater than zero.
    unsafe { NonZeroU16::new_unchecked(width as u16) }
  }

  /// Retrieve the game surface's height.
  pub fn height(&self) -> NonZeroU16 {
    let height = BOTTOM_SPACE + self.field.height() + TOP_SPACE;
    // SAFETY: The provided height is guaranteed to be greater than zero.
    unsafe { NonZeroU16::new_unchecked(height as u16) }
  }
}
