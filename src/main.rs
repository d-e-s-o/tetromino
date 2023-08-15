// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(
  clippy::let_and_return,
  clippy::let_unit_value,
  clippy::module_inception
)]

mod guard;
mod opengl;
mod point;
mod rect;

use std::io::Cursor;
use std::num::NonZeroU16;
use std::num::NonZeroU32;

use anyhow::Context as _;
use anyhow::Result;

use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use crate::opengl::Color;
use crate::opengl::Texture;
use crate::opengl::Window;
use crate::point::Point;
use crate::rect::Rect;

const TETRIS_FIELD_TEXTURE: &[u8] = include_bytes!("../var/tetris_field_256x512.png");


fn main() -> Result<()> {
  // TODO: Should inquire the game field size.
  let logic_w = NonZeroU16::new(15).unwrap();
  let logic_h = NonZeroU16::new(23).unwrap();

  let event_loop = EventLoop::new();
  let mut window =
    Window::new(&event_loop, logic_w, logic_h).context("failed to create OpenGL window")?;

  let img =
    image::io::Reader::with_format(Cursor::new(TETRIS_FIELD_TEXTURE), image::ImageFormat::Png)
      .decode()?;
  let texture = Texture::with_image(img).unwrap();

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

          window.on_resize(phys_w, phys_h, logic_w, logic_h);
        },
        _ => (),
      },
      Event::RedrawRequested(_) => {
        let renderer = window.renderer();
        let renderer = renderer.on_pre_render();
        let _guard = renderer.set_color(Color::white());
        let _guard = renderer.set_texture(&texture);
        let () = renderer.render_rect(Rect::new(1, 1, 10, 10));
      },
      _ => (),
    };
  });
}
