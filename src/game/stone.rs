// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::max;
use std::cmp::min;
use std::f32::consts::PI;

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Point;
use crate::Rect;
use crate::Texture;

use super::Piece;


#[inline]
fn deg_to_rad(x: f32) -> f32 {
  x * PI * 1.0 / 180.0
}

fn rotate_point_by(point: Point<f32>, angle: f32) -> Point<f32> {
  // Get the distance from origin.
  let distance = (point.x * point.x + point.y * point.y).sqrt();

  if distance > 0.0 {
    // Calculate the angle the stone has currently and add the angle to
    // rotate to the old one.
    let new_angle = if point.x < 0.0 { -1.0 } else { 1.0 } * (point.y / distance).acos() + angle;

    let x = new_angle.sin() * distance;
    let y = new_angle.cos() * distance;

    Point::new(x, y)
  } else {
    point
  }
}

/// Rotate a point around a center.
///
/// This function rotates a given point around a center by 90°, either
/// left or right. The provided point has integer coordinates that are
/// understood to be in game dimensions. The function furthermore
/// assumes that the point actually represents the lower left corner of
/// a 1-unit square, while rotation happens based on the center of said
/// square.
fn rotate_point(point: Point<i16>, center: Point<f32>, left: bool) -> Point<i16> {
  let angle = if left { -90.0 } else { 90.0 };

  let mut point = point.into_other::<f32>();
  point += Point::new(0.5, 0.5);
  point = rotate_point_by(point - center, deg_to_rad(angle)) + center;
  point -= Point::new(0.5, 0.5);

  Point::new(point.x as i16, point.y as i16)
}


/// The representation of a Tetris stone.
#[derive(Debug)]
pub(crate) struct Stone {
  /// The texture to use for an individual piece.
  piece_texture: Texture,
  /// The individual pieces making up the stone and their locations.
  /// Typically a stone has four pieces, but that's not set in stone.
  pieces: Vec<(Piece, Point<i16>)>,
}

impl Stone {
  pub(crate) fn new(piece_texture: Texture, template: &[Point<i8>], color: Color) -> Self {
    assert!(!template.is_empty(), "provided stone template is empty");

    Self {
      piece_texture,
      pieces: template
        .iter()
        .map(|p| (Piece::new(color), p.into_other()))
        .collect(),
    }
  }

  pub(crate) fn render(&self, renderer: &Renderer) {
    let _guard = renderer.set_texture(&self.piece_texture);

    let () = self
      .pieces
      .iter()
      .for_each(|(piece, location)| piece.render(renderer, *location));
  }

  pub(crate) fn move_by(&mut self, x: i16, y: i16) {
    let () = self.pieces.iter_mut().for_each(|(_piece, location)| {
      location.x += x;
      location.y += y;
    });
  }

  pub(crate) fn move_to(&mut self, location: Point<i16>) {
    let bounds = self.bounds();
    let x = location.x - bounds.x;
    let y = location.y - bounds.y;

    self.move_by(x, y)
  }

  pub(crate) fn rotate(&mut self, left: bool) {
    let center_x;
    let center_y;

    let bounds = self.bounds();
    let w = bounds.w;
    let h = bounds.h;
    let bounds = bounds.into_other::<f32>();

    if left {
      center_x = 0.5 * bounds.w;
      center_y = 0.5 * bounds.h + if h & 1 == 0 { 0.0 } else { 0.5 };
    } else {
      center_x = 0.5 * bounds.w + if w & 1 == 0 { 0.5 } else { 0.0 };
      center_y = 0.5 * bounds.h;
    }

    // TODO: For now we need to add a constant value here before we
    //       rotate -- this is necessary because the rotation code is
    //       somewhat broken for values close to zero -- fix this!
    let center = Point::new(bounds.x + center_x + 10.0, bounds.y + center_y + 10.0);

    let () = self.pieces.iter_mut().for_each(|(_piece, location)| {
      *location += Point::new(10, 10);
      *location = rotate_point(*location, center, left);
      *location -= Point::new(10, 10);
    });
  }

  pub(crate) fn bounds(&self) -> Rect<i16> {
    // SANITY: Our stone always has at least one piece.
    let (_piece, location) = self.pieces.first().unwrap();
    let mut x_min = location.x;
    let mut x_max = location.x;
    let mut y_min = location.y;
    let mut y_max = location.y;

    for (_piece, location) in self.pieces.iter().skip(1) {
      x_min = min(x_min, location.x);
      x_max = max(x_max, location.x);
      y_min = min(y_min, location.y);
      y_max = max(y_max, location.y);
    }

    Rect {
      x: x_min,
      y: y_min,
      w: x_max + 1 - x_min,
      h: y_max + 1 - y_min,
    }
  }

  pub(crate) fn pieces(
    &self,
  ) -> impl Iterator<Item = Point<i16>> + DoubleEndedIterator + ExactSizeIterator + '_ {
    self.pieces.iter().map(|(_piece, location)| *location)
  }

  pub(crate) fn into_pieces(
    self,
  ) -> impl Iterator<Item = (Piece, Point<i16>)> + DoubleEndedIterator + ExactSizeIterator {
    self.pieces.into_iter()
  }
}


#[cfg(test)]
mod tests {
  use super::*;


  fn new_stone(template: &[Point<i8>]) -> Stone {
    Stone::new(Texture::invalid(), template, Color::black())
  }


  /// Check that the bounds of a `Stone` are calculated correctly.
  #[test]
  fn stone_bounds() {
    let template = [Point::new(1, 2)];
    let bounds = new_stone(&template).bounds();
    assert_eq!(bounds.x, 1);
    assert_eq!(bounds.y, 2);
    assert_eq!(bounds.w, 1);
    assert_eq!(bounds.h, 1);

    let template = [Point::new(1, 2), Point::new(3, 2)];
    let bounds = new_stone(&template).bounds();
    assert_eq!(bounds.x, 1);
    assert_eq!(bounds.y, 2);
    assert_eq!(bounds.w, 3);
    assert_eq!(bounds.h, 1);

    let template = [Point::new(1, 2), Point::new(0, 1)];
    let bounds = new_stone(&template).bounds();
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 1);
    assert_eq!(bounds.w, 2);
    assert_eq!(bounds.h, 2);

    let template = [
      Point::new(0, 0),
      Point::new(0, 1),
      Point::new(1, 0),
      Point::new(1, 1),
    ];
    let bounds = new_stone(&template).bounds();
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 0);
    assert_eq!(bounds.w, 2);
    assert_eq!(bounds.h, 2);
  }

  /// Check that we can move a `Stone` object as expected.
  #[test]
  fn stone_movement() {
    let template = [Point::new(1, 2), Point::new(0, 1)];
    let mut stone = new_stone(&template);
    let bounds = stone.bounds();
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 1);
    assert_eq!(bounds.w, 2);
    assert_eq!(bounds.h, 2);

    let () = stone.move_to(Point::new(3, 4));
    let bounds = stone.bounds();
    assert_eq!(bounds.x, 3);
    assert_eq!(bounds.y, 4);

    let () = stone.move_to(Point::new(0, 0));
    let bounds = stone.bounds();
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 0);
    assert_eq!(bounds.w, 2);
    assert_eq!(bounds.h, 2);
  }

  /// Check that we can move a `Stone` object as expected.
  #[test]
  fn stone_rotation() {
    // T stone
    let template = [
      Point::new(0, 0),
      Point::new(1, 0),
      Point::new(1, 1),
      Point::new(2, 0),
    ];
    let mut stone = new_stone(&template);
    let () = stone.move_to(Point::new(6, 4));
    let before = stone.pieces().collect::<Vec<_>>();
    let () = stone.rotate(true);
    let after = stone.pieces().collect::<Vec<_>>();

    assert_ne!(before, after);
  }
}
