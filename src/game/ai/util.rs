// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::Ordering;

use super::super::Matrix;
use super::super::Stonelike;


pub(super) fn parse_matrix(description: &str) -> Matrix<Option<()>> {
  let lines = description.lines().map(str::trim).filter(|s| !s.is_empty());
  let w = lines
    .clone()
    .max_by(|l1, l2| {
      assert_eq!(l1.len(), l2.len(), "`{l1}` vs. `{l2}`");
      Ordering::Equal
    })
    .unwrap()
    .len() as i16;
  let h = lines.clone().count() as i16;

  let mut matrix = Matrix::new(w, h);

  let mut y = h;
  let mut x;

  for line in lines {
    x = 0;
    y -= 1;

    for byte in line.bytes() {
      match byte {
        b'#' => matrix[(x, y)] = Some(()),
        b'.' => (),
        _ => panic!("unexpected character: `{byte}`"),
      }
      x += 1;
    }
  }

  matrix
}


// We use macros here because then:
// 1) `rustfmt` leaves the code alone
// 2) there is no name collision between a variable `field` and a
//    function of the same name, for example
macro_rules! matrix {
  ($matrix:literal) => {{
    $crate::game::ai::util::parse_matrix($matrix)
  }};
}

pub(super) use matrix;


macro_rules! field {
  ($field:literal) => {{
    $crate::game::ai::Field::from_matrix(&$crate::game::ai::util::parse_matrix($field))
  }};
}

pub(super) use field;


macro_rules! stone {
  ($stone:literal) => {{
    $crate::game::ai::Stone::from_matrix(&$crate::game::ai::util::parse_matrix($stone))
  }};
}

pub(super) use stone;


/// Assert that two stones have pieces at the same positions.
#[track_caller]
pub(super) fn assert_stones_eq<S>(stone1: &S, stone2: &S)
where
  S: Stonelike,
{
  let mut pieces1 = stone1.pieces().collect::<Vec<_>>();
  let () = pieces1.sort();

  let mut pieces2 = stone2.pieces().collect::<Vec<_>>();
  let () = pieces2.sort();

  assert_eq!(pieces1, pieces2)
}


#[cfg(test)]
mod tests {
  use super::*;


  /// Check that we can correctly create a `Matrix` object from a
  /// "textual description".
  #[test]
  fn matrix_creation() {
    let matrix = matrix! {"
      ...
      ...
    "};
    assert_eq!(matrix.width(), 3);
    assert_eq!(matrix.height(), 2);

    let matrix = matrix! {"
      ...
      .#.
    "};
    assert_eq!(matrix.width(), 3);
    assert_eq!(matrix.height(), 2);
    assert_eq!(matrix[(0, 0)], None);
    assert_eq!(matrix[(1, 0)], Some(()));
    assert_eq!(matrix[(2, 0)], None);
    assert_eq!(matrix[(0, 1)], None);
    assert_eq!(matrix[(1, 1)], None);
    assert_eq!(matrix[(2, 1)], None);

    let matrix = matrix! {"
      ..#
      #..
    "};
    assert_eq!(matrix.width(), 3);
    assert_eq!(matrix.height(), 2);
    assert_eq!(matrix[(0, 0)], Some(()));
    assert_eq!(matrix[(1, 0)], None);
    assert_eq!(matrix[(2, 0)], None);
    assert_eq!(matrix[(0, 1)], None);
    assert_eq!(matrix[(1, 1)], None);
    assert_eq!(matrix[(2, 1)], Some(()));
  }

  /// Make sure that we cannot create a `Matrix` from a description with
  /// inconsistent width.
  #[test]
  #[should_panic]
  fn matrix_inconsistent_width() {
    let _matrix = matrix! {"
      ....
      ...
    "};
  }
}
