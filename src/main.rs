// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(
  clippy::let_and_return,
  clippy::let_unit_value,
  clippy::module_inception
)]
#![cfg_attr(feature = "nightly", feature(test))]

#[cfg(feature = "nightly")]
extern crate test;

mod game;
mod guard;
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

use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyEvent;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::Key;

use crate::game::Game;
use crate::opengl::ActiveRenderer;
use crate::opengl::Color;
use crate::opengl::Renderer;
use crate::opengl::Texture;
use crate::opengl::Window;
use crate::point::Point;
use crate::rand::Rng;
use crate::rect::Rect;


#[derive(Clone, Copy, Debug)]
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


fn main() -> Result<()> {
  let event_loop = EventLoop::new();
  let mut window = Window::new(&event_loop).context("failed to create OpenGL window")?;

  let (phys_w, phys_h) = window.size();
  let mut game = Game::new().context("failed to instantiate game object")?;
  let mut renderer = Renderer::new(phys_w, phys_h, game.width(), game.height());

  event_loop.run(move |event, _, control_flow| {
    let now = Instant::now();
    let event_state = match event {
      Event::LoopDestroyed => return,
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::KeyboardInput {
          event:
            KeyEvent {
              logical_key: key,
              state: ElementState::Pressed,
              ..
            },
          ..
        } => match key {
          Key::Character(c) => match c.as_str() {
            "1" => game.on_rotate_left(),
            "2" => game.on_rotate_right(),
            "h" => game.on_move_left(),
            "j" => game.on_move_down(),
            "l" => game.on_move_right(),
            "q" => {
              let () = control_flow.set_exit();
              return
            },
            _ => State::Unchanged,
          },
          Key::ArrowDown => game.on_move_down(),
          Key::ArrowLeft => game.on_move_left(),
          Key::ArrowRight => game.on_move_right(),
          Key::Space => game.on_drop(),
          _ => State::Unchanged,
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
      Event::RedrawRequested(_) => {
        let renderer = renderer.on_pre_render(&mut window);
        let () = game.render(&renderer);
        let () = drop(renderer);
        let () = window.swap_buffers();
        State::Unchanged
      },
      _ => State::Unchanged,
    };

    let (tick_state, wait_until) = game.tick(now);
    *control_flow = ControlFlow::WaitUntil(wait_until);

    if let State::Changed = event_state | tick_state {
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
