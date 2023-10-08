// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::min;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::os::raw::c_uint;
use std::ptr::null;
use std::time::Duration;
use std::time::Instant;

use anyhow::ensure;
use anyhow::Context as _;
use anyhow::Result;

use serde::Deserialize;
use serde::Serialize;

use winit::event::ElementState;
use winit::keyboard::KeyCode as Key;

use x11_dl::xlib;

use crate::Change;
use crate::Tick;


#[derive(Debug, Deserialize, Serialize)]
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
  Pressed { pressed_at: Instant },
  Repeated { next_repeat: Instant },
}

impl KeyState {
  fn next_tick(&self) -> Instant {
    match self {
      Self::Pressed { pressed_at } => *pressed_at,
      Self::Repeated { next_repeat } => *next_repeat,
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
pub(crate) struct Keys {
  /// The "timeout" after the initial key press after which the first
  /// repeat is issued.
  timeout: Duration,
  /// The interval for any subsequent repeats.
  interval: Duration,
  /// A map from keys that are currently pressed to internally used
  /// key repetition state.
  pressed: HashMap<Key, Option<KeyState>>,
}

impl Keys {
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

  /// This method is to be invoked on every key event.
  pub(crate) fn on_key_event(&mut self, now: Instant, key: Key, state: ElementState) {
    match state {
      ElementState::Released => {
        let _prev = self.pressed.remove(&key);
      },
      ElementState::Pressed => {
        let entry = self.pressed.entry(key);
        match entry {
          Entry::Vacant(vacancy) => {
            let _state = vacancy.insert(Some(KeyState::Pressed { pressed_at: now }));
          },
          // If the key is already pressed we just got an AutoRepeat
          // event. We manage repetitions ourselves, so we skip any
          // handling.
          Entry::Occupied(_occupancy) => (),
        }
      },
    }
  }

  // TODO: It could be beneficial to coalesce nearby ticks into a single
  //       one, to reduce the number of event loop wake ups.
  pub(crate) fn tick<F>(&mut self, now: Instant, mut handler: F) -> (Change, Tick)
  where
    F: FnMut(&Key, &mut KeyRepeat) -> Change,
  {
    let mut change = Change::Unchanged;
    let mut next_tick = Tick::None;
    let mut remove = None;

    'next_key: for (key, key_state_opt) in self.pressed.iter_mut() {
      if let Some(key_state) = key_state_opt {
        while now >= key_state.next_tick() {
          let mut repeat = KeyRepeat::Enabled;
          change |= handler(key, &mut repeat);

          match repeat {
            KeyRepeat::Disabled => {
              *key_state_opt = None;
              remove = Some(*key);
              continue 'next_key
            },
            KeyRepeat::Enabled => match key_state {
              KeyState::Pressed { pressed_at } => {
                let first_repeat = *pressed_at + self.timeout;
                *key_state = KeyState::Repeated {
                  next_repeat: first_repeat,
                };
              },
              KeyState::Repeated { next_repeat } => {
                let next_repeat = *next_repeat + self.interval;
                *key_state = KeyState::Repeated { next_repeat };
              },
            },
          }
        }

        next_tick = min(next_tick, Tick::At(key_state.next_tick()));
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
  use std::cell::Cell;

  use super::*;


  /// A `Duration` of one second.
  const SECOND: Duration = Duration::from_secs(1);


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

  /// Check that keys are being reported as pressed as expected.
  #[test]
  fn key_pressing() {
    let enter_pressed = Cell::new(0);
    let space_pressed = Cell::new(0);
    let f3_pressed = Cell::new(0);

    let mut handler = |key: &Key, repeat: &mut KeyRepeat| match key {
      Key::Enter => {
        enter_pressed.set(enter_pressed.get() + 1);
        Change::Changed
      },
      Key::Space => {
        space_pressed.set(space_pressed.get() + 1);
        Change::Changed
      },
      Key::F3 => {
        f3_pressed.set(f3_pressed.get() + 1);
        *repeat = KeyRepeat::Disabled;
        Change::Changed
      },
      _ => Change::Unchanged,
    };

    let timeout = Duration::from_secs(5);
    let interval = Duration::from_secs(1);
    let mut keys = Keys::new(timeout, interval);

    let now = Instant::now();
    let (change, tick) = keys.tick(now, &mut handler);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, Tick::None);

    let () = keys.on_key_event(now, Key::Enter, ElementState::Pressed);
    let (change, tick) = keys.tick(now, &mut handler);
    assert_eq!(enter_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Tick::At(now + 5 * SECOND));

    // Another tick at the same timestamp shouldn't change anything.
    let (change, tick) = keys.tick(now, &mut handler);
    assert_eq!(enter_pressed.get(), 1);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, Tick::At(now + 5 * SECOND));

    // Additional press events for the same key should just be ignored.
    let () = keys.on_key_event(now, Key::Enter, ElementState::Pressed);

    // Or even half a second into the future.
    let (change, tick) = keys.tick(now + Duration::from_millis(500), &mut handler);
    assert_eq!(enter_pressed.get(), 1);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, Tick::At(now + 5 * SECOND));

    // At t+5s we hit the auto-repeat timeout.
    let (change, tick) = keys.tick(now + 5 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 2);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Tick::At(now + 6 * SECOND));

    // Press F3 as well. That should be a one-time thing only, as the
    // handler disabled auto-repeat.
    let () = keys.on_key_event(now + 5 * SECOND, Key::F3, ElementState::Pressed);
    assert_eq!(f3_pressed.get(), 0);

    // We skipped a couple of ticks and at t+8s we should see three
    // additional repeats.
    let (change, tick) = keys.tick(now + 8 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 5);
    assert_eq!(f3_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Tick::At(now + 9 * SECOND));

    assert_eq!(space_pressed.get(), 0);
    // At t+9s we also press Space.
    let () = keys.on_key_event(now + 9 * SECOND, Key::Space, ElementState::Pressed);

    let (change, tick) = keys.tick(now + 10 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 7);
    assert_eq!(space_pressed.get(), 1);
    assert_eq!(f3_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Tick::At(now + 11 * SECOND));

    // At t+15s we should see another 5 repeats for Enter as well as two
    // for Space.
    let (change, tick) = keys.tick(now + 15 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 12);
    assert_eq!(space_pressed.get(), 3);
    assert_eq!(f3_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Tick::At(now + 16 * SECOND));

    // Space is released just "before" it's next tick, so we shouldn't
    // see a press fire.
    let () = keys.on_key_event(now + 16 * SECOND, Key::Space, ElementState::Released);

    let (change, tick) = keys.tick(now + 16 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 13);
    assert_eq!(space_pressed.get(), 3);
    assert_eq!(f3_pressed.get(), 1);
    assert_eq!(change, Change::Changed);
    assert_eq!(tick, Tick::At(now + 17 * SECOND));

    let () = keys.on_key_event(now + 17 * SECOND, Key::Enter, ElementState::Released);

    let (change, tick) = keys.tick(now + 17 * SECOND, &mut handler);
    assert_eq!(enter_pressed.get(), 13);
    assert_eq!(space_pressed.get(), 3);
    assert_eq!(f3_pressed.get(), 1);
    assert_eq!(change, Change::Unchanged);
    assert_eq!(tick, Tick::None);
  }
}
