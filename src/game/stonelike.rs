// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::max;
use std::cmp::min;
use std::f32::consts::PI;

use crate::Point;
use crate::Rect;


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


fn rotate<S>(stone: &mut S, left: bool)
where
  S: ?Sized + Stonelike,
{
  let center_x;
  let center_y;

  let bounds = stone.bounds();
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

  let () = stone.pieces_mut().for_each(|location| {
    *location += Point::new(10, 10);
    *location = rotate_point(*location, center, left);
    *location -= Point::new(10, 10);
  });
}


/// A trait representing anything that behaves (and can be treated as) a
/// Tetris stone.
pub(super) trait Stonelike {
  type Piece;

  type PieceIter<'slf>: Iterator<Item = Point<i16>> + DoubleEndedIterator + ExactSizeIterator
  where
    Self: 'slf;

  type PieceIterMut<'slf>: Iterator<Item = &'slf mut Point<i16>>
    + DoubleEndedIterator
    + ExactSizeIterator
  where
    Self: 'slf;

  type IntoPiecesIter: Iterator<Item = (Self::Piece, Point<i16>)>
    + DoubleEndedIterator
    + ExactSizeIterator;

  fn pieces(&self) -> Self::PieceIter<'_>;
  fn pieces_mut(&mut self) -> Self::PieceIterMut<'_>;
  fn into_pieces(self) -> Self::IntoPiecesIter;

  /// Retrieve the stone's bounds.
  fn bounds(&self) -> Rect<i16> {
    let mut pieces = self.pieces();
    // SANITY: Our stone always has at least one piece.
    let location = pieces.next().unwrap();
    let mut x_min = location.x;
    let mut x_max = location.x;
    let mut y_min = location.y;
    let mut y_max = location.y;

    for location in pieces {
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

  /// Move the pieces of the stone by the given x and y amount.
  fn move_by(&mut self, x: i16, y: i16) {
    let () = self.pieces_mut().for_each(|location| {
      location.x += x;
      location.y += y;
    });
  }

  /// Move the stone to the given location.
  fn move_to(&mut self, location: Point<i16>) {
    let bounds = self.bounds();
    let x = location.x - bounds.x;
    let y = location.y - bounds.y;

    self.move_by(x, y)
  }

  /// Move the stone down by one unit.
  #[inline]
  fn move_down(&mut self) {
    self.move_by(0, -1)
  }

  /// Move the stone to the left by one unit.
  #[inline]
  fn move_left(&mut self) {
    self.move_by(-1, 0)
  }

  /// Move the stone to the right by one unit.
  #[inline]
  fn move_right(&mut self) {
    self.move_by(1, 0)
  }

  /// Rotate the stone either left or right by 90°.
  #[inline]
  fn rotate(&mut self, left: bool) {
    rotate(self, left)
  }

  /// Rotate the stone left by 90°.
  #[inline]
  fn rotate_left(&mut self) {
    self.rotate(true)
  }

  /// Rotate the stone right by 90°.
  #[inline]
  fn rotate_right(&mut self) {
    self.rotate(false)
  }
}
