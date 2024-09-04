// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::Cell;
use std::cell::RefCell;
use std::iter;
use std::mem::replace;
use std::slice;

use crate::ActiveRenderer as Renderer;
use crate::ColorMode;
use crate::Point;

use super::Stone;
use super::StoneProducer;
use super::Stonelike as _;


/// The space between each stone in pieces.
const SPACE: i16 = 1;


/// A type used for displaying a preview of upcoming stones.
#[derive(Debug)]
pub(super) struct PreviewStones {
  /// The location of the upper left corner of the preview area.
  location: Point<i16>,
  /// The producer we use for creating new stones.
  producer: Box<dyn StoneProducer>,
  /// The upcoming stones.
  stones: RefCell<Box<[Stone]>>,
  /// The index of the next stone to yield.
  index: Cell<u8>,
  /// The color mode in use.
  mode: RefCell<ColorMode<()>>,
}

impl PreviewStones {
  /// Create a new `PreviewStones` object displaying `count` stones.
  pub(super) fn new(location: Point<i16>, count: u8, producer: Box<dyn StoneProducer>) -> Self {
    let stones = (0..count)
      .map(|_| producer.create_stone())
      .collect::<Vec<_>>()
      .into_boxed_slice();

    let slf = Self {
      location,
      producer,
      stones: RefCell::new(stones),
      index: Cell::new(0),
      mode: RefCell::new(ColorMode::default()),
    };

    let () = slf.reposition_stones();
    slf
  }

  /// Replace the currently "active" stone with a new one and adjust the
  /// "active" index by one to make this new stone the last one.
  fn add_new_stone(&self) -> Stone {
    let mut new_stone = self.producer.create_stone();
    let () = new_stone.set_color_mode(*self.mode.borrow());
    let index = usize::from(self.index.get());

    let mut stones = self.stones.borrow_mut();
    let stone = replace(stones.get_mut(index).unwrap(), new_stone);

    let () = self.index.set(((index + 1) % stones.len()) as u8);
    stone
  }

  /// Reposition all stones.
  fn reposition_stones(&self) {
    let mut location = self.location;
    let mut stones = self.stones.borrow_mut();
    let mut index = usize::from(self.index.get());

    for _ in 0..stones.len() {
      let stone = &mut stones[index];
      let bounds = stone.bounds();
      let () = stone.move_to(location - (0, bounds.h).into());

      location.y -= bounds.h + SPACE;
      index = (index + 1) % stones.len();
    }
  }

  /// Render the object.
  pub(super) fn render(&self, renderer: &Renderer) {
    for stone in self.stones.borrow().iter() {
      let () = stone.render(renderer);
    }
  }

  /// Toggle the color mode (light/dark) in use.
  #[inline]
  pub(crate) fn toggle_color_mode(&self) {
    let () = self.mode.borrow_mut().toggle();

    for stone in self.stones.borrow_mut().iter_mut() {
      let () = stone.set_color_mode(*self.mode.borrow());
    }
  }

  /// Perform an operation on the preview stones.
  #[inline]
  pub(super) fn with_stones<F, R>(&self, f: F) -> R
  where
    F: FnOnce(iter::Chain<slice::Iter<Stone>, slice::Iter<Stone>>) -> R,
  {
    let stones = self.stones.borrow();
    let (front, back) = stones.split_at(self.index.get().into());
    f(back.iter().chain(front))
  }

  /// Retrieve the maximum width of the preview stones.
  #[inline]
  pub(super) fn width(&self) -> i16 {
    self.max_dimensions().0
  }

  /// Retrieve the maximum height of the preview stones.
  #[inline]
  pub(super) fn height(&self) -> i16 {
    (self.stones.borrow().len() as i16 + SPACE) * self.max_dimensions().1
  }
}

impl StoneProducer for PreviewStones {
  /// Create a new random stone, based on one of the known templates.
  fn create_stone(&self) -> Stone {
    let stone = self.add_new_stone();
    let () = self.reposition_stones();
    stone
  }

  fn max_dimensions(&self) -> (i16, i16) {
    self.producer.max_dimensions()
  }
}
