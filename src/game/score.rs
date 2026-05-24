// Copyright (C) 2023-2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::io::Write as _;
use std::mem::MaybeUninit;
use std::rc::Rc;

use bufio::Writer as StackWriter;

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Font;
use crate::Point;
use crate::Rect;
use crate::Texture;

/// The font size to use, in game units.
const FONT_SIZE: i16 = 2;


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
  /// The texture to use for each and every "pixel" of each rendered
  /// glyph.
  texture: Rc<Texture>,
}

impl Score {
  pub(super) fn new(
    location: Point<i16>,
    start_level: u16,
    lines_for_level: u16,
    font: Font,
    texture: Rc<Texture>,
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

    let w = {
      let _guard = renderer.set_origin(self.location);
      let level_w = self.font.render_str(b"Level:  ", render_pixel);

      let _guard = renderer.set_origin(Point::new(0, -FONT_SIZE));
      let points_w = self.font.render_str(b"Points:  ", render_pixel);

      let _guard = renderer.set_origin(Point::new(0, -FONT_SIZE));
      let lines_w = self.font.render_str(b"Lines:  ", render_pixel);

      level_w.max(points_w).max(lines_w)
    };

    // 256 bytes of stack buffer ought to be enough to format all the
    // strings we care about, with a rather large margin.
    let mut buffer = [MaybeUninit::<u8>::uninit(); 256];
    let mut writer = StackWriter::new(&mut buffer);

    let _guard = renderer.set_origin(self.location + Point::new((f32::from(w) * factor) as i16, 0));
    let () = write!(writer, "{}", self.level).unwrap();
    let string = writer.written();
    let _w = self.font.render_str(string, render_pixel);

    let _guard = renderer.set_origin(Point::new(0, -FONT_SIZE));
    let () = writer.reset();
    let () = write!(writer, "{}", self.points).unwrap();
    let string = writer.written();
    let _w = self.font.render_str(string, render_pixel);

    let _guard = renderer.set_origin(Point::new(0, -FONT_SIZE));
    let () = writer.reset();
    let () = write!(writer, "{}", self.lines).unwrap();
    let string = writer.written();
    let _w = self.font.render_str(string, render_pixel);
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

  use std::rc::Rc;

  use test_fork::fork;

  use crate::gl::empty_texture;
  use crate::winit::with_opengl_context;


  /// Check that we can keep track of scores correctly.
  #[fork]
  #[test]
  fn score_counting() {
    with_opengl_context(|context| {
      let texture = Rc::new(empty_texture(context).unwrap());
      let font = Font::builtin();
      let mut score = Score::new(Point::new(0, 0), 1, 10, font, texture);
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
    })
  }
}
