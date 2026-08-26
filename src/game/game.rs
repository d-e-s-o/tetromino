// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::max;
use std::cmp::min;
use std::io::Cursor;
use std::num::NonZeroU16;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::time::Duration;

use anyhow::Context as _;
use anyhow::Result;

use xgl::sys;
use xgl::sys::Gl as _;

use crate::ActiveRenderer;
use crate::Change;
use crate::ColorMode;
use crate::ColorSet;
use crate::Instant;
use crate::Point;
use crate::Texture;
use crate::TextureBuilderExt as _;
use crate::Tick;
use crate::gl;
use crate::gl::Renderer;

use super::Blur;
use super::Camera;
use super::Config;
use super::Field;
use super::MoveResult;
use super::PreviewStones;
use super::Score;
use super::Stone;
use super::StoneFactory;
use super::ai;
use super::data;
use super::field::State as FieldState;


/// The color set used when clearing the screen.
const SCREEN_CLEAR_COLOR: ColorSet<(f32, f32, f32)> = ColorSet::new(
  (
    0xee as f32 / 0xff as f32,
    0xee as f32 / 0xff as f32,
    0xee as f32 / 0xff as f32,
  ),
  (
    0x11 as f32 / 0xff as f32,
    0x11 as f32 / 0xff as f32,
    0x11 as f32 / 0xff as f32,
  ),
);

/// Space between the left screen side and the field.
const LEFT_SPACE: i16 = 1;
/// Space between the bottom of the screen and the field.
const BOTTOM_SPACE: i16 = 1;
/// Space between the right side of the screen and the preview
/// stones/score board (whatever is wider).
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


/// The state the [`Game`] is in.
#[derive(Debug)]
enum State {
  Running {
    /// The time of the next tick, i.e., the next downward movement.
    next_tick: Instant,
  },
  Paused {
    /// The state we use for blurring the background.
    blur: Option<Blur>,
  },
  Over,
}


#[derive(Debug)]
struct Inner {
  /// The game's state.
  state: State,
  /// The color mode in use.
  color_mode: ColorMode,
  /// The Tetris field.
  field: Field,
  /// The preview stones.
  preview: Rc<PreviewStones>,
  /// The current score.
  score: Score,
  /// The AI playing the game, if any.
  ai: Option<ai::AI>,
}

impl Inner {
  fn render(&self, renderer: &ActiveRenderer) {
    let field_location = Point::new(LEFT_SPACE, BOTTOM_SPACE);
    {
      let _guard = renderer.set_origin(field_location);
      let () = self.field.render(renderer, self.color_mode);
    }

    let preview_location = field_location
      + Point::new(self.field.display_width(), self.field.display_height())
      + Point::new(RIGHT_SPACE, 0);
    {
      let _guard = renderer.set_origin(preview_location);
      let () = self.preview.render(renderer, self.color_mode);
    }

    let score_location =
      preview_location - Point::new(0, self.preview.height() + PREVIEW_SCORE_SPACE);
    {
      let _guard = renderer.set_origin(score_location);
      let () = self.score.render(renderer);
    }
  }

  /// Retrieve the game surface's width.
  fn width(&self) -> NonZeroU16 {
    let width = LEFT_SPACE
      + self.field.display_width()
      + PREVIEW_FIELD_SPACE
      + max(self.preview.width(), self.score.width())
      + RIGHT_SPACE;
    // SAFETY: The provided width is guaranteed to be greater than zero.
    unsafe { NonZeroU16::new_unchecked(width as u16) }
  }

  /// Retrieve the game surface's height.
  fn height(&self) -> NonZeroU16 {
    let height = BOTTOM_SPACE + self.field.display_height() + TOP_SPACE;
    // SAFETY: The provided height is guaranteed to be greater than zero.
    unsafe { NonZeroU16::new_unchecked(height as u16) }
  }
}


/// A type representing a game of Tetris.
#[derive(Debug)]
pub struct Game {
  /// Our GL render state.
  gl_state: gl::State,
  /// The game's virtual "camera".
  camera: Camera,
  /// The renderer we use.
  renderer: Renderer,
  /// Our inner game state.
  inner: Inner,
}

impl Game {
  /// Instantiate a new game of Tetris with the given configuration.
  pub fn with_config(
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    config: &Config,
    context: &sys::Context,
  ) -> Result<Self> {
    let reader = Cursor::new(data::TETRIS_FIELD_PIECE_TEXTURE);
    let piece = image::ImageReader::with_format(reader, image::ImageFormat::Png).decode()?;
    let piece = Texture::builder()
      .set_context(context)
      .from_dynamic_image(&piece)?;
    let piece = Rc::new(piece);

    let factory = Box::new(StoneFactory::with_default_stones(Rc::clone(&piece)));

    let preview = PreviewStones::new(config.preview_stone_count, factory);
    let preview = Rc::new(preview);

    let reader = Cursor::new(data::TETRIS_FIELD_BACK_TEXTURE);
    let field_back = image::ImageReader::with_format(reader, image::ImageFormat::Png).decode()?;
    let field_back = Texture::builder()
      .set_context(context)
      .from_dynamic_image(&field_back)?;
    let field_back = Rc::new(field_back);
    let field = Field::new(
      config.field_width,
      config.field_height,
      CLEAR_TIME,
      Rc::clone(&preview) as _,
      Rc::clone(&piece),
      field_back,
    );

    let score = Score::new(config.start_level, config.lines_for_level, piece);

    let state = match field.state() {
      FieldState::Moving { .. } | FieldState::Clearing { .. } => State::Running {
        next_tick: Self::next_tick(Instant::now(), score.level()),
      },
      FieldState::Colliding { .. } => State::Over,
    };

    let ai = if config.enable_ai {
      Self::create_ai(&field, &preview)
    } else {
      None
    };

    let inner = Inner {
      state,
      color_mode: if config.enable_dark_mode {
        ColorMode::Dark
      } else {
        ColorMode::Light
      },
      field,
      preview,
      ai,
      score,
    };

    let mut gl_state = gl::State::new(context).context("failed to initialize GL state")?;
    let object = gl_state.object();
    let camera = Camera::new(phys_w, phys_h, inner.width(), inner.height());
    let renderer = Renderer::new(object).context("failed to create GL renderer")?;

    let slf = Self {
      gl_state,
      camera,
      renderer,
      inner,
    };
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
    let next_tick = match &mut self.inner.state {
      State::Running { next_tick } => next_tick,
      State::Paused { .. } | State::Over => return (Change::Unchanged, Tick::None),
    };

    let clearing_until = if let FieldState::Clearing { until, .. } = self.inner.field.state() {
      Some(*until)
    } else {
      None
    };
    let (mut change, field_tick) = self.inner.field.tick(now);

    match self.inner.field.state() {
      FieldState::Moving { .. } => {
        if let Some(until) = clearing_until {
          *next_tick = Self::next_tick(until, self.inner.score.level());
        }
      },
      FieldState::Clearing { .. } => return (change, field_tick),
      FieldState::Colliding { .. } => {
        let () = self.set_game_over();
        return (Change::Unchanged, Tick::None)
      },
    }

    loop {
      change |= Self::ai_handle_regular_move(&mut self.inner.ai, &mut self.inner.field);

      if now >= *next_tick {
        let result = self.inner.field.move_stone_down();
        change |= result.0;

        match result.1 {
          MoveResult::None => (),
          MoveResult::Moved => {
            let () = Self::ai_remove_down_move(&mut self.inner.ai);
          },
          MoveResult::Merged(lines) => {
            change |= Self::handle_merged_lines(&mut self.inner.score, lines);
            let () = Self::ai_remove_down_move(&mut self.inner.ai);
            let () = Self::ai_remove_stone_merge(
              &mut self.inner.ai,
              &self.inner.field,
              &self.inner.preview,
            );
          },
          MoveResult::Conflict => {
            let () = self.set_game_over();
            return (change, Tick::None)
          },
        }

        *next_tick = Self::next_tick(*next_tick, self.inner.score.level());
      } else {
        break
      }
    }

    let game_tick = Tick::At(*next_tick);
    (change, min(field_tick, game_tick))
  }

  /// Update the view after the containing window or contained logical
  /// dimensions have changed.
  pub fn update_view(&mut self, phys_w: Option<NonZeroU32>, phys_h: Option<NonZeroU32>) {
    self
      .camera
      .update_view(phys_w, phys_h, self.inner.width(), self.inner.height())
  }

  /// Restart the game.
  pub fn restart(&mut self) -> Change {
    let change = self.inner.score.reset();

    let () = if self.inner.field.reset() {
      if self.inner.ai.is_some() {
        self.inner.ai = Self::create_ai(&self.inner.field, &self.inner.preview);
      }
      self.inner.state = State::Running {
        next_tick: Self::next_tick(Instant::now(), self.inner.score.level()),
      };
    } else {
      self.set_game_over()
    };

    change
  }

  /// End the current game, setting `state` to [`State::Over`].
  fn set_game_over(&mut self) {
    debug_assert!(!matches!(self.inner.state, State::Over));
    self.inner.state = State::Over;

    println!(
      "{} points @ level {}; total {} lines cleared (game over)",
      self.inner.score.points(),
      self.inner.score.level(),
      self.inner.score.lines()
    );
  }

  /// Pause or unpause the game.
  #[inline]
  pub(crate) fn pause(&mut self, pause: bool) {
    match &self.inner.state {
      State::Running { .. } if pause => {
        // Note that strictly speaking the field could change state here
        // (if it was "clearing") and, conceptually, we should cause a
        // redraw (i.e., by returning `Change::Changed`. Practically,
        // though, we do *not* want to do that, because doing so could
        // eagerly remove cleared lines and it just makes more sense to
        // leave them there for the duration of the pause.
        let () = self.inner.field.on_pause();
        self.inner.state = State::Paused {
          blur: Blur::new(&self.gl_state).ok(),
        };
      },
      State::Paused { .. } if !pause => {
        self.inner.state = State::Running {
          next_tick: Self::next_tick(Instant::now(), self.inner.score.level()),
        }
      },
      State::Over | State::Paused { .. } | State::Running { .. } => (),
    }
  }

  /// Inquire whether the game is currently paused.
  ///
  /// Note that this method truly only returns `true` when the game is
  /// paused, and not just when it isn't running (e.g., because it is
  /// over).
  #[inline]
  pub(crate) fn is_paused(&self) -> bool {
    matches!(&self.inner.state, State::Paused { .. })
  }

  /// Enable or disable auto-playing of the game.
  pub(crate) fn auto_play(&mut self, auto_play: bool) {
    if auto_play {
      if self.inner.ai.is_none() {
        self.inner.ai = Self::create_ai(&self.inner.field, &self.inner.preview);
      }
    } else {
      self.inner.ai = None;
    }
  }

  /// Check whether the game is currently controlled by an auto-playing
  /// AI.
  #[inline]
  pub(crate) fn is_auto_playing(&self) -> bool {
    self.inner.ai.is_some()
  }

  fn handle_merged_lines(score: &mut Score, lines: u16) -> Change {
    let level = score.level();
    let change = score.add(lines);
    let new_level = score.level();

    // While we actually render the score in real-time, we also print to
    // stdout on level up, just to have a history in a slightly more
    // persistent location (still visible after the main window got
    // closed).
    if new_level != level {
      println!("{} points @ level {}", score.points(), new_level);
    }
    change
  }

  /// Check whether the game in its current state accepts and reacts to
  /// input.
  ///
  /// It won't accept input if it's currently paused or if the AI is
  /// playing.
  #[inline]
  fn accepts_input(&self) -> bool {
    matches!(self.inner.state, State::Running { .. }) && !self.is_auto_playing()
  }

  #[inline]
  pub(crate) fn on_move_down(&mut self) -> Change {
    if self.accepts_input() {
      let (mut change, result) = self.inner.field.move_stone_down();
      match result {
        MoveResult::None | MoveResult::Moved => (),
        MoveResult::Merged(lines) => {
          change |= Self::handle_merged_lines(&mut self.inner.score, lines);
        },
        MoveResult::Conflict => {
          let () = self.set_game_over();
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
      let (mut change, result) = self.inner.field.drop_stone();
      match result {
        MoveResult::None | MoveResult::Moved => (),
        MoveResult::Merged(lines) => {
          change |= Self::handle_merged_lines(&mut self.inner.score, lines);
        },
        MoveResult::Conflict => {
          let () = self.set_game_over();
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
      self.inner.field.move_stone_left()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_move_right(&mut self) -> Change {
    if self.accepts_input() {
      self.inner.field.move_stone_right()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_rotate_left(&mut self) -> Change {
    if self.accepts_input() {
      self.inner.field.rotate_stone_left()
    } else {
      Change::Unchanged
    }
  }

  #[inline]
  pub(crate) fn on_rotate_right(&mut self) -> Change {
    if self.accepts_input() {
      self.inner.field.rotate_stone_right()
    } else {
      Change::Unchanged
    }
  }

  /// Render the game and its components.
  pub fn render(&mut self) {
    let clear_color = SCREEN_CLEAR_COLOR.select(self.inner.color_mode);

    match &self.inner.state {
      State::Paused { blur: Some(blur) } => {
        let gl_state = self.gl_state.object();
        let () = blur.render_scene(gl_state, clear_color, |object| {
          let () = self.camera.render_scene(object, |object| {
            let renderer = self.renderer.on_pre_render(object);
            let () = self.inner.render(&renderer);
          });
        });

        let gl_state = self.gl_state.blur();
        let () = self.camera.set_viewport(gl_state);
        let () = blur.render_blur(gl_state);
      },
      State::Running { .. } | State::Paused { blur: None } | State::Over => {
        let gl_state = self.gl_state.object();
        let () = self.camera.set_viewport(gl_state);
        let () = self.camera.render_scene(gl_state, |object| {
          let (r, g, b) = clear_color;
          let () = object.set_clear_color(r, g, b, 1.0);
          let () = object.clear(sys::ClearMask::ColorBuffer);
          let renderer = self.renderer.on_pre_render(object);
          let () = self.inner.render(&renderer);
        });
      },
    }
  }

  /// Convert the game (back) into a [`Config`].
  pub fn into_config(self) -> Config {
    Config {
      start_level: self.inner.score.start_level(),
      lines_for_level: self.inner.score.lines_for_level(),
      field_width: self.inner.field.width(),
      field_height: self.inner.field.height(),
      preview_stone_count: self.inner.preview.with_stones(|stones| stones.count()) as _,
      enable_ai: self.inner.ai.is_some(),
      enable_dark_mode: matches!(self.inner.color_mode, ColorMode::Dark),
    }
  }

  /// Toggle the color mode (light/dark) in use.
  pub(crate) fn toggle_color_mode(&mut self) {
    let () = self.inner.color_mode.toggle();
  }

  #[cfg(feature = "debug")]
  pub(crate) fn dump_state(&self) {
    if let Some((stone, field)) = self.inner.field.to_ai_data() {
      if let Some(ai) = self.inner.ai.as_ref() {
        println!("{ai:#?}");
      }
      println!("{stone:?}");
      println!("{field:?}");
    }
  }
}


#[cfg(test)]
#[cfg(feature = "nightly")]
mod tests {
  use super::*;

  use test::Bencher;

  use winit::event_loop::EventLoop;
  use winit::platform::x11::EventLoopBuilderExtX11 as _;
  use winit::raw_window_handle::HasDisplayHandle as _;

  use crate::game::Config;
  use crate::winit::Window;


  /// Benchmark the performance of the rendering path.
  // TODO: It would be good to use `with_opengl_context` in one form or
  //       another instead of repeating a lot of what it does.
  #[allow(deprecated)]
  #[bench]
  fn bench_render(b: &mut Bencher) {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let display_handle = event_loop.display_handle().unwrap();
    let raw_display_handle = display_handle.into();
    let create_window_fn = |attrs| event_loop.create_window(attrs);
    let mut window = Window::new(raw_display_handle, create_window_fn).unwrap();
    let context = window.render_context().gl_context();
    let (phys_w, phys_h) = window.size();
    let config = Config::default();
    let mut game = Game::with_config(phys_w, phys_h, &config, context).unwrap();

    let () = b.iter(|| {
      let () = game.render();
      let () = window.render_context_mut().swap_buffers();
    });
  }
}
