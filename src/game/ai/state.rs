// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;
use std::cmp::Ordering;
use std::ops::Deref as _;
use std::rc::Rc;

use super::super::Fieldlike as _;
use super::super::Stonelike as _;
use super::Action;
use super::Cost;
use super::Field;
use super::Orientation;
use super::Stone;
use super::VisitedStones;


/// Perform an action on a stone.
fn perform_action(action: Action, mut stone: Stone) -> Stone {
  let () = match action {
    Action::Merge => (),
    Action::MoveDown => stone.move_down(),
    Action::MoveLeft => stone.move_left(),
    Action::MoveRight => stone.move_right(),
    Action::RotateLeft => stone.rotate_left(),
    Action::RotateRight => stone.rotate_right(),
  };
  stone
}

fn cost_for_action(action: Action, field: &Field) -> Cost {
  let width = f32::from(field.width());
  let height = f32::from(field.height());
  let piece_capacity = width * height;

  match action {
    Action::Merge => panic!(),
    Action::MoveDown => Cost::from(1.0 / piece_capacity),
    Action::RotateLeft | Action::RotateRight => Cost::from(2.0 / piece_capacity),
    Action::MoveLeft | Action::MoveRight => Cost::from(2.0 / piece_capacity),
  }
}


/// A type helping with the "expansion" of `State` objects.
#[derive(Clone)]
pub(super) enum Expander {
  MoveDown(Rc<State>),
  MoveLeft(Rc<State>),
  MoveRight(Rc<State>),
  RotateLeft(Rc<State>),
  RotateRight(Rc<State>),
  Done,
}

impl Expander {
  #[inline]
  fn expand(state: Rc<State>) -> Self {
    debug_assert!(!state.has_collision());

    Self::MoveDown(state)
  }
}

impl Iterator for Expander {
  type Item = Rc<State>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      // The basic principle here is as follows: based on the last move
      // action performed we decide what further move actions are valid; if
      // our last move action was a move down all bets are off; if the last
      // move was move to the right there is no use in moving left at all;
      // similarly for a left move
      match self.clone() {
        Self::MoveDown(state) => {
          *self = Self::MoveLeft(Rc::clone(&state));
          break state.derive(Action::MoveDown)
        },
        Self::MoveLeft(state) => {
          *self = Self::MoveRight(Rc::clone(&state));
          if state.action != Some(Action::MoveRight) {
            break state.derive(Action::MoveLeft)
          }
        },
        Self::MoveRight(state) => {
          *self = Self::RotateLeft(Rc::clone(&state));
          if state.action != Some(Action::MoveLeft) {
            break state.derive(Action::MoveRight)
          }
        },
        Self::RotateLeft(state) => {
          *self = Self::RotateRight(Rc::clone(&state));

          let stone = state.stone.as_ref()?;
          if stone.orientation() != Orientation::Rotated180 {
            // If we are rotated by 90° we got rotated to the right once
            // in which case we do not want to rotate left -- in the
            // remaining cases (0° and 270°) we do.
            if stone.orientation() != Orientation::Rotated90 {
              break state.derive(Action::RotateLeft)
            }
          }
        },
        Self::RotateRight(state) => {
          *self = Self::Done;

          let stone = state.stone.as_ref()?;
          if stone.orientation() != Orientation::Rotated180 {
            // If we are rotated by 270° we got rotated to the left once
            // in which case we do not want to rotate right -- in the
            // remaining cases (0° and 90°) we do.
            if stone.orientation() != Orientation::Rotated270 {
              break state.derive(Action::RotateRight)
            }
          }
        },
        Self::Done => break None,
      }
    }
  }
}


/// An object representing one state in the search for the next action
/// to perform on a [`Stone`].
///
/// In abstract terms, a [`State`] is comprised mainly of a [`Field`]
/// and a [`Stone`]. The `Stone` is moved by [deriving][Self::derive] a
/// new `State` by performing an [`Action`] on the `Stone`.
///
/// Once no other movement is possible or desired, the `State` should be
/// [merged][Self::merge] and a new `Field` and `Stone` be set. This
/// process repeats.
///
/// If all preview stones are exhausted the search can eventually be
/// concluded and the `State` be [finalized][Self::finalize].
///
/// `State` objects form a list via their parent `State`. Once finalized
/// this list can be traversed and the set of actions necessary to reach
/// this final state be extracted for later replay on the "live" objects.
#[derive(Debug)]
#[repr(align(64))]
pub(super) struct State {
  /// The parent state, i.e., the one this state was created from by
  /// applying the given action.
  pub parent: Option<Rc<Self>>,
  pub field: Rc<Field>,
  /// The stone currently being moved. If not present the state is
  /// considered "final".
  pub stone: Option<Stone>,
  pub visited: Rc<RefCell<VisitedStones>>,
  /// The index of the next stone to evaluate. The index is incremented
  /// on every [`update`][Self::update]. It has no meaning to the
  /// `State` itself, but is logically associated in it.
  ///
  /// As a conjecture of using `u8`, we cannot handle more than 255
  /// preview stones.
  pub index: u8,
  /// The total number of stones we may ever visit.
  pub count: u8,
  pub action: Option<Action>,
  /// The state's cost, which is basically the cost of actions for
  /// getting to the state, the utility of working with stones further
  /// down the line (which bring us closer towards the goal of
  /// evaluating all of the known next stones), and the utility of the
  /// field.
  pub action_cost: Cost,
  pub field_cost: Cost,
  pub stone_cost: Cost,
}

impl State {
  pub fn initial(field: Field, cost: Cost, stone: Stone, count: u8) -> Rc<Self> {
    debug_assert!(!field.collides(&stone));

    let slf = Self {
      parent: None,
      visited: Rc::new(RefCell::new(VisitedStones::new(
        field.width(),
        field.height(),
      ))),
      field: Rc::new(field),
      stone: Some(stone),
      index: 0,
      count,
      action: None,
      action_cost: Cost::none(),
      field_cost: cost,
      stone_cost: Cost::from(count),
    };

    Rc::new(slf)
  }

  pub fn derive(self: &Rc<Self>, action: Action) -> Option<Rc<Self>> {
    debug_assert!(!self.has_collision());

    let Self {
      parent: _,
      field,
      stone,
      visited,
      index,
      count,
      action: _,
      action_cost,
      field_cost,
      stone_cost,
    } = self.deref();

    let slf = Self {
      parent: Some(Rc::clone(self)),
      field: Rc::clone(field),
      stone: Some(perform_action(action, stone.as_ref()?.clone())),
      visited: Rc::clone(visited),
      index: *index,
      count: *count,
      action: Some(action),
      action_cost: *action_cost + cost_for_action(action, field),
      field_cost: *field_cost,
      stone_cost: *stone_cost,
    };

    // Action cost should be increasing as we make more actions.
    debug_assert!(*action_cost < slf.action_cost);

    Some(Rc::new(slf))
  }

  pub fn merged(self: &Rc<Self>, field: Field, cost: Cost, stone: Stone) -> Rc<Self> {
    debug_assert!(!self.has_collision());

    let Self {
      parent: _,
      field: _,
      stone: _,
      visited: _,
      index,
      count,
      action: _,
      action_cost,
      field_cost: _,
      stone_cost,
    } = self.deref();

    let index = index + 1;

    let slf = Self {
      parent: Some(Rc::clone(self)),
      visited: Rc::new(RefCell::new(VisitedStones::new(
        field.width(),
        field.height(),
      ))),
      field: Rc::new(field),
      stone: Some(stone),
      index,
      count: *count,
      action: Some(Action::Merge),
      action_cost: *action_cost,
      field_cost: cost,
      stone_cost: Cost::from(count - index),
    };

    // Stone cost should be decreasing as we move on to new stones.
    debug_assert!(*stone_cost > slf.stone_cost);

    Rc::new(slf)
  }

  pub fn finalize(self: &Rc<Self>, field: Field, cost: Cost) -> Rc<Self> {
    debug_assert!(!self.has_collision());

    let Self {
      parent: _,
      field: _,
      stone: _,
      visited,
      index,
      count,
      action: _,
      action_cost,
      field_cost: _,
      stone_cost,
    } = self.deref();

    let slf = Self {
      parent: Some(Rc::clone(self)),
      visited: Rc::clone(visited),
      field: Rc::new(field),
      stone: None,
      index: *index,
      count: *count,
      action: Some(Action::Merge),
      action_cost: *action_cost,
      field_cost: cost,
      stone_cost: *stone_cost,
    };

    Rc::new(slf)
  }

  pub fn reparent(self: &Rc<Self>, parent: Option<Rc<Self>>) -> Rc<Self> {
    let Self {
      parent: _,
      field,
      stone,
      visited,
      index,
      count,
      action,
      action_cost,
      field_cost,
      stone_cost,
    } = self.deref();

    let slf = Self {
      parent,
      field: Rc::clone(field),
      stone: stone.clone(),
      visited: Rc::clone(visited),
      index: *index,
      count: *count,
      action: *action,
      action_cost: *action_cost,
      field_cost: *field_cost,
      stone_cost: *stone_cost,
    };

    Rc::new(slf)
  }

  /// "Expand" this `State` into all possible derivations.
  #[inline]
  pub fn expand(self: &Rc<Self>) -> Expander {
    Expander::expand(Rc::clone(self))
  }

  /// Mark the [`State`] as visited.
  #[inline]
  pub fn visit(&self) {
    debug_assert!(!self.was_visited());
    if let Some(stone) = &self.stone {
      let () = self.visited.borrow_mut().visit(stone);
    }
  }

  /// Check whether this [`State`] was already visited.
  #[inline]
  pub fn was_visited(&self) -> bool {
    if let Some(stone) = &self.stone {
      self.visited.borrow().contains(stone)
    } else {
      false
    }
  }

  /// Check whether this [`State`] has a collision.
  #[inline]
  pub fn has_collision(&self) -> bool {
    if let Some(stone) = &self.stone {
      self.field.collides(stone)
    } else {
      false
    }
  }

  /// Retrieve this [`State`]'s accumulated (total) cost estimate.
  #[inline]
  pub fn cost(&self) -> Cost {
    self.action_cost + self.field_cost + self.stone_cost
  }
}

impl PartialEq for State {
  fn eq(&self, other: &Self) -> bool {
    self.cmp(other) == Ordering::Equal
  }
}

impl Eq for State {}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    // We want states to be ordered by "minimum cost" because they'll be
    // used in conjunction with a max-heap. So invert the cost
    // comparison.
    self.cost().cmp(&other.cost()).reverse()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::mem::align_of;
  use std::mem::size_of;

  use super::super::util::assert_stones_eq;
  use super::super::util::field;
  use super::super::util::stone;


  /// Make sure that a `State` instance fits into a single cache line.
  #[test]
  fn state_size() {
    assert_eq!(align_of::<State>(), 64);
    assert_eq!(size_of::<State>(), 64);
  }

  /// Check that we can correctly expand a `State`.
  #[test]
  fn expansion() {
    let mut stone = stone! {"
      ...#.
      ..###
    "};
    let field = field! {"
      .......
      #......
      ##.....
      ##...##
    "};
    assert!(field.reset_stone(&mut stone));

    let state = State::initial(field, Cost::none(), stone.clone(), 0);
    let mut expanded = state.expand();

    let next = expanded.next().unwrap();
    let mut moved_down = stone.clone();
    let () = moved_down.move_down();
    assert_stones_eq(next.stone.as_ref().unwrap(), &moved_down);

    let next = expanded.next().unwrap();
    let mut moved_left = stone.clone();
    let () = moved_left.move_left();
    assert_stones_eq(next.stone.as_ref().unwrap(), &moved_left);

    let next = expanded.next().unwrap();
    let mut moved_right = stone.clone();
    let () = moved_right.move_right();
    assert_stones_eq(next.stone.as_ref().unwrap(), &moved_right);

    let next = expanded.next().unwrap();
    let mut rotated_left = stone.clone();
    let () = rotated_left.rotate_left();
    assert_stones_eq(next.stone.as_ref().unwrap(), &rotated_left);

    let next = expanded.next().unwrap();
    let mut rotated_right = stone.clone();
    let () = rotated_right.rotate_right();
    assert_stones_eq(next.stone.as_ref().unwrap(), &rotated_right);

    assert_eq!(expanded.next(), None);
  }
}
