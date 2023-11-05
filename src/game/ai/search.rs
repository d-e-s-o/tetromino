// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::BinaryHeap;
use std::iter::from_fn;
use std::ops::Deref as _;
use std::rc::Rc;

use crate::Point;

use super::super::Fieldlike as _;
use super::super::Stonelike as _;
use super::Action;
use super::Cost;
use super::Field;
use super::State;
use super::Stone;


pub(super) fn actions(
  state: Option<Rc<State>>,
) -> (impl Iterator<Item = Action>, Option<Rc<Field>>) {
  let mut field = state.as_ref().map(|state| state.field.clone());
  let mut prev_field = None;
  let mut prev = Option::<Rc<State>>::None;
  let mut next = state.clone();
  let mut first = state;
  let mut merge = None;

  // The states provided form a linked list from "best" one to the
  // initial state. But that's not the order we need: we need a list
  // starting with the initial state. So here we effectively invert
  // everything.
  while let Some(state) = next {
    let parent = state.parent.clone();
    let state = state.reparent(prev);

    // On top of inverting, we also make a cut off whenever there is no
    // action. A state without an action is one after the stone changed.
    // What we are ultimately interested in reporting is the action
    // sequence for just the first stone, but not any of the preview
    // ones. Those were just used to improve the precision of the
    // estimation.
    match state.action {
      Some(Action::Merge) => {
        prev_field = field;
        field = parent.as_ref().map(|parent| parent.field.clone());
        merge = state.action;
        prev = None
      },
      None => {
        debug_assert_eq!(parent, None);
        break
      },
      Some(_) => {
        prev = Some(state.clone());
      },
    }

    first = Some(state);
    next = parent;
  }

  let mut next = first;

  let it = from_fn(move || {
    while let Some(state) = &next {
      if let action @ Some(..) = state.action {
        next = state.parent.clone();
        return action
      } else {
        next = state.parent.clone();
      }
    }

    merge.take()
  });

  (it, prev_field)
}

fn estimate_cost(field: &Field) -> Cost {
  let width = f32::from(field.width());
  let height = f32::from(field.height());
  let center_x = width / 2.0;

  let piece_capacity = width * height;

  let mut holes = 0i16;
  let mut top_most_piece_y = 0f32;
  let mut prev_top_piece_y = None;
  let mut roughness = 0f32;

  let mut coverage = 0i16;
  let mut stack_size = 0f32;

  for x in 0..field.width() {
    let mut top_piece_y = None;
    let mut pieces = 0i16;

    for y in (0..field.height()).rev() {
      let location = Point::new(x, y);
      let piece = field[location];

      if piece.is_some() {
        pieces += 1;
      }

      if top_piece_y.is_none() && piece.is_some() {
        top_piece_y = Some(f32::from(y + 1));
      }

      if piece.is_none() && top_piece_y.is_some() {
        holes += 1;
        coverage += pieces;
      }
    }

    // Favor tangentially lower "stacks" in the center and higher
    // towards the sides. That's a consequence of it requiring more
    // moves to get a stone to the side, which ultimately makes it less
    // suitable once we get closer to the top.
    let x_scale = 0.5 - (f32::from(x + 1) - center_x).abs() / (4.0 * center_x);
    debug_assert!((0.25..=0.5).contains(&x_scale));

    stack_size += top_piece_y.unwrap_or(0.0) * x_scale;

    roughness +=
      (prev_top_piece_y.or(top_piece_y).unwrap_or(0.0) - top_piece_y.unwrap_or(0.0)).abs();
    prev_top_piece_y = top_piece_y.or(Some(0.0));
    top_most_piece_y = top_most_piece_y.max(top_piece_y.unwrap_or(0.0));
  }

  let holes_cost = f32::from(holes) / piece_capacity;
  let cover_cost = f32::from(coverage) / piece_capacity * (top_most_piece_y / height);
  let stack_cost = stack_size / piece_capacity;
  let rough_cost = roughness / piece_capacity;
  let high_point_cost = top_most_piece_y / height;

  let mut cost = Cost::none();
  cost += 4.0 * holes_cost;
  cost += 0.1 * cover_cost;
  cost += stack_cost;
  cost += 1.5 * rough_cost;
  // We value the top most piece very highly. That's effectively our way
  // of favoring the clearing of lines, but also making clearing of
  // lines more important as the field gets filled more overall.
  cost += 8.0 * high_point_cost.powi(16);

  cost
}

fn update_field(mut field: Field, stone: Stone) -> Field {
  let bounds = stone.bounds();
  let y_range = bounds.y..bounds.y + bounds.h;
  let _cleared = field.merge_stone(stone);
  let _removed = field.remove_complete_lines(y_range);
  debug_assert_eq!(_cleared, _removed);
  field
}

/// Search for an action sequence that is expected to yield the best
/// possible outcome.
///
/// The search is inspired by the A* algorithm. However, we don't search
/// for a "goal", as there is no such thing here. We search for what we
/// suspect to be the best possible next state given the provided set of
/// stones. This is an estimate based on a set of partly subjective
/// heuristics. In the end, we still do an exhaustive fill state search.
// TODO: It would probably be better to employ the following algorithm:
//       1) we maintain some notion of a frontier on which stones will
//          be dropped (i.e., where dropped pieces start)
//       2) we only scan all actions and the resulting states along this
//          frontier, as opposed to everything else; potentially
//          starting at the center point and then doing
//          divide-and-conquer style work distribution (which would be
//          nicely parallelizable)
//       3) once we found the "best" state (as per our heuristic), we
//          perform text book A* search to get there
//       There may be some way here to improve incrementality for
//       subsequent stones that could speed things up further (though
//       details are unclear).
pub(super) fn search(field: &Field, stone: &Stone, next_stones: &[Stone]) -> Option<Rc<State>> {
  debug_assert!(!field.collides(stone));

  let stone_count =
    u8::try_from(next_stones.len()).expect("more than 255 preview stones are unsupported");

  let mut best = Option::<Rc<State>>::None;
  let mut open = BinaryHeap::new();

  let cost = estimate_cost(field);
  let state = State::initial(field.clone(), cost, stone.clone(), stone_count);
  let () = open.push(state);

  while let Some(state) = open.pop() {
    debug_assert!(
      state.stone.is_some(),
      "encountered unexpected final state: {state:?}"
    );

    // Note that although we do not insert visited states into the list
    // of open states directly, ones in there might become visited
    // between insertion and removal if expansion of another state
    // yielded the same stone -- skip those ones here.
    if state.was_visited() {
      continue
    }

    let () = state.visit();

    if state.has_collision() {
      if state.action == Some(Action::MoveDown) {
        // SANITY: There has to exist a parent because we just made
        //         sure that the stone had a `MoveDown` action, which
        //         means the state must have been derived at least
        //         once.
        let parent = state.parent.as_ref().unwrap();
        // SANITY: The parent state cannot have been final or we would
        //         not have a child. If it's not final, it must have a
        //         stone.
        let stone = parent.stone.as_ref().unwrap();
        let field = update_field(parent.field.deref().clone(), stone.clone());
        let cost = estimate_cost(&field);

        if let Some(next_stone) = next_stones.get(usize::from(parent.index)) {
          let mut next_stone = next_stone.clone();
          let result = field.reset_stone(&mut next_stone);
          // If we ended up hitting a collision on reset, this is not a
          // viable state after all, but otherwise we can continue the
          // search with it.
          if result {
            let () = open.push(parent.merged(field, cost, next_stone));
          }
        } else {
          let best_cost = best
            .as_ref()
            // For reporting the final best state, we only look at the
            // actual field cost. All other costs are really only used
            // to guide the search.
            .map(|state| state.field_cost)
            .unwrap_or_else(Cost::max);

          if cost < best_cost {
            // Strictly speaking `parent` already has a cost associated
            // with it, but really that one is about the *initial* state
            // of the associated field. When we reach this point we have
            // derived it so far that the associated stone has been merged
            // and so really what we are working with here *is* the
            // relevant cost.
            best = Some(State::finalize(parent, field, cost));
          }
        }
      }
    } else {
      for expanded in state.expand() {
        if expanded.was_visited() {
          continue
        }

        // Note that one important fact here is that the expand()
        // function itself does not check whether an expanded state is
        // valid, i.e., whether the corresponding stone is actually
        // within the bounds of the field; this means we might insert
        // an "invalid" stone here, but this is by design because this
        // is essentially how we check for a reached goal
        let () = open.push(expanded);
      }
    }
  }

  best
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::collections::VecDeque;
  use std::iter::from_fn;

  use crate::Rng;

  use super::super::super::Matrix;
  use super::super::util::field;
  use super::super::util::stone;


  /// Check that we correctly create an empty action list when provided
  /// with no [`State`].
  #[test]
  fn action_list_creation() {
    let (mut actions, _field) = actions(None);
    assert_eq!(actions.next(), None);
  }

  /// Check that the equal fields yield same utilities.
  #[test]
  fn cost_same_field() {
    let field = field! {"
      ......
      ......
      ......
      ......
      ######
    "};

    let cost1 = estimate_cost(&field);
    let cost2 = estimate_cost(&field);
    assert_eq!(cost1, cost2);
  }

  /// Make sure that utility increases few lines are occupied in the
  /// field.
  #[test]
  fn cost_fewer_lines() {
    let field1 = field! {"
      ......
      ......
      ......
      #...##
      ######
    "};
    let field2 = field! {"
      ......
      ......
      ......
      ......
      #...##
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 > cost2, "{cost1} vs. {cost2}");
  }

  /// Make sure that utility increases when more lines are cleared.
  #[test]
  fn cost_rougher_field() {
    let field1 = field! {"
      ......
      ......
      ......
      ......
      ######
    "};
    let field2 = field! {"
      ......
      ......
      ......
      ..#...
      ######
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 < cost2, "{cost1} vs. {cost2}");
  }

  /// Check that all things equal, we avoid central placement.
  #[test]
  fn cost_avoid_central_placement() {
    let field1 = field! {"
      ......
      ......
      ......
      ......
      #.....
    "};

    let field2 = field! {"
      ......
      ......
      ......
      ......
      ...#..
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 < cost2, "{cost1} vs. {cost2}");

    let field1 = field! {"
      .........
      .........
      .........
      .........
      ....####.
    "};
    let field2 = field! {"
      .........
      .........
      .........
      .........
      .....####
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 > cost2, "{cost1} vs. {cost2}");
  }

  /// Make sure that is utility is higher as pieces are shifted towards
  /// the outside of the field.
  #[test]
  fn cost_rougher_field_sides() {
    let field1 = field! {"
      ......
      ......
      ......
      .#....
      ######
    "};
    let field2 = field! {"
      ......
      ......
      ......
      #.....
      ######
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 > cost2, "{cost1} vs. {cost2}");
  }

  /// Make sure that leaving a hole results in less utility than not
  /// leaving a hole.
  #[test]
  fn cost_less_with_hole() {
    let field1 = field! {"
      ......
      ......
      ......
      ..#...
      ###.##
    "};
    let field2 = field! {"
      ......
      ......
      ......
      ..#...
      ##.###
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 < cost2, "{cost1} vs. {cost2}");
  }

  /// Make sure that we prefer not leaving a hole over placing a stone
  /// closer to the sides.
  #[test]
  fn cost_hole_versus_side() {
    let field1 = field! {"
      ..........
      ..........
      ..........
      ....##....
      .#######..
      ######.###
      ##.#.#..#.
      ####.#####
    "};
    let field2 = field! {"
      ..........
      ..........
      ..........
      ....##....
      ####.###..
      ######.###
      ##.#.#..#.
      ####.#####
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 < cost2, "{cost1} vs. {cost2}");
  }

  /// Check that exposing a bunch of roughness still results in an
  /// overall cost decrease.
  #[test]
  fn cost_roughness_versus_holes() {
    let field1 = field! {"
      ....................
      ....................
      ....................
      ########.###########
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
    "};
    let field2 = field! {"
      ....................
      ....................
      ....................
      .......###..........
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
      #.#.#.#.#.#.#.#.#.#.
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 > cost2, "{cost1} vs. {cost2}");
  }

  /// Test that we favor clearing a line and lowering the "high-point"
  /// even if we end up covering additional "holes".
  #[test]
  fn cost_hole_versus_cleared() {
    let field1 = field! {"
      .....
      .....
      #...#
      #...#
      #..##
      #..##
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
    "};
    let field2 = field! {"
      .....
      .....
      .....
      #...#
      #.###
      #..##
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
      #...#
    "};
    let cost1 = estimate_cost(&field1);
    let cost2 = estimate_cost(&field2);
    assert!(cost1 > cost2, "{cost1} vs. {cost2}");
  }

  fn replay<A>(mut field: Field, mut stone: Stone, actions: A) -> Field
  where
    A: IntoIterator<Item = Action>,
  {
    assert!(!field.collides(&stone));

    let mut actions = actions.into_iter();

    while let Some(action) = actions.next() {
      match action {
        Action::MoveDown => stone.move_down(),
        Action::MoveLeft => stone.move_left(),
        Action::MoveRight => stone.move_right(),
        Action::RotateLeft => stone.rotate_left(),
        Action::RotateRight => stone.rotate_right(),
        Action::Merge => {
          let bounds = stone.bounds();
          let y_range = bounds.y..bounds.y + bounds.h;
          let cleared = field.merge_stone(stone);
          let removed = field.remove_complete_lines(y_range);
          assert_eq!(cleared, removed);
          assert_eq!(actions.next(), None);
          break
        },
      }
    }

    field
  }

  fn evaluate_single(field: Field, stone: &Stone) -> Field {
    evaluate_multi(field, stone, &[])
  }


  fn evaluate_multi(field: Field, stone: &Stone, next_stones: &[Stone]) -> Field {
    let mut stone = stone.clone();
    let result = field.reset_stone(&mut stone);
    assert!(result);

    let best = search(&field, &stone, next_stones);
    let (actions, expected_field) = actions(best);
    let field = replay(field, stone, actions);
    assert_eq!(Some(&field), expected_field.as_deref());
    field
  }


  /// Check that we find an action sequence clearing lines if that's
  /// clearly the best thing we can do.
  #[test]
  fn search_clearing_lines() {
    let stone = stone! {"
      ##
      ##
    "};
    let field = field! {"
      ......
      ......
      ......
      ####..
      ####..
    "};

    let field = evaluate_single(field, &stone);
    let expected = field! {"
      ......
      ......
      ......
      ......
      ......
    "};
    assert_eq!(field, expected);
  }

  /// Make sure that we plug an easily fillable hole, resulting in lines
  /// being cleared.
  #[test]
  fn search_fill_hole_1() {
    let stone = stone! {"
      #.
      #.
      ##
    "};
    let field = field! {"
      ..........
      ..........
      ..........
      ..........
      ..........
      ..###...##
      #####...##
      #####.####
      #######.##
      #.#####.##
    "};

    let field = evaluate_single(field, &stone);
    let expected = field! {"
      ..........
      ..........
      ..........
      ..........
      ..........
      ..........
      ..........
      ..###...##
      #######.##
      #.#####.##
    "};
    assert_eq!(field, expected);
  }

  /// Make sure that we plug an easily fillable hole.
  #[test]
  fn search_fill_hole_2() {
    let stone = stone! {"
      .#.
      ###
    "};
    let field = field! {"
      ..........
      ..........
      ..........
      .......##.
      ...##.####
      ....#.####
      ######.###
    "};

    let field = evaluate_single(field, &stone);
    let expected = field! {"
      ..........
      ..........
      ..........
      .......##.
      ..###.####
      .####.####
      ######.###
    "};
    assert_eq!(field, expected);
  }

  /// Check that we can successfully search for an action sequence in a
  /// complex situation.
  ///
  /// This test is partly a regression test for an earlier issue we had
  /// where an action sequence was reported that resulted in an
  /// unexpected collision.
  #[test]
  #[ignore = "stress test; may take excessive time"]
  fn search_complex_with_look_ahead() {
    let field = field! {"
      .............#......
      ............##......
      ........#...##......
      .......#######......
      .......#######......
      .......########...##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      .#....###########.##
      .##...###########.##
      .##...##############
      .##...###########.##
      .##...##############
      .##...##############
      .####..#############
      .####.##############
      .##.#.###########.##
      .####.##############
      .####.##############
      .####.##########.###
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
    "};

    let stone = stone! {"
      #.
      #.
      ##
    "};

    let next = [
      stone! {"
        .#
        .#
        ##
      "},
      stone! {"
        .#
        .#
        ##
      "},
      stone! {"
        #.
        #.
        ##
      "},
    ];

    // For this test we don't really care about outcome, just that we
    // didn't trigger any unexpected code paths throughout.
    let _field = evaluate_multi(field, &stone, &next);
  }

  fn stone_factory(seed: u64) -> impl Iterator<Item = Stone> {
    let templates = {
      let o = stone! {"
        ##
        ##
        "};
      let s = stone! {"
        .##
        ##.
        "};
      let z = stone! {"
        ##.
        .##
      "};
      let i = stone! {"
        #
        #
        #
        #
      "};
      let t = stone! {"
        .#.
        ###
      "};
      let j = stone! {"
        .#
        .#
        ##
      "};
      let l = stone! {"
        #.
        #.
        ##
      "};
      [o, s, z, i, t, j, l]
    };
    let rng = Rng::with_seed(seed);

    from_fn(move || {
      let index = rng.rand_u32() as usize % templates.len();
      templates.get(index).cloned()
    })
  }


  fn play(mut field: Field) {
    let mut count = 0;
    let mut factory = stone_factory(1337);
    let mut next_stones = factory.by_ref().take(1).collect::<VecDeque<_>>();

    loop {
      let mut stone = next_stones.pop_front().unwrap();
      let () = next_stones.push_back(factory.next().unwrap());

      let result = field.reset_stone(&mut stone);
      if !result {
        break
      }

      let best = search(&field, &stone, next_stones.make_contiguous());
      let (actions, expected_field) = actions(best);
      field = replay(field, stone, actions);
      println!("{field:?}");

      if let Some(expected_field) = expected_field {
        assert_eq!(field, *expected_field);
      }

      count += 1;
    }

    println!("merged {count} stones");
  }

  /// "Stress-test" playing a game from start to finish.
  #[test]
  #[ignore = "stress test; may take excessive time"]
  fn play_until_over() {
    let field = Field::from_matrix(&Matrix::<Option<()>>::new(20, 40));
    play(field)
  }


  /// "Stress-test" playing the game from a pretty bad state.
  ///
  /// This test is mostly interesting to see whether the AI is capable
  /// of recovering.
  #[test]
  #[ignore = "stress test; may take excessive time"]
  fn play_to_fix_mess1() {
    let field = field! {"
      ....................
      ....................
      ....................
      ....................
      ........####........
      .......########...##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      ......##########..##
      .#....###########.##
      .##...###########.##
      .##...##############
      .##...###########.##
      .##...##############
      .##...##############
      .####..#############
      .####.##############
      .##.#.###########.##
      .####.##############
      .####.##############
      .####.##########.###
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
      .####.##############
    "};

    play(field)
  }

  #[test]
  #[ignore = "stress test; may take excessive time"]
  fn play_to_fix_mess2() {
    let field = field! {"
      ..........
      ..........
      ..........
      ..........
      ..........
      ..........
      ..........
      ..........
      ..........
      #....#....
      ###.###...
      #######...
      ########..
      #####.###.
      ##.#####..
      ######.#..
      #########.
      ####.#..#.
      ####....#.
      #######.#.
      .##.#####.
      ##.....##.
      #.......#.
      #.......#.
      ##...#..##
      ##...#..##
      .##..####.
      .#..##.##.
      ###..####.
      ##....##..
      ##.####...
      ##..##....
      .##..#....
      .#...#....
      ###..#....
      .#...#....
      .#..##....
      ##..###...
      ##..#...##
      ##..##..##
    "};

    play(field)
  }

  #[test]
  #[ignore = "stress test; may take excessive time"]
  fn play_to_fix_mess3() {
    let field = field! {"
      ..........
      ........#.
      ........##
      ........##
      ........#.
      ........##
      ........##
      ........##
      ........##
      ....##..##
      ....##..##
      ....##..##
      ....##..##
      .#..##..##
      .##.##..##
      .##..#..##
      .##.##..#.
      .##.##..#.
      .##..#..##
      .##.##..##
      .##..#..##
      .##.##..##
      .#..##..##
      .##.##.###
      .##.##.###
      .##.##.###
      .##.##.###
      .##.##.###
      .##.##.###
      .##.##.###
      ###.##.###
      ###.##.###
      .##.##.###
      .#..##.###
      ###.##.###
      .#..##.###
      .##.##.###
      ###.######
      ###.#..###
      ###.##.###
    "};

    play(field)
  }
}
