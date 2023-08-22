// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(
  clippy::let_and_return,
  clippy::let_unit_value,
  clippy::module_inception
)]

mod game;
mod guard;
mod opengl;
mod point;
mod rect;

use std::num::NonZeroU32;

use anyhow::Context as _;
use anyhow::Result;

use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use crate::game::Game;
use crate::opengl::ActiveRenderer;
use crate::opengl::Color;
use crate::opengl::Renderer;
use crate::opengl::Texture;
use crate::opengl::Window;
use crate::point::Point;
use crate::rect::Rect;


fn main() -> Result<()> {
  let event_loop = EventLoop::new();
  let mut window = Window::new(&event_loop).context("failed to create OpenGL window")?;

  let (phys_w, phys_h) = window.size();
  let game = Game::new().context("failed to instantiate game object")?;
  let mut renderer = Renderer::new(phys_w, phys_h, game.width(), game.height());

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    let () = match event {
      Event::LoopDestroyed => (),
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::ReceivedCharacter(c) if c == 'q' => control_flow.set_exit(),
        WindowEvent::CloseRequested => control_flow.set_exit(),
        WindowEvent::Resized(phys_size) => {
          let phys_w = NonZeroU32::new(phys_size.width)
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
          let phys_h = NonZeroU32::new(phys_size.height)
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });

          let () = window.on_resize(phys_w, phys_h);
          let () = renderer.update_view(phys_w, phys_h, game.width(), game.height());
        },
        _ => (),
      },
      Event::RedrawRequested(_) => {
        {
          let renderer = renderer.on_pre_render(&mut window);
          let () = game.render(&renderer);
        }
        let () = window.swap_buffers();
      },
      _ => (),
    };
  });
}
