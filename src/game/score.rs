// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Write as _;
use std::mem::MaybeUninit;
use std::rc::Rc;

use bufio::Writer as StackWriter;

use crate::ActiveRenderer as Renderer;
use crate::Change;
use crate::Color;
use crate::Font;
use crate::Point;
use crate::Rect;
use crate::Texture;

/// The font size to use, in game units.
const FONT_SIZE: i16 = 2;

static LEVEL_STR: &[u8] = b"Level:";
static POINTS_STR: &[u8] = b"Points:";
static LINES_STR: &[u8] = b"Lines:";
static FIXED_STRS: [&[u8]; 3] = [LEVEL_STR, POINTS_STR, LINES_STR];

/// The pre-calculated maximum width of the strings above when rendered
/// using `Font::builtin`.
const MAX_FIXED_STR_WIDTH: i16 = 33;
/// The pre-calculated maximum width of any single digit (0-9) when
/// rendered using `Font::builtin`.
const MAX_DIGIT_WIDTH: i16 = 6;


/// Calculate the digits in a number (when represented as decimal).
#[inline]
fn digits(x: u64) -> i16 {
  if x == 0 {
    1
  } else {
    i16::try_from(x.ilog10()).unwrap() + 1
  }
}


/// A type helping with keeping track of score in a Tetris game.
#[derive(Debug)]
pub(super) struct Score {
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
  /// The texture to use for each and every "pixel" of each rendered
  /// glyph.
  texture: Rc<Texture>,
}

impl Score {
  pub fn new(start_level: u16, lines_for_level: u16, texture: Rc<Texture>) -> Self {
    Self {
      start_level,
      level: start_level,
      points: 0,
      lines: 0,
      lines_for_level,
      lines_since_up: 0,
      font: Font::builtin(),
      texture,
    }
  }

  /// Render the object.
  pub fn render(&self, renderer: &Renderer) {
    let factor = f32::from(FONT_SIZE) / f32::from(self.font.size());
    let render_pixel = |point: Point<i16>| {
      let () = renderer.render_rect_f32(Rect::new(
        f32::from(point.x) * factor,
        f32::from(point.y) * factor,
        factor,
        factor,
      ));
    };

    let _guard = renderer.set_color(Color::orange());
    let _guard = renderer.set_texture(&self.texture);

    {
      let () = self.font.render_str(LEVEL_STR, render_pixel);

      let _guard = renderer.set_origin(Point::new(0, -FONT_SIZE));
      let () = self.font.render_str(POINTS_STR, render_pixel);

      let _guard = renderer.set_origin(Point::new(0, -FONT_SIZE));
      let () = self.font.render_str(LINES_STR, render_pixel);
    }

    // 256 bytes of stack buffer ought to be enough to format all the
    // strings we care about, with a rather large margin.
    let mut buffer = [MaybeUninit::<u8>::uninit(); 256];
    let mut writer = StackWriter::new(&mut buffer);

    let _guard = renderer.set_origin(Point::new(
      (f32::from(MAX_FIXED_STR_WIDTH) * factor).ceil() as i16,
      0,
    ));
    let () = write!(writer, "{}", self.level).unwrap();
    let string = writer.written();
    let () = self.font.render_str(string, render_pixel);

    let _guard = renderer.set_origin(Point::new(0, -FONT_SIZE));
    let () = writer.reset();
    let () = write!(writer, "{}", self.points).unwrap();
    let string = writer.written();
    let () = self.font.render_str(string, render_pixel);

    let _guard = renderer.set_origin(Point::new(0, -FONT_SIZE));
    let () = writer.reset();
    let () = write!(writer, "{}", self.lines).unwrap();
    let string = writer.written();
    let () = self.font.render_str(string, render_pixel);
  }

  fn with_dyn_str_change<F>(score: &mut Score, mut f: F) -> Change
  where
    F: FnMut(&mut Score),
  {
    let before_len = score.dyn_str_len();
    let () = f(score);
    let after_len = score.dyn_str_len();

    if before_len != after_len {
      Change::Resize
    } else {
      Change::Changed
    }
  }

  /// Add the given number of lines to the score.
  pub fn add(&mut self, lines: u16) -> Change {
    Self::with_dyn_str_change(self, |slf| {
      // Strictly speaking the point calculation is wrong: if
      // `lines_for_level_` is a low value (e.g. 1) then the points will
      // be calculated based only on the current level -- though the level
      // would increase after some lines are cleared (e.g. lines is 4 then
      // the increase could be 4 levels). However it should be quite
      // impossible to change this because the points depend on the number
      // of lines cleared and is not limited so we cannot split the
      // calculation, increase the level and continue.
      slf.points += slf.calculate_points(lines);
      slf.lines += u32::from(lines);
      slf.lines_since_up += lines;
      slf.level += slf.lines_since_up / slf.lines_for_level;
      slf.lines_since_up %= slf.lines_for_level;
    })
  }

  /// This method is used to calculate the number of points for the
  /// given number of cleared lines based on the current level.
  #[inline]
  fn calculate_points(&self, lines: u16) -> u64 {
    (5 * (lines * lines) * self.level).into()
  }

  /// Reset the `Score`'s state to its initial value.
  pub fn reset(&mut self) -> Change {
    Self::with_dyn_str_change(self, |slf| {
      slf.level = slf.start_level;
      slf.lines = 0;
      slf.points = 0;
      slf.lines_since_up = 0;
    })
  }

  fn dyn_str_len(&self) -> i16 {
    // If the fixed strings are changed the calculation below will
    // likely also need to be adjusted.
    debug_assert_eq!(FIXED_STRS.len(), 3);

    [u64::from(self.level), self.points, u64::from(self.lines)]
      .into_iter()
      .map(digits)
      .max()
      .unwrap_or_default()
  }

  /// Calculate the width of the score board.
  pub fn width(&self) -> i16 {
    let factor = f32::from(FONT_SIZE) / f32::from(self.font.size());

    let statc = (f32::from(MAX_FIXED_STR_WIDTH) * factor).ceil() as i16;
    let dynmc = (f32::from(self.dyn_str_len() * MAX_DIGIT_WIDTH) * factor).ceil() as i16;

    statc + dynmc
  }

  /// Retrieve the start level.
  #[inline]
  pub fn start_level(&self) -> u16 {
    self.start_level
  }

  /// Retrieve the lines required to be cleared to level up.
  #[inline]
  pub fn lines_for_level(&self) -> u16 {
    self.lines_for_level
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

  use std::rc::Rc;

  use test_fork::fork;

  use crate::gl::empty_texture;
  use crate::winit::with_opengl_context;


  /// Verify that our [`digit`] functions work correctly on a bunch of
  /// inputs.
  #[test]
  fn digit_calculation() {
    assert_eq!(digits(0), 1);
    assert_eq!(digits(1), 1);
    assert_eq!(digits(5), 1);
    assert_eq!(digits(9), 1);
    assert_eq!(digits(10), 2);
    assert_eq!(digits(42), 2);
    assert_eq!(digits(99), 2);
    assert_eq!(digits(100), 3);
    assert_eq!(digits(999), 3);
    assert_eq!(digits(1_000), 4);
    assert_eq!(digits(10_000), 5);
    assert_eq!(digits(1_000_000), 7);
    assert_eq!(digits(12_345_678_901), 11);
    assert_eq!(digits(u64::MAX), 20);
  }

  /// Check that our pre-calculated `FIXED_STR_MAX_WIDTH` constant is
  /// correct.
  #[test]
  fn max_fixed_string_width_checking() {
    let font = Font::builtin();
    let max_w = FIXED_STRS.iter().map(|s| font.str_width(s)).max().unwrap();

    assert_eq!(max_w, MAX_FIXED_STR_WIDTH);
  }

  /// Test that our `MAX_DIGIT_WIDTH` constant matches reality.
  #[test]
  fn max_digit_width_checking() {
    let font = Font::builtin();
    let max_w = (b'0'..=b'9').map(|b| font.str_width(&[b])).max().unwrap();

    assert_eq!(max_w, MAX_DIGIT_WIDTH);
  }

  /// Check that we can keep track of scores correctly.
  #[fork]
  #[test]
  fn score_counting() {
    with_opengl_context(|context| {
      let texture = Rc::new(empty_texture(context).unwrap());
      let mut score = Score::new(1, 10, texture);
      assert_eq!(score.level, 1);
      assert_eq!(score.points, 0);
      assert_eq!(score.lines, 0);
      assert_eq!(score.lines_for_level, 10);

      let change = score.add(5);
      assert_eq!(change, Change::Resize);
      assert_eq!(score.level, 1);
      assert_eq!(score.points, 125);
      assert_eq!(score.lines, 5);
      assert_eq!(score.lines_for_level, 10);

      let change = score.add(1);
      assert_eq!(change, Change::Changed);
      assert_eq!(score.level, 1);
      assert_eq!(score.points, 130);
      assert_eq!(score.lines, 6);
      assert_eq!(score.lines_for_level, 10);

      let _change = score.add(4);
      assert_eq!(change, Change::Changed);
      assert_eq!(score.level, 2);
      assert_eq!(score.points, 210);
      assert_eq!(score.lines, 10);
      assert_eq!(score.lines_for_level, 10);
    })
  }
}
