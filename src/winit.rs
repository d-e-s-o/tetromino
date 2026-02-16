// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::OnceCell;
use std::fs::read_to_string;
use std::io::ErrorKind;
use std::num::NonZeroU32;
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
use winit::keyboard::PhysicalKey;
use winit::raw_window_handle::HasDisplayHandle as _;
use winit::window::WindowId;

use crate::app::App as AppT;
use crate::app::Ops;
use crate::game::Game;
use crate::keys::Keys;
use crate::opengl::Context;
use crate::opengl::Renderer;
use crate::Change;
use crate::Config;
use crate::Tick;
use crate::Window;


type App = AppT<Window>;


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


impl Ops for Window {
  fn context_mut(&mut self) -> &mut Context {
    Window::context_mut(self)
  }
}


#[derive(Default)]
struct Handler {
  app: OnceCell<Result<App>>,
}

impl Handler {
  fn app<'slf>(&'slf mut self, event_loop: &ActiveEventLoop) -> Option<&'slf mut App> {
    match self.app.get_mut()? {
      Ok(state) => Some(state),
      Err(..) => {
        let () = event_loop.exit();
        None
      },
    }
  }
}

impl ApplicationHandler for Handler {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    fn create_app(event_loop: &ActiveEventLoop) -> Result<App> {
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

      let app = App::new(window, game, renderer, keys);
      Ok(app)
    }

    let _app = self.app.get_or_init(|| create_app(event_loop));
    // "Check" the app and potentially trigger an event loop exit if
    // we failed part of the initialization.
    let _app = self.app(event_loop);
  }

  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    _window_id: WindowId,
    event: WindowEvent,
  ) {
    if let Some(app) = self.app(event_loop) {
      let change = match event {
        WindowEvent::Focused(focused) => {
          let () = app.on_focus_event(focused);
          Change::Unchanged
        },
        WindowEvent::CloseRequested => {
          let () = event_loop.exit();
          return
        },
        WindowEvent::RedrawRequested => {
          let () = app.render();
          Change::Unchanged
        },
        WindowEvent::Resized(phys_size) => {
          let phys_w = NonZeroU32::new(phys_size.width)
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
          let phys_h = NonZeroU32::new(phys_size.height)
            .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });

          let () = app.ops_mut().on_resize(phys_w, phys_h);
          let () = app.on_window_resize(phys_w, phys_h);
          Change::Changed
        },
        _ => Change::Unchanged,
      };

      match change {
        Change::Changed => app.ops().request_redraw(),
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
    if let Some(app) = self.app(event_loop) {
      if let DeviceEvent::Key(RawKeyEvent {
        physical_key: PhysicalKey::Code(key),
        state: key_state,
      }) = event
      {
        let now = Instant::now();
        match key_state {
          ElementState::Pressed => app.on_key_press(key, now),
          ElementState::Released => app.on_key_release(key, now),
        }
      }
    }
  }

  fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
    if let Some(app) = self.app(event_loop) {
      let (change, tick) = app.tick();
      let control_flow = match tick {
        Tick::None => ControlFlow::Wait,
        Tick::At(wait_until) => ControlFlow::WaitUntil(wait_until),
      };
      let () = event_loop.set_control_flow(control_flow);

      match change {
        Change::Changed => app.ops().request_redraw(),
        Change::Quit => event_loop.exit(),
        Change::Unchanged => (),
      }
    }
  }
}


pub fn run_app() -> Result<()> {
  let event_loop = EventLoop::new().context("failed to create event loop")?;
  let () = event_loop.set_control_flow(ControlFlow::Wait);
  let mut handler = Handler::default();
  let () = event_loop.run_app(&mut handler)?;
  if let Some(result) = handler.app.into_inner() {
    result.map(|_app| ())
  } else {
    Ok(())
  }
}
