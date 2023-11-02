// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use super::super::Matrix;
use super::super::Stonelike;
use super::Orientations;
use super::Stone;


/// For the sake of our search, stones may be moved to invalid positions
/// and be added to the set of "visited" stones despite being at such an
/// invalid location. However, our invariant is that we always make at
/// most one such invalid move. As a result, we just need to adjust
/// dimensions of the matrix by one on each side.
const OFFSET: i16 = 1;


/// An object of this type represents the set of visited (i.e., already
/// examined) stones at particular positions and orientations.
#[derive(Debug)]
pub(super) struct VisitedStones {
  field: Matrix<Orientations>,
}

impl VisitedStones {
  #[inline]
  pub fn new(width: i16, height: i16) -> Self {
    Self {
      field: Matrix::new(width + 2 * OFFSET, height + 2 * OFFSET),
    }
  }

  #[inline]
  pub fn visit(&mut self, stone: &Stone) {
    debug_assert!(!self.contains(stone));

    *self.entry_mut(stone) |= stone.orientation();
  }

  #[inline]
  pub fn contains(&self, stone: &Stone) -> bool {
    self.entry(stone).contains(stone.orientation())
  }

  #[inline]
  fn entry(&self, stone: &Stone) -> &Orientations {
    let bounds = stone.bounds();
    let x = OFFSET + bounds.x;
    let y = OFFSET + bounds.y;
    &self.field[(x, y)]
  }

  #[inline]
  fn entry_mut(&mut self, stone: &Stone) -> &mut Orientations {
    let bounds = stone.bounds();
    let x = OFFSET + bounds.x;
    let y = OFFSET + bounds.y;
    &mut self.field[(x, y)]
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use super::super::util::stone;


  /// Make sure that we can add a stone to a [`VisitedStones`] object and
  /// that we can later check whether we've seen this stone already.
  #[test]
  fn addition_and_check() {
    let mut stone = stone! {"
      #.
      #.
      ##
    "};

    let mut visited = VisitedStones::new(3, 4);
    assert!(!visited.contains(&stone));

    let () = visited.visit(&stone);
    assert!(visited.contains(&stone));

    let () = stone.rotate_left();
    assert!(!visited.contains(&stone));

    let () = stone.rotate_right();
    assert!(visited.contains(&stone));

    let () = stone.move_right();
    assert!(!visited.contains(&stone));
  }
}
