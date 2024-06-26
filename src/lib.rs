// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A graphical Tetris clone.

#![allow(
  clippy::collapsible_else_if,
  clippy::let_and_return,
  clippy::let_unit_value,
  clippy::module_inception
)]
#![warn(
  missing_debug_implementations,
  missing_docs,
  rustdoc::broken_intra_doc_links
)]
#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
extern crate test;

mod config;
mod game;
mod guard;
mod keys;
mod opengl;
mod point;
mod rand;
mod rect;

use std::cmp::min;
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::io::ErrorKind;
use std::num::NonZeroU32;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::path::PathBuf;
use std::time::Instant;

use anyhow::Context as _;
use anyhow::Result;

use dirs::config_dir;

use winit::event::DeviceEvent;
use winit::event::Event;
use winit::event::RawKeyEvent;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode as Key;
use winit::keyboard::PhysicalKey;

use crate::keys::KeyRepeat;
use crate::keys::Keys;
use crate::opengl::ActiveRenderer;
use crate::opengl::Color;
use crate::opengl::Font;
use crate::opengl::Texture;
use crate::point::Point;
use crate::rand::Rng;
use crate::rect::Rect;

#[doc(hidden)]
pub use crate::config::Config;
pub use crate::game::Config as GameConfig;
pub use crate::game::Game;
pub use crate::opengl::Renderer;
pub use crate::opengl::Window;


/// An enumeration of possible state changes performed/desired by lower
/// level parts of the program.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Change {
  /// Some state was changed that necessitates a redraw.
  Changed,
  /// A desire to quite the program has been made.
  Quit,
  /// No visible state has changed.
  Unchanged,
}

impl BitOr<Change> for Change {
  type Output = Change;

  fn bitor(self, rhs: Change) -> Self::Output {
    match (self, rhs) {
      (Self::Quit, _) | (_, Self::Quit) => Self::Quit,
      (Self::Changed, _) | (_, Self::Changed) => Self::Changed,
      (Self::Unchanged, Self::Unchanged) => Self::Unchanged,
    }
  }
}

impl BitOrAssign<Change> for Change {
  fn bitor_assign(&mut self, rhs: Change) {
    *self = *self | rhs;
  }
}


/// An enumeration describing when the next program "tick" should occur.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Tick {
  /// The next tick should happen at the given instant.
  At(Instant),
  /// No additional tick is necessary at this point.
  None,
}

impl PartialOrd<Tick> for Tick {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Tick {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::None, Self::None) => Ordering::Equal,
      (Self::At(_instant), Self::None) => Ordering::Less,
      (Self::None, Self::At(_instant)) => Ordering::Greater,
      (Self::At(instant1), Self::At(instant2)) => instant1.cmp(instant2),
    }
  }
}


/// Retrieve the default path to the program's configuration file.
fn default_config_path() -> Result<PathBuf> {
  let config = config_dir()
    .context("unable to determine config directory")?
    .join("tetromino")
    .join("config.toml");

  Ok(config)
}


fn load_config() -> Result<Config> {
  let path = default_config_path().context("failed to retrieve program config directory path")?;
  let contents = match read_to_string(&path) {
    Ok(contents) => contents,
    Err(err) if err.kind() == ErrorKind::NotFound => return Ok(Config::default()),
    e @ Err(..) => {
      e.with_context(|| format!("failed to load program configuration at {}", path.display()))?
    },
  };
  let config = toml::from_str(&contents)
    .with_context(|| format!("failed to parse TOML configuration at {}", path.display()))?;
  Ok(config)
}


// This function is really only meant to be used by the main program.
#[doc(hidden)]
pub fn run() -> Result<()> {
  let config = load_config().context("failed to load program configuration")?;
  let event_loop = EventLoop::new().context("failed to create event loop")?;
  let mut window = Window::new(&event_loop).context("failed to create OpenGL window")?;

  let (phys_w, phys_h) = window.size();
  let mut game = Game::with_config(&config.game).context("failed to instantiate game object")?;
  let mut renderer = Renderer::new(phys_w, phys_h, game.width(), game.height());
  let mut keys =
    Keys::with_config(config.keyboard).context("failed to instantiate auto key repeat manager")?;
  let mut was_paused = game.is_paused();

  let () = event_loop.run(move |event, target| {
    let now = Instant::now();
    let change = match event {
      Event::LoopExiting => return,
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Focused(focused) => {
          if focused {
            if let Some(false) = was_paused {
              // The game was not paused when we lost focus. That means
              // we ended up pausing it. Unpause it again.
              let () = game.pause(false);
            }
          } else {
            was_paused = game.is_paused();
            if let Some(false) = was_paused {
              // The game is currently running but we are about to loose
              // focus. Pause it, as the user will no longer have a
              // chance to control it and it's not great to have it
              // actively running in the background.
              let () = game.pause(true);
            }

            // We may not get informed about key releases once unfocused.
            // So just treat such an event as clearing all pressed keys
            // eagerly.
            let () = keys.clear();
          }
          Change::Unchanged
        },
        WindowEvent::CloseRequested => {
          let () = target.exit();
          return
        },
        WindowEvent::RedrawRequested => {
          let renderer = renderer.on_pre_render(&mut window);
          let () = game.render(&renderer);
          let () = drop(renderer);
          let () = window.swap_buffers();
          Change::Unchanged
        },
        WindowEvent::Resized(phys_size) => {
          let phys_w = NonZeroU32::new(phys_size.width)
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
          let phys_h = NonZeroU32::new(phys_size.height)
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });

          let () = window.on_resize(phys_w, phys_h);
          let () = renderer.update_view(phys_w, phys_h, game.width(), game.height());
          Change::Changed
        },
        _ => Change::Unchanged,
      },
      Event::DeviceEvent {
        event:
          DeviceEvent::Key(RawKeyEvent {
            physical_key: PhysicalKey::Code(key),
            state,
          }),
        ..
      } => {
        let () = keys.on_key_event(now, key, state);
        Change::Unchanged
      },
      Event::AboutToWait => {
        let handle_key = |key: &Key, repeat: &mut KeyRepeat| match key {
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
            if let Some(paused) = game.is_paused() {
              let () = game.pause(!paused);
            }
            *repeat = KeyRepeat::Disabled;
            Change::Unchanged
          },
          #[cfg(debug_assertions)]
          Key::F11 => {
            let () = game.dump_state();
            Change::Unchanged
          },
          _ => Change::Unchanged,
        };

        let (keys_change, keys_wait) = keys.tick(now, handle_key);
        let (game_change, game_wait) = game.tick(now);

        let control_flow = match min(game_wait, keys_wait) {
          Tick::None => ControlFlow::Wait,
          Tick::At(wait_until) => ControlFlow::WaitUntil(wait_until),
        };
        let () = target.set_control_flow(control_flow);

        keys_change | game_change
      },
      _ => Change::Unchanged,
    };

    match change {
      Change::Changed => window.request_redraw(),
      Change::Quit => target.exit(),
      Change::Unchanged => (),
    }
  })?;

  Ok(())
}


#[cfg(test)]
#[cfg(feature = "nightly")]
mod tests {
  use super::*;

  use crate::game::Config;

  use test::Bencher;

  use winit::event_loop::EventLoopBuilder;
  use winit::platform::x11::EventLoopBuilderExtX11 as _;


  /// Benchmark the performance of the rendering path.
  #[bench]
  fn bench_render(b: &mut Bencher) {
    let event_loop = EventLoopBuilder::new()
      .with_any_thread(true)
      .build()
      .unwrap();
    let mut window = Window::new(&event_loop).unwrap();

    let (phys_w, phys_h) = window.size();
    let config = Config::default();
    let game = Game::with_config(&config).unwrap();
    let renderer = Renderer::new(phys_w, phys_h, game.width(), game.height());

    let () = b.iter(|| {
      let renderer = renderer.on_pre_render(&mut window);
      let () = game.render(&renderer);
      let () = drop(renderer);
      let () = window.swap_buffers();
    });
  }
}
