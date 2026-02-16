// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cmp::min;
use std::num::NonZeroU32;
use std::time::Instant;

use winit::keyboard::KeyCode as Key;

use crate::game::Game;
use crate::keys::KeyRepeat;
use crate::keys::Keys;
use crate::opengl::Renderer;
use crate::winit::Context;
use crate::Change;
use crate::Tick;


/// An abstraction over operations provided by the windowing system (or
/// similar), as required by the application.
pub(crate) trait Ops {
  fn context_mut(&mut self) -> &mut Context;
}

/// Our application's state.
pub(crate) struct App<O> {
  ops: O,
  game: Game,
  renderer: Renderer,
  keys: Keys<Key>,
  was_paused: bool,
}

impl<O> App<O>
where
  O: Ops,
{
  pub fn new(ops: O, game: Game, renderer: Renderer, keys: Keys<Key>) -> Self {
    let was_paused = game.is_paused();
    Self {
      ops,
      game,
      renderer,
      keys,
      was_paused,
    }
  }

  pub fn on_key_press(&mut self, key: Key, now: Instant) {
    let () = self.keys.on_key_press(now, key);
  }

  pub fn on_key_release(&mut self, key: Key, now: Instant) {
    let () = self.keys.on_key_release(now, key);
  }

  pub fn on_focus_event(&mut self, focused: bool) {
    if focused {
      if !self.was_paused {
        // The game was not paused when we lost focus. That means
        // we ended up pausing it. Unpause it again.
        let () = self.game.pause(false);
      }
    } else {
      self.was_paused = self.game.is_paused();
      if !self.was_paused {
        // The game is currently running but we are about to loose
        // focus. Pause it, as the user will no longer have a
        // chance to control it and it's not great to have it
        // actively running in the background.
        let () = self.game.pause(true);
      }

      // We may not get informed about key releases once unfocused.
      // So just treat such an event as clearing all pressed keys
      // eagerly.
      let () = self.keys.clear();
    }
  }

  pub fn on_window_resize(&mut self, phys_w: NonZeroU32, phys_h: NonZeroU32) {
    let () = self
      .renderer
      .update_view(phys_w, phys_h, self.game.width(), self.game.height());
  }

  fn handle_key(key: &Key, repeat: &mut KeyRepeat, game: &mut Game) -> Change {
    match key {
      Key::Digit1 => {
        *repeat = KeyRepeat::Disabled;
        game.on_rotate_left()
      },
      Key::Digit2 => {
        *repeat = KeyRepeat::Disabled;
        game.on_rotate_right()
      },
      Key::KeyH => game.on_move_left(),
      Key::KeyJ => game.on_move_down(),
      Key::KeyL => game.on_move_right(),
      Key::KeyQ => Change::Quit,
      Key::Backspace => {
        *repeat = KeyRepeat::Disabled;
        let () = game.restart();
        Change::Changed
      },
      Key::ArrowDown => game.on_move_down(),
      Key::ArrowLeft => game.on_move_left(),
      Key::ArrowRight => game.on_move_right(),
      Key::Space => {
        *repeat = KeyRepeat::Disabled;
        game.on_drop()
      },
      Key::F2 => {
        let () = game.auto_play(!game.is_auto_playing());
        *repeat = KeyRepeat::Disabled;
        Change::Unchanged
      },
      Key::F3 => {
        let () = game.pause(!game.is_paused());
        *repeat = KeyRepeat::Disabled;
        Change::Unchanged
      },
      Key::F4 => {
        let () = game.toggle_color_mode();
        *repeat = KeyRepeat::Disabled;
        Change::Changed
      },
      #[cfg(feature = "debug")]
      Key::F11 => {
        let () = game.dump_state();
        Change::Unchanged
      },
      _ => Change::Unchanged,
    }
  }

  pub fn tick(&mut self) -> (Change, Tick) {
    let now = Instant::now();
    let (keys_change, keys_wait) = self.keys.tick(now, |key, repeat| {
      Self::handle_key(key, repeat, &mut self.game)
    });
    let (game_change, game_wait) = self.game.tick(now);

    let change = keys_change | game_change;
    let tick = min(game_wait, Tick::from(keys_wait));
    (change, tick)
  }

  pub fn render(&mut self) {
    let context = self.ops.context_mut();
    let renderer = self.renderer.on_pre_render(context);
    let () = self.game.render(&renderer);
    let () = drop(renderer);
    let () = context.swap_buffers();
  }

  #[inline]
  pub fn ops(&self) -> &O {
    &self.ops
  }

  #[inline]
  pub fn ops_mut(&mut self) -> &mut O {
    &mut self.ops
  }
}
