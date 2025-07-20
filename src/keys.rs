// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::ops::BitOrAssign;
use std::os::raw::c_uint;
use std::ptr::null;
use std::time::Duration;
use std::time::Instant;

use anyhow::ensure;
use anyhow::Context as _;
use anyhow::Result;

use serde::Deserialize;
use serde::Serialize;

use x11_dl::xlib;


/// Find the lesser of two `Option<Instant>` values.
///
/// Compared to using the default `Ord` impl of `Option`, `None` values
/// are actually strictly "greater" than any `Some`.
fn min_instant(a: Option<Instant>, b: Option<Instant>) -> Option<Instant> {
  match (a, b) {
    (None, None) => None,
    (Some(_instant), None) => a,
    (None, Some(_instant)) => b,
    (Some(instant1), Some(instant2)) => Some(instant1.min(instant2)),
  }
}


#[derive(Debug, Deserialize, Serialize)]
#[non_exhaustive]
pub struct Config {
  /// The auto-repeat timeout, in milliseconds.
  auto_repeat_timeout_ms: u32,
  /// The auto-repeat interval, in milliseconds.
  auto_repeat_interval_ms: u32,
}

impl Config {
  /// Instantiate a `Config` object using system defaults.
  pub(crate) fn with_system_defaults() -> Result<Self> {
    let mut timeout = MaybeUninit::<c_uint>::uninit();
    let mut interval = MaybeUninit::<c_uint>::uninit();
    // Value XkbUseCoreKbd constant defined somewhere in XKB.h and not
    // exported anywhere. Gorgeous!
    let xkb_use_core_kbd = 0x0100;

    let xlib = xlib::Xlib::open().context("failed to open xlib")?;
    // We could conceivably get the display passed in from our window or
    // some such, but the reality is that it's a royal mess to convert
    // one into the other.
    let display = unsafe { (xlib.XOpenDisplay)(null()) };
    ensure!(!display.is_null(), "failed to open X display");

    let result = unsafe {
      (xlib.XkbGetAutoRepeatRate)(
        display,
        xkb_use_core_kbd,
        timeout.as_mut_ptr(),
        interval.as_mut_ptr(),
      )
    };
    ensure!(result != 0, "failed to query keyboard auto repeat settings");

    let timeout = unsafe { timeout.assume_init() };
    let interval = unsafe { interval.assume_init() };

    let slf = Self {
      auto_repeat_timeout_ms: timeout,
      auto_repeat_interval_ms: interval,
    };
    Ok(slf)
  }
}

impl Default for Config {
  fn default() -> Self {
    match Self::with_system_defaults() {
      Ok(config) => config,
      Err(..) => {
        // We may not be able to query system settings in all cases. For
        // example, if no display is available we will likely fail.
        // However, for better user experience we still want to provide
        // somewhat sensible defaults. So just come up with something.
        Self {
          auto_repeat_timeout_ms: 100,
          auto_repeat_interval_ms: 50,
        }
      },
    }
  }
}


/// The state a single key can be in.
#[derive(Clone, Copy, Debug)]
enum KeyState {
  Pressed {
    pressed_at: Instant,
    fire_count: usize,
  },
  Repeated {
    pressed_at: Instant,
    next_repeat: Instant,
    fire_count: usize,
  },
  ReleasePending {
    pressed_at: Instant,
    fire_count: usize,
  },
}

impl KeyState {
  fn pressed(pressed_at: Instant) -> Self {
    Self::Pressed {
      pressed_at,
      fire_count: 0,
    }
  }

  fn on_press(&mut self, now: Instant) {
    match self {
      Self::Pressed { .. } | Self::Repeated { .. } => {
        // If the key is already pressed we just got an AutoRepeat
        // event. We manage repetitions ourselves, so we skip any
        // handling.
      },
      Self::ReleasePending { fire_count, .. } => {
        // The key had been released, but some events were still
        // undelivered. Mark it as pressed again, and carry over said
        // events.
        *self = Self::Pressed {
          pressed_at: now,
          fire_count: *fire_count,
        }
      },
    }
  }

  fn on_release(&mut self, now: Instant, timeout: Duration, interval: Duration) {
    match self {
      Self::Pressed {
        pressed_at,
        fire_count,
      } => {
        let next_repeat = *pressed_at + timeout;
        if now >= next_repeat {
          // We hit the auto-repeat "threshold".
          *self = Self::Repeated {
            pressed_at: *pressed_at,
            next_repeat,
            fire_count: *fire_count + 1,
          };
          let () = self.on_release(now, timeout, interval);
        } else {
          *self = Self::ReleasePending {
            pressed_at: *pressed_at,
            fire_count: *fire_count + 1,
          }
        }
      },
      Self::Repeated {
        pressed_at,
        next_repeat,
        fire_count,
      } => {
        let diff = now.saturating_duration_since(*next_repeat);
        // TODO: Use `Duration::div_duration_f64` once stable.
        *fire_count += (diff.as_secs_f64() / interval.as_secs_f64()).trunc() as usize;
        // If `now` is past the next auto repeat, take that into account
        // as well.
        if now > *next_repeat {
          *fire_count += 1;
        }

        *self = Self::ReleasePending {
          pressed_at: *pressed_at,
          fire_count: *fire_count,
        }
      },
      Self::ReleasePending { .. } => {
        debug_assert!(false, "released key was not pressed");
      },
    }
  }

  fn next_tick(&self) -> Option<Instant> {
    match self {
      Self::Pressed { pressed_at, .. } => Some(*pressed_at),
      Self::Repeated {
        pressed_at,
        next_repeat,
        fire_count,
      } => {
        if *fire_count > 0 {
          Some(*pressed_at)
        } else {
          Some(*next_repeat)
        }
      },
      Self::ReleasePending {
        pressed_at,
        fire_count,
      } => {
        if *fire_count > 0 {
          Some(*pressed_at)
        } else {
          None
        }
      },
    }
  }

  /// # Notes
  /// This method should only be called once the `Instant` returned by
  /// [`KeyState::next_tick`] has been reached.
  fn tick(&mut self, timeout: Duration, interval: Duration) {
    match self {
      Self::Pressed {
        pressed_at,
        fire_count,
      } => {
        if let Some(count) = fire_count.checked_sub(1) {
          *fire_count = count;
        } else {
          *self = KeyState::Repeated {
            pressed_at: *pressed_at,
            next_repeat: *pressed_at + timeout,
            fire_count: 0,
          };
        }
      },
      Self::Repeated {
        next_repeat,
        fire_count,
        ..
      } => {
        if let Some(count) = fire_count.checked_sub(1) {
          *fire_count = count;
        } else {
          *next_repeat += interval;
        }
      },
      Self::ReleasePending { fire_count, .. } => {
        *fire_count = fire_count.saturating_sub(1);
      },
    }
  }
}


/// An enum representing the two possible auto-key-repeat states
/// supported.
pub(crate) enum KeyRepeat {
  Enabled,
  Disabled,
}


/// A type helping with key press repetitions.
#[derive(Debug)]
pub(crate) struct Keys<K> {
  /// The "timeout" after the initial key press after which the first
  /// repeat is issued.
  timeout: Duration,
  /// The interval for any subsequent repeats.
  interval: Duration,
  /// A map from keys that are currently pressed to internally used
  /// key repetition state.
  ///
  /// The state may be `None` temporarily, in which case it is about to
  /// be removed.
  pressed: HashMap<K, Option<KeyState>>,
}

impl<K> Keys<K>
where
  K: Copy + Eq + Hash,
{
  /// Instantiate a `Keys` object using system default auto repeat
  /// timeout and interval.
  pub(crate) fn with_config(config: Config) -> Result<Self> {
    let timeout = Duration::from_millis(config.auto_repeat_timeout_ms.into());
    let interval = Duration::from_millis(config.auto_repeat_interval_ms.into());

    Ok(Self::new(timeout, interval))
  }

  fn new(timeout: Duration, interval: Duration) -> Self {
    Self {
      timeout,
      interval,
      pressed: HashMap::new(),
    }
  }

  fn on_key_event(&mut self, now: Instant, key: K, pressed: bool) {
    match pressed {
      false => match self.pressed.entry(key) {
        Entry::Vacant(_vacancy) => {
          // Note that a key could be released without being marked here
          // as pressed anymore, if auto repeat had been disabled. In
          // such a case it is fine to just ignore the release.
        },
        Entry::Occupied(mut occupancy) => {
          if let Some(ref mut state) = occupancy.get_mut() {
            let () = state.on_release(now, self.timeout, self.interval);
          } else {
            let _state = occupancy.remove();
          }
        },
      },
      true => match self.pressed.entry(key) {
        Entry::Vacant(vacancy) => {
          let _state = vacancy.insert(Some(KeyState::pressed(now)));
        },
        Entry::Occupied(mut occupancy) => {
          if let Some(ref mut state) = occupancy.get_mut() {
            let () = state.on_press(now);
          } else {
            let _state = occupancy.insert(Some(KeyState::pressed(now)));
          }
        },
      },
    }
  }

  /// This method is to be invoked on every key press.
  pub(crate) fn on_key_press(&mut self, now: Instant, key: K) {
    self.on_key_event(now, key, true)
  }

  /// This method is to be invoked on every key release.
  pub(crate) fn on_key_release(&mut self, now: Instant, key: K) {
    self.on_key_event(now, key, false)
  }

  // TODO: It could be beneficial to coalesce nearby ticks into a single
  //       one, to reduce the number of event loop wake ups.
  pub(crate) fn tick<F, C>(&mut self, now: Instant, mut handler: F) -> (C, Option<Instant>)
  where
    F: FnMut(&K, &mut KeyRepeat) -> C,
    C: Default + BitOrAssign,
  {
    let mut change = C::default();
    let mut next_tick = None;
    let mut remove = None;

    'next_key: for (key, key_state_opt) in self.pressed.iter_mut() {
      if let Some(key_state) = key_state_opt {
        loop {
          if let Some(tick) = key_state.next_tick() {
            if tick > now {
              next_tick = min_instant(next_tick, Some(tick));
              continue 'next_key
            }

            let mut repeat = KeyRepeat::Enabled;
            change |= handler(key, &mut repeat);

            match repeat {
              KeyRepeat::Disabled => {
                *key_state_opt = None;
                remove = remove.or(Some(*key));
                continue 'next_key
              },
              KeyRepeat::Enabled => {
                let () = key_state.tick(self.timeout, self.interval);
              },
            }
          } else {
            // If there is no next tick then the key had been released
            // earlier. Make sure to remove the state after we are done.
            *key_state_opt = None;
            remove = remove.or(Some(*key));
            continue 'next_key
          }
        }
      }
    }

    if let Some(key) = remove {
      // We only ever remove one key at a time to not have to allocate.
      // It won't take many invocations of this function to clear all
      // keys for which the "user" wants to disable auto-repeat, though.
      let _state = self.pressed.remove(&key);
      debug_assert!(_state.is_some());
    }

    (change, next_tick)
  }

  /// Clear all pressed keys.
  #[inline]
  pub(crate) fn clear(&mut self) {
    self.pressed.clear()
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use std::cell::Cell;

  use crate::Change;

  type Key = char;


  /// A `Duration` of one second.
  const SECOND: Duration = Duration::from_secs(1);
  const TIMEOUT: Duration = Duration::from_secs(5);
  const INTERVAL: Duration = Duration::from_secs(1);


  /// Make sure that we can create a `Config` object using system
  /// defaults.
  #[test]
  fn config_instantiation() {
    match Config::with_system_defaults() {
      Ok(_config) => (),
      // In something like a CI environment we may not have an X
      // display. Let's just ignore those rare setups for the time
      // being...
      Err(err) if err.to_string() == "failed to open X display" => (),
      Err(err) => panic!("{}", err),
    }
  }

  /// Check that we correctly handle press-release sequences without an
  /// intermediate tick.
  #[test]
  fn press_release_without_tick() {
    let l_pressed = Cell::new(0);

    let mut handler = |key: &Key, _repeat: &mut KeyRepeat| match key {
      'l' => {
        l_pressed.set(l_pressed.get() + 1);
        Change::Changed
      },
      _ => Change::Unchanged,
    };

    let now = Instant::now();
    let mut keys = Keys::<Key>::new(TIMEOUT, INTERVAL);

    let () = keys.on_key_press(now, 'l');
    let () = keys.on_key_release(now + 1 * SECOND, 'l');
    let (change, tick) = keys.tick(now + 1 * SECOND, &mut handler);
    assert_eq!(l_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, None);

    let (change, tick) = keys.tick(now + 2 * SECOND, &mut handler);
    assert_eq!(l_pressed.get(), 1);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, None);
  }


  /// Check that we handle a press after a release without a tick as
  /// expected.
  #[test]
  fn press_after_release_pending() {
    let h_pressed = Cell::new(0);

    let mut handler = |key: &Key, _repeat: &mut KeyRepeat| match key {
      'h' => {
        h_pressed.set(h_pressed.get() + 1);
        Change::Changed
      },
      _ => Change::Unchanged,
    };

    let now = Instant::now();
    let mut keys = Keys::<Key>::new(TIMEOUT, INTERVAL);

    let () = keys.on_key_press(now, 'h');
    let () = keys.on_key_release(now + 1 * SECOND, 'h');
    let () = keys.on_key_press(now + 2 * SECOND, 'h');

    let (change, tick) = keys.tick(now + 2 * SECOND, &mut handler);
    assert_eq!(h_pressed.get(), 2);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Some(now + 7 * SECOND));

    let (change, tick) = keys.tick(now + 3 * SECOND, &mut handler);
    assert_eq!(h_pressed.get(), 2);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, Some(now + 7 * SECOND));
  }


  /// Test that our `KeyState` logic works correctly when a key is
  /// released after auto-repeat already kicked in.
  #[test]
  fn release_pending_after_repeat() {
    let h_pressed = Cell::new(0);

    let mut handler = |key: &Key, _repeat: &mut KeyRepeat| match key {
      'h' => {
        h_pressed.set(h_pressed.get() + 1);
        Change::Changed
      },
      _ => Change::Unchanged,
    };

    let now = Instant::now();
    let mut keys = Keys::<Key>::new(TIMEOUT, INTERVAL);

    let () = keys.on_key_press(now, 'h');
    // Auto-repeat should kick in at `now + 5`. The one at `now + 7`
    // should not trigger, though, because of release.
    let () = keys.on_key_release(now + 7 * SECOND, 'h');

    let (change, tick) = keys.tick(now + 8 * SECOND, &mut handler);
    assert_eq!(h_pressed.get(), 4);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, None);
  }


  /// Check that keys are being reported as pressed as expected.
  #[test]
  fn key_pressing() {
    let enter_pressed = Cell::new(0);
    let space_pressed = Cell::new(0);
    let f_pressed = Cell::new(0);

    let mut handler = |key: &Key, repeat: &mut KeyRepeat| match key {
      '\n' => {
        enter_pressed.set(enter_pressed.get() + 1);
        Change::Changed
      },
      ' ' => {
        space_pressed.set(space_pressed.get() + 1);
        Change::Changed
      },
      'f' => {
        f_pressed.set(f_pressed.get() + 1);
        *repeat = KeyRepeat::Disabled;
        Change::Changed
      },
      _ => Change::Unchanged,
    };

    let mut keys = Keys::<Key>::new(TIMEOUT, INTERVAL);

    let now = Instant::now();
    let (change, tick) = keys.tick(now, &mut handler);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, None);

    let () = keys.on_key_press(now, '\n');
    let (change, tick) = keys.tick(now, &mut handler);
    assert_eq!(enter_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Some(now + 5 * SECOND));

    // Another tick at the same timestamp shouldn't change anything.
    let (change, tick) = keys.tick(now, &mut handler);
    assert_eq!(enter_pressed.get(), 1);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, Some(now + 5 * SECOND));

    // Additional press events for the same key should just be ignored.
    let () = keys.on_key_press(now, '\n');

    // Or even half a second into the future.
    let (change, tick) = keys.tick(now + Duration::from_millis(500), &mut handler);
    assert_eq!(enter_pressed.get(), 1);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, Some(now + 5 * SECOND));

    // At t+5s we hit the auto-repeat timeout.
    let (change, tick) = keys.tick(now + 5 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 2);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Some(now + 6 * SECOND));

    // Press F3 as well. That should be a one-time thing only, as the
    // handler disabled auto-repeat.
    let () = keys.on_key_press(now + 5 * SECOND, 'f');
    assert_eq!(f_pressed.get(), 0);

    // We skipped a couple of ticks and at t+8s we should see three
    // additional repeats.
    let (change, tick) = keys.tick(now + 8 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 5);
    assert_eq!(f_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Some(now + 9 * SECOND));

    assert_eq!(space_pressed.get(), 0);
    // At t+9s we also press Space.
    let () = keys.on_key_press(now + 9 * SECOND, ' ');

    let (change, tick) = keys.tick(now + 10 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 7);
    assert_eq!(space_pressed.get(), 1);
    assert_eq!(f_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Some(now + 11 * SECOND));

    // At t+15s we should see another 5 repeats for Enter as well as two
    // for Space.
    let (change, tick) = keys.tick(now + 15 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 12);
    assert_eq!(space_pressed.get(), 3);
    assert_eq!(f_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Some(now + 16 * SECOND));

    // Space is released just "before" it's next tick, so we shouldn't
    // see a press fire.
    let () = keys.on_key_release(now + 16 * SECOND, ' ');

    let (change, tick) = keys.tick(now + 16 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 13);
    assert_eq!(space_pressed.get(), 3);
    assert_eq!(f_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Some(now + 17 * SECOND));

    let () = keys.on_key_release(now + 17 * SECOND, '\n');

    let (change, tick) = keys.tick(now + 17 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 13);
    assert_eq!(space_pressed.get(), 3);
    assert_eq!(f_pressed.get(), 1);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, None);
  }
}
