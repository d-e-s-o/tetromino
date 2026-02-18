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

use glutin::config::Config as GlConfig;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::ContextApi;
use glutin::context::ContextAttributesBuilder;
use glutin::context::GlProfile;
use glutin::context::NotCurrentGlContext as _;
use glutin::context::PossiblyCurrentContext;
use glutin::context::Version;
use glutin::display::Display;
use glutin::display::DisplayApiPreference;
use glutin::display::GlDisplay as _;
use glutin::platform::x11::X11GlConfigExt as _;
use glutin::surface::GlSurface;
use glutin::surface::Surface;
use glutin::surface::SurfaceAttributesBuilder;
use glutin::surface::SwapInterval;
use glutin::surface::WindowSurface;

use winit::application::ApplicationHandler;
use winit::error::OsError;
use winit::event::DeviceEvent;
use winit::event::DeviceId;
use winit::event::ElementState;
use winit::event::RawKeyEvent;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::keyboard::PhysicalKey;
use winit::platform::x11::register_xlib_error_hook;
use winit::platform::x11::WindowAttributesExtX11 as _;
use winit::raw_window_handle::DisplayHandle;
use winit::raw_window_handle::HasDisplayHandle as _;
use winit::raw_window_handle::HasWindowHandle as _;
use winit::raw_window_handle::RawDisplayHandle;
use winit::raw_window_handle::RawWindowHandle;
use winit::raw_window_handle::XlibDisplayHandle;
use winit::raw_window_handle::XlibWindowHandle;
use winit::window::Window as WinitWindow;
use winit::window::WindowAttributes;
use winit::window::WindowId;

use xgl::sys;

use crate::app::App as AppT;
use crate::app::Ops;
use crate::game::Game;
use crate::keys::Keys;
use crate::opengl::Renderer;
use crate::Change;
use crate::Config;
use crate::Tick;


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


fn window_size(window: &WinitWindow) -> (NonZeroU32, NonZeroU32) {
  let size = window.inner_size();
  let phys_w =
    NonZeroU32::new(size.width).unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
  let phys_h =
    NonZeroU32::new(size.height).unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
  (phys_w, phys_h)
}


/// An OpenGL context.
///
/// A context encapsulates the OpenGL specific setup and state on a
/// window. Hence, conceptually a context is associated with a window,
/// but that does not have to be a `winit` created one (and can also
/// include one not managed at the Rust level at all).
#[derive(Debug)]
pub struct Context {
  /// The OpenGL surface that is used for rendering.
  surface: Surface<WindowSurface>,
  /// The OpenGL context used for double buffering.
  // TODO: It may be wrong to keep the context current: when creating
  //       multiple windows we end up with confusion surrounding created
  //       textures, for example, where destruction of the first window
  //       may invalidate textures in the second (because the second
  //       window's context is what is active).
  //       We may need it stored as deactivated and the only activate it
  //       for various operations (including rendering). However, this
  //       likely requires a larger API redesign, because texture
  //       creation and similar would need to be somehow tied to the
  //       window with an active context.
  render_context: PossiblyCurrentContext,
  /// The "virtual" OpenGL state context.
  gl_context: sys::Context,
}

impl Context {
  /// Create proper [`Display`] object and an associated [`GlConfig`] from
  /// a system display handle.
  fn create_display_and_config(display_handle: RawDisplayHandle) -> Result<(Display, GlConfig)> {
    let preference = DisplayApiPreference::Glx(Box::new(register_xlib_error_hook));
    let display = unsafe { Display::new(display_handle, preference) }
      .context("failed to create display object")?;
    let template = ConfigTemplateBuilder::new()
      .with_alpha_size(8)
      .with_transparency(false)
      .build();
    let display_clone = display.clone();
    let mut configs = unsafe { display_clone.find_configs(template) }
      .context("failed to find OpenGL configurations")?;
    let config = configs
      .next()
      .context("failed to find any OpenGL configuration")?;

    Ok((display, config))
  }

  /// Create a new OpenGL context on the given window.
  pub fn new(display: &Display, config: &GlConfig, window: &WinitWindow) -> Result<Self> {
    let window_handle = window
      .window_handle()
      .context("failed to retrieve window handle")?;
    let raw_window_handle = window_handle.into();
    let (phys_w, phys_h) = window_size(window);

    Self::new_impl(display, config, raw_window_handle, phys_w, phys_h)
  }

  /// Create a new OpenGL context given some xlib display & window state.
  pub fn from_xlib_data(
    display_handle: XlibDisplayHandle,
    window_handle: XlibWindowHandle,
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
  ) -> Result<Self> {
    let display_handle = RawDisplayHandle::Xlib(display_handle);
    let (display, config) = Self::create_display_and_config(display_handle)?;
    let raw_window_handle = RawWindowHandle::Xlib(window_handle);

    Self::new_impl(&display, &config, raw_window_handle, phys_w, phys_h)
  }

  fn new_impl(
    display: &Display,
    config: &GlConfig,
    raw_window_handle: RawWindowHandle,
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
  ) -> Result<Self> {
    let (major, minor, _suffix) = sys::version();
    let context_attributes = ContextAttributesBuilder::new()
      .with_context_api(ContextApi::OpenGl(Some(Version::new(major, minor))))
      .with_profile(GlProfile::Core)
      .build(Some(raw_window_handle));
    let attrs =
      SurfaceAttributesBuilder::<WindowSurface>::default().build(raw_window_handle, phys_w, phys_h);
    let surface = unsafe { display.create_window_surface(config, &attrs) }
      .context("failed to create window surface")?;
    let render_context = unsafe { display.create_context(config, &context_attributes) }
      .context("failed to create context")?
      .make_current(&surface)
      .context("failed to make context current")?;

    // Disable vsync. We are using demand-driven rendering and vsync
    // would cause artificial delays by synchronizing buffer swaps to
    // some video frame.
    let () = surface
      .set_swap_interval(&render_context, SwapInterval::DontWait)
      .context("failed to disable vsync")?;

    let gl_context = sys::Context::default();

    let slf = Self {
      surface,
      render_context,
      gl_context,
    };
    Ok(slf)
  }

  /// Inform the surface that the window has been resized.
  #[inline]
  pub fn on_resize(&mut self, phys_w: NonZeroU32, phys_h: NonZeroU32) {
    let () = self.surface.resize(&self.render_context, phys_w, phys_h);
  }

  /// Swap the rendering buffers to activate the one that any rendering
  /// operations occurred on.
  // This method has an exclusive receiver to prevent invocation while a
  // renderer is active, because an active renderer already has an
  // exclusive reference to the window.
  #[inline]
  pub fn swap_buffers(&mut self) {
    let () = self
      .surface
      .swap_buffers(&self.render_context)
      .expect("failed to swap OpenGL buffers");
  }

  /// Retrieve the underlying OpenGL "state" context.
  #[inline]
  pub fn gl_context(&self) -> &sys::Context {
    &self.gl_context
  }
}


/// The Tetris window.
///
/// # Notes
/// Please note that currently the creation of multiple windows (at the
/// same time) is not a supported workflow.
#[derive(Debug)]
pub struct Window {
  /// The underlying `winit` window.
  window: WinitWindow,
  /// The OpenGL context associated with the window.
  context: Context,
}

impl Window {
  /// Create a new window using the provided event loop.
  pub(crate) fn new<F>(display_handle: DisplayHandle<'_>, create_window_fn: F) -> Result<Self>
  where
    F: FnOnce(WindowAttributes) -> Result<WinitWindow, OsError>,
  {
    let raw_display_handle = display_handle.into();
    let (display, config) = Context::create_display_and_config(raw_display_handle)?;

    let visual = config.x11_visual().map(|visual| visual.visual_id());
    let attributes = WinitWindow::default_attributes().with_transparent(false);
    let attributes = if let Some(x11_visual) = visual {
      attributes.with_x11_visual(x11_visual as _)
    } else {
      attributes
    };
    let window = create_window_fn(attributes).context("failed to build window object")?;
    let context = Context::new(&display, &config, &window)?;
    let slf = Self { window, context };

    Ok(slf)
  }

  /// Retrieve a mutable reference to the window's render context.
  #[inline]
  pub fn render_context_mut(&mut self) -> &mut Context {
    &mut self.context
  }

  /// Retrieve the window's inner size (i.e., the size of the drawable
  /// area).
  pub fn size(&self) -> (NonZeroU32, NonZeroU32) {
    window_size(&self.window)
  }

  /// Inform the window that it has been resized.
  #[inline]
  pub fn on_resize(&mut self, phys_w: NonZeroU32, phys_h: NonZeroU32) {
    let () = self.context.on_resize(phys_w, phys_h);
  }

  /// Request a redraw of the window's contents.
  ///
  /// This method informs the system that the window's contents may be
  /// out-of-date to ultimately send a redraw event.
  #[inline]
  pub(crate) fn request_redraw(&self) {
    self.window.request_redraw()
  }
}

impl Ops for Window {
  #[inline]
  fn context(&self) -> &sys::Context {
    self.context.gl_context()
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
          let () = app.ops_mut().render_context_mut().swap_buffers();
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


/// A test helper to run a function with an OpenGL window (and context)
/// present.
///
/// # Notes
/// You most likely want to use this function in a separate process,
/// because of the effectively global OpenGL state.
#[cfg(test)]
#[expect(deprecated)]
pub(crate) fn with_opengl_context<F>(f: F)
where
  F: FnOnce(&sys::Context),
{
  use winit::event_loop::EventLoop;
  use winit::platform::x11::EventLoopBuilderExtX11 as _;
  use winit::raw_window_handle::HasDisplayHandle as _;

  let event_loop = EventLoop::builder().with_any_thread(true).build().unwrap();
  let display_handle = event_loop.display_handle().unwrap();
  // Create an invisible window, which will contain a valid OpenGL
  // context.
  let create_window_fn = |mut attrs: WindowAttributes| {
    attrs.visible = false;
    event_loop.create_window(attrs)
  };
  let _window = Window::new(display_handle, create_window_fn).unwrap();
  let context = sys::Context::default();

  f(&context)
}
