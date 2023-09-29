// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(
  clippy::collapsible_else_if,
  clippy::let_and_return,
  clippy::let_unit_value,
  clippy::module_inception
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

use crate::game::Game;
use crate::keys::maybe_min_instant;
use crate::keys::KeyRepeat;
use crate::keys::Keys;
use crate::opengl::ActiveRenderer;
use crate::opengl::Color;
use crate::opengl::Font;
use crate::opengl::Renderer;
use crate::opengl::Texture;
use crate::opengl::Window;
use crate::point::Point;
use crate::rand::Rng;
use crate::rect::Rect;

pub use crate::config::Config;


#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
  Changed,
  Quit,
  Unchanged,
}

impl BitOr<State> for State {
  type Output = State;

  fn bitor(self, rhs: State) -> Self::Output {
    match (self, rhs) {
      (Self::Quit, _) | (_, Self::Quit) => Self::Quit,
      (Self::Changed, _) | (_, Self::Changed) => Self::Changed,
      (Self::Unchanged, Self::Unchanged) => Self::Unchanged,
    }
  }
}

impl BitOrAssign<State> for State {
  fn bitor_assign(&mut self, rhs: State) {
    *self = *self | rhs;
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


pub fn run() -> Result<()> {
  let config = load_config().context("failed to load program configuration")?;
  let event_loop = EventLoop::new();
  let mut window = Window::new(&event_loop).context("failed to create OpenGL window")?;

  let (phys_w, phys_h) = window.size();
  let mut game = Game::with_config(&config.game).context("failed to instantiate game object")?;
  let mut renderer = Renderer::new(phys_w, phys_h, game.width(), game.height());
  let mut keys =
    Keys::with_config(config.keyboard).context("failed to instantiate auto key repeat manager")?;
  let mut was_paused = game.is_paused();

  event_loop.run(move |event, _, control_flow| {
    let now = Instant::now();
    let state = match event {
      Event::LoopDestroyed => return,
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
          State::Unchanged
        },
        WindowEvent::CloseRequested => {
          let () = control_flow.set_exit();
          return
        },
        WindowEvent::Resized(phys_size) => {
          let phys_w = NonZeroU32::new(phys_size.width)
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
          let phys_h = NonZeroU32::new(phys_size.height)
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });

          let () = window.on_resize(phys_w, phys_h);
          let () = renderer.update_view(phys_w, phys_h, game.width(), game.height());
          State::Changed
        },
        _ => State::Unchanged,
      },
      Event::DeviceEvent {
        event:
          DeviceEvent::Key(RawKeyEvent {
            physical_key: key,
            state,
          }),
        ..
      } => {
        let () = keys.on_key_event(now, key, state);
        State::Unchanged
      },
      Event::MainEventsCleared => {
        let handle_key = |key: &Key, repeat: &mut KeyRepeat| match key {
          Key::Digit1 => game.on_rotate_left(),
          Key::Digit2 => game.on_rotate_right(),
          Key::KeyH => game.on_move_left(),
          Key::KeyJ => game.on_move_down(),
          Key::KeyL => game.on_move_right(),
          Key::KeyQ => State::Quit,
          Key::Enter => {
            *repeat = KeyRepeat::Disabled;
            game.restart()
          },
          Key::ArrowDown => game.on_move_down(),
          Key::ArrowLeft => game.on_move_left(),
          Key::ArrowRight => game.on_move_right(),
          Key::Space => {
            *repeat = KeyRepeat::Disabled;
            game.on_drop()
          },
          Key::F3 => {
            if let Some(paused) = game.is_paused() {
              let () = game.pause(!paused);
            }
            *repeat = KeyRepeat::Disabled;
            State::Unchanged
          },
          _ => State::Unchanged,
        };

        let (keys_state, keys_wait) = keys.tick(now, handle_key);
        let (game_state, game_wait) = game.tick(now);

        if let Some(wait_until) = maybe_min_instant(game_wait, keys_wait) {
          *control_flow = ControlFlow::WaitUntil(wait_until);
        } else {
          *control_flow = ControlFlow::Wait;
        }

        keys_state | game_state
      },
      Event::RedrawRequested(_) => {
        let renderer = renderer.on_pre_render(&mut window);
        let () = game.render(&renderer);
        let () = drop(renderer);
        let () = window.swap_buffers();
        State::Unchanged
      },
      _ => State::Unchanged,
    };

    match state {
      State::Changed => window.request_redraw(),
      State::Quit => control_flow.set_exit(),
      State::Unchanged => (),
    }
  });
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
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
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
