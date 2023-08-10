// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(
  clippy::let_and_return,
  clippy::let_unit_value,
  clippy::module_inception
)]

mod opengl;

use anyhow::Context as _;
use anyhow::Result;

use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;

use crate::opengl::Window;


fn main() -> Result<()> {
  let event_loop = EventLoop::new();
  let mut window = Window::new(&event_loop).context("failed to create OpenGL window")?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;
    let result = (|| {
      let () = match event {
        Event::LoopDestroyed => (),
        Event::WindowEvent { event, .. } => match event {
          WindowEvent::ReceivedCharacter(c) if c == 'q' => control_flow.set_exit(),
          WindowEvent::CloseRequested => control_flow.set_exit(),
          _ => (),
        },
        Event::RedrawRequested(_) => {
          let renderer = window.renderer();
          let () = renderer.on_pre_render()?;
          let () = renderer.on_post_render()?;
        },
        _ => (),
      };

      <Result<()>>::Ok(())
    })();

    if let Err(err) = result {
      eprintln!("{err:#}");
      control_flow.set_exit_with_code(1);
    }
  });
}
