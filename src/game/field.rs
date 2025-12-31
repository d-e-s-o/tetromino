// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem::replace;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

use crate::mode::ColorMode;
use crate::mode::ColorSet;
use crate::ActiveRenderer as Renderer;
use crate::Change;
use crate::Color;
use crate::Point;
use crate::Rect;
use crate::Texture;

use super::ai;
use super::Fieldlike;
use super::Matrix;
use super::Piece;
use super::Stone;
use super::StoneProducer;
use super::Stonelike as _;


/// The width of each wall.
const WALL_WIDTH: i16 = 1;

const BACKGROUND_COLOR: ColorSet = ColorSet::new(
  Color::white(),
  Color::white().csub(Color {
    r: 85,
    g: 85,
    b: 85,
    a: 0,
  }),
);

const WALL_COLOR: ColorSet = ColorSet::new(Color::orange(), Color::gray());


/// The result of a stone downward movement.
#[derive(Debug)]
pub(super) enum MoveResult {
  /// Nothing was moved.
  None,
  /// The stone was moved down successfully and without a collision.
  Moved,
  /// The stone got merged into the field. Reported are the number of
  /// lines cleared.
  Merged(u16),
  /// A conflict has occurred, i.e., a stone got merged, but the
  /// replacement stone immediately collided with previously merged
  /// pieces in the field.
  Conflict,
}


/// An enumeration of the possible states that the field can be in.
#[derive(Debug)]
pub(super) enum State {
  Moving {
    /// The currently active stone.
    stone: Stone,
  },
  /// Completed lines are currently being cleared.
  Clearing {
    /// The next stone to be controlled by the user.
    next_stone: Stone,
    /// The instant at which we are done clearing completed lines.
    until: Instant,
    /// The y-range containing completed lines.
    y_range: Range<i16>,
  },
  /// The last move has resulted in a collision. No further stone
  /// movement is possible.
  Colliding {
    /// The colliding stone.
    stone: Stone,
  },
}


#[derive(Debug)]
pub(crate) struct Field {
  /// The location of the lower left corner of the field, in game units.
  location: Point<i16>,
  /// The time we take for clearing completed lines.
  clear_time: Duration,
  /// The inner field area, containing dropped pieces.
  pieces: PieceField,
  /// The field's current state.
  state: State,
  /// The producer we use for creating new stones.
  producer: Rc<dyn StoneProducer>,
  /// The texture to use for one unit of wall.
  wall: Rc<Texture>,
  /// The color to use for the walls.
  wall_color: ColorMode,
}

impl Field {
  pub(super) fn new(
    location: Point<i16>,
    width: i16,
    height: i16,
    clear_time: Duration,
    producer: Rc<dyn StoneProducer>,
    piece: Rc<Texture>,
    back: Rc<Texture>,
  ) -> Self {
    let pieces = PieceField::new(width, height, back, Rc::clone(&piece));
    let mut stone = producer.create_stone();
    let state = if pieces.reset_stone(&mut stone) {
      State::Moving { stone }
    } else {
      State::Colliding { stone }
    };

    Self {
      location,
      clear_time,
      state,
      producer,
      pieces,
      // The walls just use the "piece" texture.
      wall: piece,
      wall_color: ColorMode::Light(WALL_COLOR.light),
    }
  }

  /// Remove all completed lines from the field.
  pub(super) fn clear_complete_lines(&mut self) {
    match &mut self.state {
      State::Clearing {
        next_stone,
        until,
        y_range,
      } => {
        debug_assert!(Instant::now() > *until);
        let _removed = self.pieces.remove_complete_lines(y_range.clone());
        self.state = State::Moving {
          stone: next_stone.take(),
        };
      },
      State::Moving { .. } | State::Colliding { .. } => (),
    }
  }

  /// Reset the field back to its initial state, with no merged pieces
  /// and a stone at its initial position.
  pub(super) fn reset(&mut self) -> bool {
    let () = self.pieces.clear();
    let mut stone = self.producer.create_stone();
    if self.pieces.reset_stone(&mut stone) {
      self.state = State::Moving { stone };
      true
    } else {
      self.state = State::Colliding { stone };
      false
    }
  }

  /// Move the stone down.
  fn move_stone_down_impl(&mut self) -> (Change, MoveResult) {
    match &mut self.state {
      State::Moving { stone } => {
        debug_assert!(!self.pieces.collides(stone));
        let () = stone.move_down();

        if self.pieces.collides(stone) {
          let () = stone.move_by(0, 1);

          let new_stone = self.producer.create_stone();
          let old_stone = replace(stone, new_stone);
          let bounds = old_stone.bounds();
          let y_range = bounds.y..bounds.y + bounds.h;

          let cleared = self.pieces.merge_stone(old_stone);
          if !self.pieces.reset_stone(stone) {
            self.state = State::Colliding {
              stone: stone.take(),
            };
            (Change::Changed, MoveResult::Conflict)
          } else {
            if cleared > 0 {
              self.state = State::Clearing {
                next_stone: stone.take(),
                until: Instant::now() + self.clear_time,
                y_range,
              };
            }
            (Change::Changed, MoveResult::Merged(cleared))
          }
        } else {
          (Change::Changed, MoveResult::Moved)
        }
      },
      State::Clearing { .. } => (Change::Unchanged, MoveResult::None),
      State::Colliding { .. } => (Change::Unchanged, MoveResult::Conflict),
    }
  }

  pub(super) fn drop_stone(&mut self) -> (Change, MoveResult) {
    let mut change = Change::Unchanged;
    loop {
      let result = self.move_stone_down_impl();
      change |= result.0;

      if !matches!(result.1, MoveResult::Moved) {
        break (change, result.1)
      }
    }
  }

  pub(super) fn move_stone_down(&mut self) -> (Change, MoveResult) {
    self.move_stone_down_impl()
  }

  /// Move the stone horizontally by the given amount.
  fn move_stone_by(&mut self, x: i16) -> Change {
    match &mut self.state {
      State::Moving { stone }
      | State::Clearing {
        next_stone: stone, ..
      } => {
        let () = stone.move_by(x, 0);

        if self.pieces.collides(stone) {
          let () = stone.move_by(-x, 0);
          Change::Unchanged
        } else {
          Change::Changed
        }
      },
      State::Colliding { .. } => Change::Unchanged,
    }
  }

  pub(super) fn move_stone_left(&mut self) -> Change {
    self.move_stone_by(-1)
  }

  pub(super) fn move_stone_right(&mut self) -> Change {
    self.move_stone_by(1)
  }

  fn rotate_stone(&mut self, left: bool) -> Change {
    match &mut self.state {
      State::Moving { stone }
      | State::Clearing {
        next_stone: stone, ..
      } => {
        let () = stone.rotate(left);

        if self.pieces.collides(stone) {
          let () = stone.rotate(!left);
          Change::Unchanged
        } else {
          Change::Changed
        }
      },
      State::Colliding { .. } => Change::Unchanged,
    }
  }

  pub(super) fn rotate_stone_left(&mut self) -> Change {
    self.rotate_stone(true)
  }

  pub(super) fn rotate_stone_right(&mut self) -> Change {
    self.rotate_stone(false)
  }

  /// "Event handler" for informing the field that the overall game has
  /// been paused.
  pub(super) fn on_pause(&mut self) {
    match &mut self.state {
      State::Clearing {
        next_stone,
        y_range,
        ..
      } => {
        let _removed = self.pieces.remove_complete_lines(y_range.clone());
        self.state = State::Moving {
          stone: next_stone.take(),
        };
      },
      State::Colliding { .. } => panic!("attempted to pause from collision state"),
      State::Moving { .. } => (),
    }
  }

  /// Render the walls of the field.
  fn render_walls(&self, renderer: &Renderer) {
    let _guard = renderer.set_texture(&self.wall);
    let _guard = renderer.set_color(self.wall_color.color());

    let left = Rect::new(0, 0, WALL_WIDTH, self.height());
    let () = renderer.render_rect_with_tex_coords(left.into_other(), left);

    let bottom = Rect::new(0, 0, self.width(), WALL_WIDTH);
    let () = renderer.render_rect_with_tex_coords(bottom.into_other(), bottom);

    let right = Rect::new(
      WALL_WIDTH + self.pieces.width(),
      0,
      WALL_WIDTH,
      self.height(),
    );
    let () = renderer.render_rect_with_tex_coords(right.into_other(), right);
  }

  /// Render the currently active stone (if any).
  fn render_stone(&self, renderer: &Renderer) {
    match &self.state {
      State::Moving { stone }
      | State::Clearing {
        next_stone: stone, ..
      } => stone.render(renderer),
      State::Colliding { stone } => stone.render_with_overlay(renderer, Color::white()),
    }
  }

  /// Render the Tetris field.
  pub(super) fn render(&self, renderer: &Renderer) {
    let _guard = renderer.set_origin(self.location);

    {
      let _guard = renderer.set_origin(Point::new(WALL_WIDTH, WALL_WIDTH));
      let () = self.pieces.render(renderer);
      let () = self.render_stone(renderer);
    }

    let () = self.render_walls(renderer);
  }

  /// Toggle the color mode (light/dark) in use.
  pub(crate) fn toggle_color_mode(&mut self) {
    let () = self.wall_color.toggle_with(&WALL_COLOR);
    let () = self.pieces.toggle_color_mode();
  }

  /// Convert this `Field` into an `ai::Field` together with an
  /// `ai::Stone` representing the currently active stone.
  ///
  /// This method returns `None` if there is a collision.
  #[inline]
  pub(super) fn to_ai_data(&self) -> Option<(ai::Field, ai::Stone)> {
    match &self.state {
      State::Moving { stone } => {
        let field = ai::Field::from_matrix(&self.pieces.matrix);
        let stone = stone.to_ai_stone();
        Some((field, stone))
      },
      State::Clearing {
        next_stone: stone,
        y_range,
        ..
      } => {
        // If we are still clearing completed lines we haven't yet
        // updated the piece data, but the AI certainly should only see
        // the state with cleared lines. So clear them after conversion.
        let mut field = ai::Field::from_matrix(&self.pieces.matrix);
        let _removed = field.remove_complete_lines(y_range.clone());
        let stone = stone.to_ai_stone();
        Some((field, stone))
      },
      State::Colliding { .. } => None,
    }
  }

  #[inline]
  pub(super) fn state(&self) -> &State {
    &self.state
  }

  #[inline]
  pub(super) fn total_width(width: i16) -> i16 {
    2 * WALL_WIDTH + width
  }

  #[inline]
  pub(super) fn total_height(height: i16) -> i16 {
    WALL_WIDTH + height
  }

  #[inline]
  pub(super) fn width(&self) -> i16 {
    Self::total_width(self.pieces.width())
  }

  #[inline]
  pub(super) fn height(&self) -> i16 {
    Self::total_height(self.pieces.height())
  }
}


#[derive(Debug)]
struct PieceField {
  /// The matrix (2D array) of pieces.
  matrix: Matrix<Option<Piece>>,
  /// The texture to use for the entire inner back area.
  back: Rc<Texture>,
  /// The color to use for the background image.
  back_color: ColorMode,
  /// The texture to use for pieces.
  piece: Rc<Texture>,
}

impl PieceField {
  fn new(width: i16, height: i16, back: Rc<Texture>, piece: Rc<Texture>) -> Self {
    Self {
      matrix: Matrix::new(width, height),
      back,
      back_color: ColorMode::Light(BACKGROUND_COLOR.light),
      piece,
    }
  }

  /// Clear all pieces from the field.
  #[inline]
  fn clear(&mut self) {
    self.matrix.clear()
  }

  /// Render the background of the field and draw vertical lines.
  fn render_back(&self, renderer: &Renderer) {
    // Render background image.
    {
      let _guard = renderer.set_texture(&self.back);
      let _guard = renderer.set_color(self.back_color.color());

      let () = renderer.render_rect(Rect::new(0, 0, self.width(), self.height()));
    }

    // Render the vertical lines in the background.
    {
      let _guard = renderer.set_no_texture();
      let _guard = renderer.set_color(Color::black());

      for i in 1..self.width() {
        let () = renderer.render_line(Point::new(i, 0), Point::new(i, self.height()));
      }
    }
  }

  /// Render the already dropped pieces.
  fn render_pieces(&self, renderer: &Renderer) {
    let _guard = renderer.set_texture(&self.piece);

    let mut complete = (-1, false);
    // By overlaying the piece's color with white we effectively force
    // it to be white altogether, because adding white to anything
    // always results in white.
    let overlay = Color::white();

    self
      .matrix
      .iter()
      .filter_map(|(piece, location)| piece.map(|piece| (piece, location)))
      .for_each(|(piece, location)| {
        if complete.0 != location.y {
          complete = (location.y, self.line_complete(location.y));
        }

        if complete.1 {
          let () = piece.render_with_overlay(renderer, location, overlay);
        } else {
          let () = piece.render(renderer, location);
        }
      })
  }

  fn render(&self, renderer: &Renderer) {
    let () = self.render_back(renderer);
    let () = self.render_pieces(renderer);
  }

  /// Toggle the color mode (light/dark) in use.
  pub(crate) fn toggle_color_mode(&mut self) {
    let () = self.back_color.toggle_with(&BACKGROUND_COLOR);
    let mode = self.back_color.stripped();

    for (piece, _position) in self.matrix.iter_mut() {
      if let Some(piece) = piece {
        let () = piece.set_color_mode(mode);
      }
    }
  }
}

impl Index<Point<i16>> for PieceField {
  type Output = Option<Piece>;

  fn index(&self, index: Point<i16>) -> &Self::Output {
    &self.matrix[(index.x, index.y)]
  }
}

impl IndexMut<Point<i16>> for PieceField {
  fn index_mut(&mut self, index: Point<i16>) -> &mut Self::Output {
    &mut self.matrix[(index.x, index.y)]
  }
}

impl Fieldlike<Stone> for PieceField {
  #[inline]
  fn width(&self) -> i16 {
    self.matrix.width()
  }

  #[inline]
  fn height(&self) -> i16 {
    self.matrix.height()
  }

  #[inline]
  fn line_complete(&self, line: i16) -> bool {
    self.matrix.iter_line(line).all(Option::is_some)
  }

  #[inline]
  fn remove_line(&mut self, line: i16) {
    self.matrix.remove_line(line)
  }
}
