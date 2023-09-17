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

mod game;
mod guard;
mod keys;
mod opengl;
mod point;
mod rand;
mod rect;

use std::num::NonZeroU32;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::time::Instant;

use anyhow::Context as _;
use anyhow::Result;

use winit::event::DeviceEvent;
use winit::event::Event;
use winit::event::RawKeyEvent;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode as Key;

use crate::game::Game;
use crate::keys::maybe_min_instant;
use crate::keys::Keys;
use crate::opengl::ActiveRenderer;
use crate::opengl::Color;
use crate::opengl::Renderer;
use crate::opengl::Texture;
use crate::opengl::Window;
use crate::point::Point;
use crate::rand::Rng;
use crate::rect::Rect;


#[derive(Clone, Copy, Debug, PartialEq)]
enum State {
  Changed,
  Unchanged,
}

impl BitOr<State> for State {
  type Output = State;

  fn bitor(self, rhs: State) -> Self::Output {
    match (self, rhs) {
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

pub fn run() -> Result<()> {
  let event_loop = EventLoop::new();
  let mut window = Window::new(&event_loop).context("failed to create OpenGL window")?;

  let (phys_w, phys_h) = window.size();
  let mut game = Game::new().context("failed to instantiate game object")?;
  let mut renderer = Renderer::new(phys_w, phys_h, game.width(), game.height());
  let mut keys =
    Keys::with_system_defaults().context("failed to instantiate auto key repeat manager")?;

  event_loop.run(move |event, _, control_flow| {
    let now = Instant::now();
    let event_state = match event {
      Event::LoopDestroyed => return,
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::Focused(false) => {
          // We may not get informed about key releases once unfocused.
          // So just treat such an event as clearing all pressed keys
          // eagerly.
          let () = keys.clear();
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
        let handle_key = |key: &Key| match key {
          Key::Digit1 => game.on_rotate_left(),
          Key::Digit2 => game.on_rotate_right(),
          Key::KeyH => game.on_move_left(),
          Key::KeyJ => game.on_move_down(),
          Key::KeyL => game.on_move_right(),
          Key::KeyQ => {
            let () = control_flow.set_exit();
            State::Unchanged
          },
          Key::Enter => game.restart(),
          Key::ArrowDown => game.on_move_down(),
          Key::ArrowLeft => game.on_move_left(),
          Key::ArrowRight => game.on_move_right(),
          Key::Space => game.on_drop(),
          Key::F3 => {
            let () = game.toggle_pause();
            State::Unchanged
          },
          _ => State::Unchanged,
        };

        let (keys_state, keys_wait) = keys.tick(now, handle_key);
        let (game_state, game_wait) = game.tick(now);

        // A key handler may have indicated a desire to exit. Don't
        // overwrite that.
        if !matches!(control_flow, ControlFlow::ExitWithCode(_)) {
          if let Some(wait_until) = maybe_min_instant(game_wait, keys_wait) {
            *control_flow = ControlFlow::WaitUntil(wait_until);
          } else {
            *control_flow = ControlFlow::Wait;
          }
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

    if let State::Changed = event_state {
      let () = window.request_redraw();
    }
  });
}


#[cfg(test)]
#[cfg(feature = "nightly")]
mod tests {
  use super::*;

  use test::Bencher;

  use winit::event_loop::EventLoopBuilder;
  use winit::platform::x11::EventLoopBuilderExtX11 as _;


  /// Benchmark the performance of the rendering path.
  #[bench]
  fn bench_render(b: &mut Bencher) {
    let event_loop = EventLoopBuilder::new().with_any_thread(true).build();
    let mut window = Window::new(&event_loop).unwrap();

    let (phys_w, phys_h) = window.size();
    let game = Game::new().unwrap();
    let renderer = Renderer::new(phys_w, phys_h, game.width(), game.height());

    let () = b.iter(|| {
      let renderer = renderer.on_pre_render(&mut window);
      let () = game.render(&renderer);
      let () = drop(renderer);
      let () = window.swap_buffers();
    });
  }
}