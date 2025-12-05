// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

//! A graphical Tetris clone.

#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(all(feature = "nightly", test))]
extern crate test;

mod config;
mod game;
mod guard;
mod keys;
mod mode;
mod opengl;
mod point;
mod rand;
mod rect;

use std::cell::OnceCell;
use std::cmp::min;
use std::cmp::Ordering;
use std::fs::read_to_string;
use std::io::ErrorKind;
use std::num::NonZeroU32;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

use anyhow::Context as _;
use anyhow::Result;

use dirs::config_dir;

use winit::application::ApplicationHandler;
use winit::event::DeviceEvent;
use winit::event::DeviceId;
use winit::event::ElementState;
use winit::event::RawKeyEvent;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode as Key;
use winit::keyboard::PhysicalKey;
use winit::raw_window_handle::HasDisplayHandle as _;
use winit::window::WindowId;

use crate::keys::KeyRepeat;
use crate::keys::Keys;
use crate::mode::ColorMode;
use crate::mode::ColorSet;
use crate::opengl::ActiveRenderer;
use crate::opengl::Color;
use crate::opengl::Font;
use crate::opengl::Texture;
use crate::opengl::TextureBuilderExt;
use crate::point::Point;
use crate::rand::Rng;
use crate::rect::Rect;

#[doc(hidden)]
pub use crate::config::Config;
pub use crate::game::Config as GameConfig;
pub use crate::game::Game;
pub use crate::opengl::Context;
pub use crate::opengl::Renderer;
pub use crate::opengl::Window;


/// An enumeration of possible state changes performed/desired by lower
/// level parts of the program.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Change {
  /// Some state was changed that necessitates a redraw.
  Changed,
  /// A desire to quite the program has been made.
  Quit,
  /// No visible state has changed.
  #[default]
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

impl From<Option<Instant>> for Tick {
  fn from(other: Option<Instant>) -> Self {
    match other {
      Some(instant) => Tick::At(instant),
      None => Tick::None,
    }
  }
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


/// Our application's state.
struct State {
  window: Window,
  game: Game,
  renderer: Renderer,
  keys: Keys<Key>,
  was_paused: bool,
}


#[derive(Default)]
struct App {
  state: OnceCell<Result<State>>,
}

impl App {
  fn state<'slf>(&'slf mut self, event_loop: &ActiveEventLoop) -> Option<&'slf mut State> {
    match self.state.get_mut()? {
      Ok(state) => Some(state),
      Err(..) => {
        let () = event_loop.exit();
        None
      },
    }
  }
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    fn create_state(event_loop: &ActiveEventLoop) -> Result<State> {
      let config = load_config().context("failed to load program configuration")?;
      let display_handle = event_loop
        .display_handle()
        .context("failed to retrieve display handle")?;
      let create_window_fn = |attrs| event_loop.create_window(attrs);
      let window =
        Window::new(display_handle, create_window_fn).context("failed to create OpenGL window")?;
      let (phys_w, phys_h) = window.size();
      let game = Game::with_config(&config.game).context("failed to instantiate game object")?;
      let renderer = Renderer::new(phys_w, phys_h, game.width(), game.height())
        .context("failed to create OpenGL renderer")?;
      let timeout = Duration::from_millis(config.keyboard.auto_repeat_timeout_ms.into());
      let interval = Duration::from_millis(config.keyboard.auto_repeat_interval_ms.into());
      let keys = Keys::new(timeout, interval);
      let was_paused = game.is_paused();

      let state = State {
        window,
        game,
        renderer,
        keys,
        was_paused,
      };
      Ok(state)
    }

    let _state = self.state.get_or_init(|| create_state(event_loop));
    // "Check" the state and potentially trigger an event loop exit if
    // we failed part of the initialization.
    let _state = self.state(event_loop);
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    if let Some(State {
      ref mut window,
      ref mut game,
      ref mut renderer,
      ref mut keys,
      ref mut was_paused,
      ..
    }) = self.state(event_loop)
    {
      let change = match event {
        WindowEvent::Focused(focused) => {
          if focused {
            if !*was_paused {
              // The game was not paused when we lost focus. That means
              // we ended up pausing it. Unpause it again.
              let () = game.pause(false);
            }
          } else {
            *was_paused = game.is_paused();
            if !*was_paused {
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
          let () = event_loop.exit();
          return
        },
        WindowEvent::RedrawRequested => {
          let context = window.context_mut();
          let renderer = renderer.on_pre_render(context);
          let () = game.render(&renderer);
          let () = drop(renderer);
          let () = context.swap_buffers();
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
      };

      match change {
        Change::Changed => window.request_redraw(),
        Change::Quit => event_loop.exit(),
        Change::Unchanged => (),
      }
    }
  }

  fn device_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _device_id: DeviceId,
    event: DeviceEvent,
  ) {
    if let Some(State { ref mut keys, .. }) = self.state(event_loop) {
      if let DeviceEvent::Key(RawKeyEvent {
        physical_key: PhysicalKey::Code(key),
        state,
      }) = event
      {
        let now = Instant::now();
        match state {
          ElementState::Pressed => keys.on_key_press(now, key),
          ElementState::Released => keys.on_key_release(now, key),
        }
      }
    }
  }

  fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    if let Some(State {
      ref mut window,
      ref mut game,
      ref mut keys,
      ..
    }) = self.state(event_loop)
    {
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
      };

      let now = Instant::now();
      let (keys_change, keys_wait) = keys.tick(now, handle_key);
      let (game_change, game_wait) = game.tick(now);

      let control_flow = match min(game_wait, Tick::from(keys_wait)) {
        Tick::None => ControlFlow::Wait,
        Tick::At(wait_until) => ControlFlow::WaitUntil(wait_until),
      };
      let () = event_loop.set_control_flow(control_flow);

      let change = keys_change | game_change;

      match change {
        Change::Changed => window.request_redraw(),
        Change::Quit => event_loop.exit(),
        Change::Unchanged => (),
      }
    }
  }
}


// This function is really only meant to be used by the main program.
#[doc(hidden)]
pub fn run() -> Result<()> {
  let event_loop = EventLoop::new().context("failed to create event loop")?;
  let () = event_loop.set_control_flow(ControlFlow::Wait);
  let mut app = App::default();
  let () = event_loop.run_app(&mut app)?;
  if let Some(result) = app.state.into_inner() {
    result.map(|_state| ())
  } else {
    Ok(())
  }
}


#[cfg(test)]
#[cfg(feature = "nightly")]
mod tests {
  use super::*;

  use crate::game::Config;

  use test::Bencher;

  use winit::platform::x11::EventLoopBuilderExtX11 as _;


  /// Benchmark the performance of the rendering path.
  #[allow(deprecated)]
  #[bench]
  fn bench_render(b: &mut Bencher) {
    let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
    let display_handle = event_loop.display_handle().unwrap();
    let raw_display_handle = display_handle.into();
    let create_window_fn = |attrs| event_loop.create_window(attrs);
    let mut window = Window::new(raw_display_handle, create_window_fn).unwrap();

    let (phys_w, phys_h) = window.size();
    let config = Config::default();
    let game = Game::with_config(&config).unwrap();
    let renderer = Renderer::new(phys_w, phys_h, game.width(), game.height()).unwrap();

    let () = b.iter(|| {
      let context = window.context_mut();
      let renderer = renderer.on_pre_render(context);
      let () = game.render(&renderer);
      let () = drop(renderer);
      let () = context.swap_buffers();
    });
  }
}
