// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Index;
use std::ops::IndexMut;
use std::ops::Range;

use crate::Point;

use super::Stonelike;


pub(super) trait Fieldlike<S>
where
  S: Stonelike,
  Self: Index<Point<i16>, Output = Option<S::Piece>>,
  Self: IndexMut<Point<i16>>,
{
  fn width(&self) -> i16;
  fn height(&self) -> i16;

  /// Check whether the provided stone collides with any of the pieces.
  fn collides(&self, stone: &S) -> bool {
    stone.pieces().any(|location| {
      if location.x < 0 || location.x >= self.width() {
        return true
      }

      if location.y < 0 || location.y >= self.height() {
        return true
      }

      self[location].is_some()
    })
  }

  /// Merge the provided stone into the field.
  fn merge_stone(&mut self, stone: S) -> u16 {
    // We should not have a current collision so that there is no
    // overlap of pieces in any shape or form.
    debug_assert!(!self.collides(&stone));

    let bounds = stone.bounds();
    let () = stone.into_pieces().for_each(|(piece, location)| {
      let _prev = self[location].replace(piece);
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

  /// Move the stone to its initial position.
  ///
  /// # Returns
  /// This method returns `true` if the stone could be positioned and
  /// `false` if it immediately collided with already merged pieces.
  fn reset_stone(&self, stone: &mut S) -> bool {
    let stone_bounds = stone.bounds();
    let x = self.width() / 2 - stone_bounds.w / 2;
    let y = self.height() - stone_bounds.h;
    let () = stone.move_to(Point::new(x, y));

    !self.collides(stone)
  }

  /// Check whether the given line is complete.
  fn line_complete(&self, line: i16) -> bool;

  /// Remove the given line.
  fn remove_line(&mut self, line: i16);

  /// Remove completed lines in the provided y-range.
  fn remove_complete_lines(&mut self, range: Range<i16>) -> u16 {
    debug_assert!(0 <= range.start && range.end <= self.height(), "{range:?}");

    let mut removed = 0;
    // Remove all completed lines; from top to bottom so that we are
    // unaffected by changes of index to lower lines caused by the
    // removal.
    for line in range.rev() {
      if self.line_complete(line) {
        let () = self.remove_line(line);
        removed += 1;
      }
    }
    removed
  }
}
