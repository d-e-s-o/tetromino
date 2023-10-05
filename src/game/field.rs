// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem::replace;
use std::rc::Rc;
use std::time::Duration;
use std::time::Instant;

use crate::ActiveRenderer as Renderer;
use crate::Change;
use crate::Color;
use crate::Point;
use crate::Rect;
use crate::Texture;

use super::Matrix;
use super::Piece;
use super::Stone;
use super::StoneProducer;


/// The width of each wall.
const WALL_WIDTH: i16 = 1;


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
pub(super) enum State {
  Moving {
    /// The currently active stone.
    stone: Stone,
  },
  /// Completed lines are currently being cleared.
  Clearing { next_stone: Stone, until: Instant },
  /// The last move has resulted in a collision. No further stone
  /// movement is possible.
  Colliding,
}


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
  wall: Texture,
}

impl Field {
  pub(super) fn new(
    location: Point<i16>,
    width: i16,
    height: i16,
    clear_time: Duration,
    producer: Rc<dyn StoneProducer>,
    piece: Texture,
    back: Texture,
  ) -> Self {
    let pieces = PieceField::new(width, height, back, piece.clone());
    let mut stone = producer.create_stone();
    let state = if pieces.reset_stone(&mut stone) {
      State::Moving { stone }
    } else {
      State::Colliding
    };

    Self {
      location,
      clear_time,
      state,
      producer,
      pieces,
      // The walls just use the "piece" texture.
      wall: piece,
    }
  }

  /// Remove all completed lines from the field.
  pub(super) fn clear_complete_lines(&mut self) {
    match &mut self.state {
      State::Clearing { next_stone, until } => {
        debug_assert!(Instant::now() > *until);
        let () = self.pieces.remove_complete_lines();
        self.state = State::Moving {
          stone: next_stone.take(),
        };
      },
      State::Moving { .. } | State::Colliding => (),
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
      self.state = State::Colliding;
      false
    }
  }

  /// Move the stone down.
  fn move_stone_down_impl(&mut self) -> (Change, MoveResult) {
    match &mut self.state {
      State::Moving { stone } => {
        debug_assert!(!self.pieces.collides(stone));
        let () = stone.move_by(0, -1);

        if self.pieces.collides(stone) {
          let () = stone.move_by(0, 1);

          let new_stone = self.producer.create_stone();
          let old_stone = replace(stone, new_stone);

          let cleared = self.pieces.merge_stone(old_stone);
          if !self.pieces.reset_stone(stone) {
            (Change::Changed, MoveResult::Conflict)
          } else {
            if cleared > 0 {
              self.state = State::Clearing {
                next_stone: stone.take(),
                until: Instant::now() + self.clear_time,
              };
            }
            (Change::Changed, MoveResult::Merged(cleared))
          }
        } else {
          (Change::Changed, MoveResult::Moved)
        }
      },
      State::Clearing { .. } => (Change::Unchanged, MoveResult::None),
      State::Colliding => (Change::Unchanged, MoveResult::Conflict),
    }
  }

  pub(super) fn drop_stone(&mut self) -> (Change, MoveResult) {
    loop {
      let (change, result) = self.move_stone_down_impl();
      if !matches!(result, MoveResult::Moved) {
        break (change, result)
      }
    }
  }

  pub(super) fn move_stone_down(&mut self) -> (Change, MoveResult) {
    self.move_stone_down_impl()
  }

  fn move_stone_by(&mut self, x: i16, y: i16) -> Change {
    match &mut self.state {
      State::Moving { stone } => {
        let () = stone.move_by(x, y);

        if self.pieces.collides(stone) {
          let () = stone.move_by(-x, -y);
          Change::Unchanged
        } else {
          Change::Changed
        }
      },
      State::Clearing { .. } | State::Colliding => Change::Unchanged,
    }
  }

  pub(super) fn move_stone_left(&mut self) -> Change {
    self.move_stone_by(-1, 0)
  }

  pub(super) fn move_stone_right(&mut self) -> Change {
    self.move_stone_by(1, 0)
  }

  fn rotate_stone(&mut self, left: bool) -> Change {
    match &mut self.state {
      State::Moving { stone } => {
        let () = stone.rotate(left);

        if self.pieces.collides(stone) {
          let () = stone.rotate(!left);
          Change::Unchanged
        } else {
          Change::Changed
        }
      },
      State::Clearing { .. } | State::Colliding => Change::Unchanged,
    }
  }

  pub(super) fn rotate_stone_left(&mut self) -> Change {
    self.rotate_stone(true)
  }

  pub(super) fn rotate_stone_right(&mut self) -> Change {
    self.rotate_stone(false)
  }

  /// Render the walls of the field.
  fn render_walls(&self, renderer: &Renderer) {
    let _guard = renderer.set_texture(&self.wall);
    let _guard = renderer.set_color(Color::orange());

    let left = Rect::new(0, 0, WALL_WIDTH, self.height());
    let () = renderer.render_rect_with_tex_coords(left, left.into_other());

    let bottom = Rect::new(0, 0, self.width(), WALL_WIDTH);
    let () = renderer.render_rect_with_tex_coords(bottom, bottom.into_other());

    let right = Rect::new(
      WALL_WIDTH + self.pieces.width(),
      0,
      WALL_WIDTH,
      self.height(),
    );
    let () = renderer.render_rect_with_tex_coords(right, right.into_other());
  }

  /// Render the currently active stone (if any).
  fn render_stone(&self, renderer: &Renderer) {
    match &self.state {
      State::Moving { stone }
      | State::Clearing {
        next_stone: stone, ..
      } => stone.render(renderer),
      State::Colliding => (),
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


struct PieceField {
  /// The matrix (2D array) of pieces.
  matrix: Matrix<Piece>,
  /// The texture to use for the entire inner back area.
  back: Texture,
  /// The texture to use for pieces.
  piece: Texture,
}

impl PieceField {
  fn new(width: i16, height: i16, back: Texture, piece: Texture) -> Self {
    Self {
      matrix: Matrix::new(width, height),
      back,
      piece,
    }
  }

  /// Move the stone to its initial position.
  ///
  /// # Returns
  /// This method returns `true` when the stone could be positioned and
  /// `false` if it immediately collided with already merged pieces.
  fn reset_stone(&self, stone: &mut Stone) -> bool {
    let stone_bounds = stone.bounds();
    let x = self.width() / 2 - stone_bounds.w / 2;
    let y = self.height() - stone_bounds.h;
    let () = stone.move_to(Point::new(x, y));

    !self.collides(stone)
  }

  /// Check whether the provided stone collides with any of the pieces.
  fn collides(&self, stone: &Stone) -> bool {
    stone.pieces().any(|location| {
      if location.x < 0 || location.x >= self.matrix.width() {
        return true
      }

      if location.y < 0 || location.y >= self.matrix.height() {
        return true
      }

      self.matrix[location].is_some()
    })
  }

  fn merge_stone(&mut self, stone: Stone) -> u16 {
    // We should not have a current collision so that there is no
    // overlap of pieces in any shape or form.
    debug_assert!(!self.collides(&stone));

    let bounds = stone.bounds();
    let () = stone.into_pieces().for_each(|(piece, location)| {
      let _prev = self.matrix[location].replace(piece);
      debug_assert!(_prev.is_none(), "{location:?}");
    });

    let mut cleared = 0;
    for line in (bounds.y..bounds.y + bounds.h).rev() {
      if self.line_complete(line) {
        cleared += 1;
      }
    }
    cleared
  }

  fn remove_complete_lines(&mut self) {
    // Remove all completed lines; from top to bottom so that we are
    // unaffected by changes of index to lower lines caused by the
    // removal.
    for line in (0..self.height()).rev() {
      if self.line_complete(line) {
        let () = self.matrix.remove_line(line);
      }
    }
  }

  /// Checker whether the line at the given y position is complete.
  #[inline]
  fn line_complete(&self, line: i16) -> bool {
    self.matrix.iter_line(line).all(|elem| elem.is_some())
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
      // TODO: Make the color configurable.
      let _guard = renderer.set_texture(&self.back);
      let _guard = renderer.set_color(Color::white());

      let () = renderer.render_rect(Rect::new(0, 0, self.width(), self.height()));
    }

    // Render the vertical lines in the background.
    {
      let _guard = renderer.set_no_texture();
      let _guard = renderer.set_color(Color::black());

      for i in 0..=self.width() {
        let () = renderer.render_line(Point::new(i, 0), Point::new(i, self.height()));
      }
    }
  }

  /// Render the already dropped pieces.
  fn render_pieces(&self, renderer: &Renderer) {
    let _guard = renderer.set_texture(&self.piece);

    self.matrix.iter_present().for_each(|(piece, location)| {
      let () = piece.render(renderer, location);
    })
  }

  fn render(&self, renderer: &Renderer) {
    let () = self.render_back(renderer);
    let () = self.render_pieces(renderer);
  }

  #[inline]
  fn width(&self) -> i16 {
    self.matrix.width()
  }

  #[inline]
  fn height(&self) -> i16 {
    self.matrix.height()
  }
}
