// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::mem::replace;
use std::rc::Rc;

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Point;
use crate::Rect;
use crate::State;
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


pub(crate) struct Field {
  /// The location of the lower left corner of the field, in game units.
  location: Point<i16>,
  /// The inner field area, containing dropped pieces.
  pieces: PieceField,
  /// The producer we use for creating new stones.
  producer: Rc<dyn StoneProducer>,
  /// The currently active stone.
  stone: Stone,
  /// The texture to use for one unit of wall.
  wall: Texture,
}

impl Field {
  pub(super) fn new(
    location: Point<i16>,
    width: i16,
    height: i16,
    producer: Rc<dyn StoneProducer>,
    piece: Texture,
    back: Texture,
  ) -> Result<Self, Self> {
    let pieces = PieceField::new(width, height, back, piece.clone());
    let mut stone = producer.create_stone();
    let result = pieces.reset_stone(&mut stone);

    let slf = Self {
      location,
      producer,
      stone,
      pieces,
      // The walls just use the "piece" texture.
      wall: piece,
    };

    if result {
      Ok(slf)
    } else {
      Err(slf)
    }
  }

  /// Reset the field back to its initial state, with no merged pieces
  /// and a stone at its initial position.
  pub(super) fn reset(&mut self) -> bool {
    let () = self.pieces.clear();
    self.stone = self.producer.create_stone();
    self.pieces.reset_stone(&mut self.stone)
  }

  /// Move the stone down.
  fn move_stone_down_impl(&mut self) -> MoveResult {
    debug_assert!(!self.pieces.collides(&self.stone));

    let () = self.stone.move_by(0, -1);

    if self.pieces.collides(&self.stone) {
      let () = self.stone.move_by(0, 1);

      let new_stone = self.producer.create_stone();
      let old_stone = replace(&mut self.stone, new_stone);

      let cleared = self.pieces.merge_stone(old_stone);
      if !self.pieces.reset_stone(&mut self.stone) {
        MoveResult::Conflict
      } else {
        MoveResult::Merged(cleared)
      }
    } else {
      MoveResult::Moved
    }
  }

  pub(super) fn drop_stone(&mut self) -> (State, MoveResult) {
    loop {
      let result = self.move_stone_down_impl();
      if !matches!(result, MoveResult::Moved) {
        break (State::Changed, result)
      }
    }
  }

  pub(super) fn move_stone_down(&mut self) -> (State, MoveResult) {
    let result = self.move_stone_down_impl();
    (State::Changed, result)
  }

  fn move_stone_by(&mut self, x: i16, y: i16) -> State {
    let () = self.stone.move_by(x, y);

    if self.pieces.collides(&self.stone) {
      let () = self.stone.move_by(-x, -y);
      State::Unchanged
    } else {
      State::Changed
    }
  }

  pub(super) fn move_stone_left(&mut self) -> State {
    self.move_stone_by(-1, 0)
  }

  pub(super) fn move_stone_right(&mut self) -> State {
    self.move_stone_by(1, 0)
  }

  fn rotate_stone(&mut self, left: bool) -> State {
    let () = self.stone.rotate(left);

    if self.pieces.collides(&self.stone) {
      let () = self.stone.rotate(!left);
      State::Unchanged
    } else {
      State::Changed
    }
  }

  pub(super) fn rotate_stone_left(&mut self) -> State {
    self.rotate_stone(true)
  }

  pub(super) fn rotate_stone_right(&mut self) -> State {
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
    let () = self.stone.render(renderer);
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
  pub(super) fn width(&self) -> i16 {
    2 * WALL_WIDTH + self.pieces.width()
  }

  #[inline]
  pub(super) fn height(&self) -> i16 {
    WALL_WIDTH + self.pieces.height()
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
    // Remove all completed lines; from top to bottom so that we are
    // unaffected by changes of index to lower lines caused by the
    // removal.
    for line in (bounds.y..bounds.y + bounds.h).rev() {
      if self.line_complete(line) {
        let () = self.matrix.remove_line(line);
        cleared += 1;
      }
    }
    cleared
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
