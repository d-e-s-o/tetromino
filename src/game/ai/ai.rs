// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#[cfg(debug_assertions)]
use std::rc::Rc;

use super::actions;
use super::search;
use super::Action;
use super::Field;
use super::Stone;


/// A type used for calculating and managing the estimated best actions
/// for playing a game of Tetris.
#[derive(Debug)]
pub(crate) struct AI {
  /// The list of actions to be performed.
  actions: Vec<Action>,
  /// The index of the next action.
  index: usize,
  /// The field that we expect to have once all actions have been
  /// executed.
  #[cfg(debug_assertions)]
  expected_field: Option<Rc<Field>>,
}

impl AI {
  /// Create a new `AI` using the provided `Field`, `Stone`, and
  /// an optional list of preview stones.
  pub(crate) fn new(field: &Field, stone: &Stone, next_stones: &[Stone]) -> Self {
    let best = search(field, stone, next_stones);
    let (actions, _field) = actions(best);

    Self {
      #[cfg(debug_assertions)]
      expected_field: _field,
      actions: actions.collect(),
      index: 0,
    }
  }

  /// Advance to the next stone.
  ///
  /// # Notes
  /// This method assumes (and potentially checks) that `field` is
  /// *actually* the previously calculated field.
  pub fn advance_stone(&mut self, field: &Field, stone: &Stone, next_stones: &[Stone]) {
    #[cfg(debug_assertions)]
    debug_assert_eq!(self.expected_field.as_deref(), Some(field));
    *self = Self::new(field, stone, next_stones);
  }

  /// Take a peek at the next upcoming [`Action`].
  pub fn peek(&self) -> Option<Action> {
    self.actions.get(self.index).copied()
  }
}

impl Iterator for AI {
  type Item = Action;

  fn next(&mut self) -> Option<Self::Item> {
    let action = self.actions.get(self.index);
    self.index = self.index.saturating_add(1);
    action.copied()
  }
}
