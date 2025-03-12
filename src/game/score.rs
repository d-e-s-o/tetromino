// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Write as _;
use std::mem::MaybeUninit;

use bufio::Writer as StackWriter;

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Font;
use crate::Point;

/// The font size to use, in game units.
const FONT_SIZE: f32 = 2.0;


/// A type helping with keeping track of score in a Tetris game.
#[derive(Debug)]
pub(super) struct Score {
  /// The location of the upper left corner of the score board.
  location: Point<i16>,
  /// The starting level.
  start_level: u16,
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
  /// The font to use for rendering the current score.
  font: Font,
}

impl Score {
  pub(super) fn new(
    location: Point<i16>,
    start_level: u16,
    lines_for_level: u16,
    font: Font,
  ) -> Self {
    Self {
      location,
      start_level,
      level: start_level,
      points: 0,
      lines: 0,
      lines_for_level,
      lines_since_up: 0,
      font,
    }
  }

  /// Render the object.
  pub fn render(&self, renderer: &Renderer) {
    let _guard = renderer.set_color(Color::orange());

    let location = self.location.into_other::<f32>();

    let (level_w, level_h) = self.font.render_str(
      renderer,
      location + Point::new(0.0, 0.0),
      b"Level:  ",
      FONT_SIZE,
    );

    let (points_w, points_h) = self.font.render_str(
      renderer,
      location + Point::new(0.0, -level_h),
      b"Points:  ",
      FONT_SIZE,
    );

    let (lines_w, _lines_h) = self.font.render_str(
      renderer,
      location + Point::new(0.0, -level_h - points_h),
      b"Lines:  ",
      FONT_SIZE,
    );

    let w = level_w.max(points_w).max(lines_w);

    // 256 bytes of stack buffer ought to be enough to format all the
    // strings we care about, with a rather large margin.
    let mut buffer = [MaybeUninit::<u8>::uninit(); 256];
    let mut writer = StackWriter::new(&mut buffer);

    let () = write!(writer, "{}", self.level).unwrap();
    let string = writer.written();
    let (_w, _h) = self
      .font
      .render_str(renderer, location + Point::new(w, 0.0), string, FONT_SIZE);

    let () = writer.reset();
    let () = write!(writer, "{}", self.points).unwrap();
    let string = writer.written();
    let (_w, _h) = self.font.render_str(
      renderer,
      location + Point::new(w, -level_h),
      string,
      FONT_SIZE,
    );

    let () = writer.reset();
    let () = write!(writer, "{}", self.lines).unwrap();
    let string = writer.written();
    let (_w, _h) = self.font.render_str(
      renderer,
      location + Point::new(w, -level_h - points_h),
      string,
      FONT_SIZE,
    );
  }

  /// Add the given number of lines to the score.
  pub fn add(&mut self, lines: u16) {
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
  }

  /// This method is used to calculate the number of points for the
  /// given number of cleared lines based on the current level.
  #[inline]
  fn calculate_points(&self, lines: u16) -> u64 {
    (5 * (lines * lines) * self.level).into()
  }

  /// Reset the `Score`'s state to its initial value.
  pub fn reset(&mut self) {
    self.level = self.start_level;
    self.lines = 0;
    self.points = 0;
    self.lines_since_up = 0;
  }

  /// Retrieve the current level.
  #[inline]
  pub fn level(&self) -> u16 {
    self.level
  }

  /// Retrieve the current points.
  #[inline]
  pub fn points(&self) -> u64 {
    self.points
  }

  /// Retrieve the number of lines cleared to far.
  #[inline]
  pub fn lines(&self) -> u32 {
    self.lines
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use crate::Texture;


  /// Check that we can keep track of scores correctly.
  #[test]
  fn score_counting() {
    let font = Font::builtin(Texture::invalid());
    let mut score = Score::new(Point::new(0, 0), 1, 10, font);
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
