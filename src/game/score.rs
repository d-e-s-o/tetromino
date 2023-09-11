// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later


/// A type helping with keeping track of score in a Tetris game.
pub(super) struct Score {
  /// The current level.
  level: u16,
  /// The number of points earned so far.
  points: u64,
  /// The total number of lines cleared.
  lines: u32,
  /// The lines required to be cleared to level up.
  lines_for_level: u16,
  /// The number of lines cleared since the last level up.
  lines_since_up: u16,
}

impl Score {
  /// Add the given number of lines to the score.
  pub fn add(&mut self, lines: u16) {
    let level = self.level;
    // Strictly speaking the point calculation is wrong: if
    // `lines_for_level_` is a low value (e.g. 1) then the points will
    // be calculated based only on the current level -- though the level
    // would increase after some lines are cleared (e.g. lines is 4 then
    // the increase could be 4 levels). However it should be quite
    // impossible to change this because the points depend on the number
    // of lines cleared and is not limited so we cannot split the
    // calculation, increase the level and continue.
    self.points += self.calculate_points(lines);
    self.lines += u32::from(lines);
    self.lines_since_up += lines;
    self.level += self.lines_since_up / self.lines_for_level;
    self.lines_since_up %= self.lines_for_level;

    if self.level != level {
      println!("{} points @ level {level}", self.points);
    }
  }

  /// This method is used to calculate the number of points for the
  /// given number of cleared lines based on the current level.
  #[inline]
  fn calculate_points(&self, lines: u16) -> u64 {
    (5 * (lines * lines) * self.level).into()
  }

  /// Retrieve the current level.
  #[inline]
  pub fn level(&self) -> u16 {
    self.level
  }
}

impl Default for Score {
  fn default() -> Self {
    Self {
      level: 1,
      points: 0,
      lines: 0,
      lines_for_level: 10,
      lines_since_up: 0,
    }
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Check that we can keep track of scores correctly.
  #[test]
  fn score_counting() {
    let mut score = Score::default();
    assert_eq!(score.level, 1);
    assert_eq!(score.points, 0);
    assert_eq!(score.lines, 0);
    assert_eq!(score.lines_for_level, 10);

    let () = score.add(5);
    assert_eq!(score.level, 1);
    assert_ne!(score.points, 0);
    assert_eq!(score.lines, 5);
    assert_eq!(score.lines_for_level, 10);

    let () = score.add(1);
    assert_eq!(score.level, 1);
    assert_ne!(score.points, 0);
    assert_eq!(score.lines, 6);
    assert_eq!(score.lines_for_level, 10);

    let () = score.add(4);
    assert_eq!(score.level, 2);
    assert_ne!(score.points, 0);
    assert_eq!(score.lines, 10);
    assert_eq!(score.lines_for_level, 10);
  }
}
