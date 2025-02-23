// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::num::NonZeroU32;

use anyhow::Context as _;
use anyhow::Result;

use glutin::config::Config;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::ContextApi;
use glutin::context::ContextAttributesBuilder;
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

use winit::error::OsError;
use winit::platform::x11::register_xlib_error_hook;
use winit::platform::x11::WindowAttributesExtX11 as _;
use winit::raw_window_handle::DisplayHandle;
use winit::raw_window_handle::HasWindowHandle as _;
use winit::raw_window_handle::RawDisplayHandle;
use winit::raw_window_handle::RawWindowHandle;
use winit::raw_window_handle::XlibDisplayHandle;
use winit::raw_window_handle::XlibWindowHandle;
use winit::window::Window as WinitWindow;
use winit::window::WindowAttributes;


fn window_size(window: &WinitWindow) -> (NonZeroU32, NonZeroU32) {
  let size = window.inner_size();
  let phys_w =
    NonZeroU32::new(size.width).unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
  let phys_h =
    NonZeroU32::new(size.height).unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
  (phys_w, phys_h)
}


/// Retrieve the OpenGL version to use.
fn opengl_version() -> (u8, u8) {
  let major = env!(
    "OPENGL_MAJOR",
    "OPENGL_MAJOR environment variable not found"
  )
  .parse()
  .expect("OPENGL_MAJOR environment variable does not contain a valid number");

  let minor = env!(
    "OPENGL_MINOR",
    "OPENGL_MINOR environment variable not found"
  )
  .parse()
  .expect("OPENGL_MINOR environment variable does not contain a valid number");

  (major, minor)
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
  context: PossiblyCurrentContext,
}

impl Context {
  /// Create proper [`Display`] object and an associated [`Config`] from
  /// a system display handle.
  fn create_display_and_config(display_handle: RawDisplayHandle) -> Result<(Display, Config)> {
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
  pub fn new(display: &Display, config: &Config, window: &WinitWindow) -> Result<Self> {
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
    config: &Config,
    raw_window_handle: RawWindowHandle,
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
  ) -> Result<Self> {
    let (major, minor) = opengl_version();
    let context_attributes = ContextAttributesBuilder::new()
      .with_context_api(ContextApi::OpenGl(Some(Version::new(major, minor))))
      .build(Some(raw_window_handle));
    let attrs =
      SurfaceAttributesBuilder::<WindowSurface>::default().build(raw_window_handle, phys_w, phys_h);
    let surface = unsafe { display.create_window_surface(config, &attrs) }
      .context("failed to create window surface")?;
    let context = unsafe { display.create_context(config, &context_attributes) }
      .context("failed to create context")?
      .make_current(&surface)
      .context("failed to make context current")?;

    // Disable vsync. We are using demand-driven rendering and vsync
    // would cause artificial delays by synchronizing buffer swaps to
    // some video frame.
    let () = surface
      .set_swap_interval(&context, SwapInterval::DontWait)
      .context("failed to disable vsync")?;

    let slf = Self { surface, context };
    Ok(slf)
  }

  /// Inform the surface that the window has been resized.
  #[inline]
  pub fn on_resize(&mut self, phys_w: NonZeroU32, phys_h: NonZeroU32) {
    let () = self.surface.resize(&self.context, phys_w, phys_h);
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
      .swap_buffers(&self.context)
      .expect("failed to swap OpenGL buffers");
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

  /// Retrieve the window's OpenGL context.
  #[inline]
  pub fn context_mut(&mut self) -> &mut Context {
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
