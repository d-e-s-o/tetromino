// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::num::NonZeroU32;

use anyhow::Context as _;
use anyhow::Result;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::ContextApi;
use glutin::context::ContextAttributesBuilder;
use glutin::context::NotCurrentGlContextSurfaceAccessor as _;
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

use raw_window_handle::HasRawDisplayHandle as _;
use raw_window_handle::HasRawWindowHandle as _;

use winit::event_loop::EventLoop;
use winit::platform::x11::register_xlib_error_hook;
use winit::platform::x11::WindowBuilderExtX11 as _;
use winit::window::Window as WinitWindow;
use winit::window::WindowBuilder;


fn window_size(window: &WinitWindow) -> (NonZeroU32, NonZeroU32) {
  let size = window.inner_size();
  let phys_w =
    NonZeroU32::new(size.width).unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
  let phys_h =
    NonZeroU32::new(size.height).unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
  (phys_w, phys_h)
}

/// The Tetris window.
pub(crate) struct Window {
  /// The underlying `winit` window.
  window: WinitWindow,
  /// The OpenGL surface that is used for rendering.
  surface: Surface<WindowSurface>,
  /// The OpenGL context used for double buffering.
  context: PossiblyCurrentContext,
}

impl Window {
  /// Create a new window using the provided `EventLoop`.
  pub(crate) fn new(event_loop: &EventLoop<()>) -> Result<Self> {
    let preference = DisplayApiPreference::Glx(Box::new(register_xlib_error_hook));
    let display = unsafe { Display::new(event_loop.raw_display_handle(), preference) }
      .context("failed to create display object")?;
    let template = ConfigTemplateBuilder::new()
      .with_alpha_size(8)
      .with_transparency(false)
      .build();
    let mut configs =
      unsafe { display.find_configs(template) }.context("failed to find OpenGL configurations")?;
    let config = configs
      .next()
      .context("failed to find any OpenGL configuration")?;

    let window = WindowBuilder::new().with_transparent(false);
    let window = if let Some(x11_visual) = config.x11_visual() {
      window.with_x11_visual(x11_visual.visual_id() as _)
    } else {
      window
    };
    let window = window
      .build(event_loop)
      .context("failed to build window object")?;

    let raw_window_handle = window.raw_window_handle();
    let context_attributes = ContextAttributesBuilder::new()
      .with_context_api(ContextApi::OpenGl(Some(Version::new(1, 3))))
      .build(Some(raw_window_handle));
    let (phys_w, phys_h) = window_size(&window);
    let attrs =
      SurfaceAttributesBuilder::<WindowSurface>::default().build(raw_window_handle, phys_w, phys_h);
    let surface = unsafe { display.create_window_surface(&config, &attrs) }
      .context("failed to create window surface")?;
    // It is essential to make the context current before calling `gl::load_with`.
    let context = unsafe { display.create_context(&config, &context_attributes) }
      .context("failed to create context")?
      .make_current(&surface)
      .context("failed to make context current")?;

    // Disable vsync. We are using demand-driven rendering and vsync
    // would cause artificial delays by synchronizing buffer swaps to
    // some video frame.
    let () = surface
      .set_swap_interval(&context, SwapInterval::DontWait)
      .context("failed to disable vsync")?;

    let slf = Self {
      window,
      surface,
      context,
    };
    Ok(slf)
  }

  /// Retrieve the window's inner size (i.e., the size of the drawable
  /// area).
  pub(crate) fn size(&self) -> (NonZeroU32, NonZeroU32) {
    window_size(&self.window)
  }

  /// Inform the window that it has been resized.
  #[inline]
  pub(crate) fn on_resize(&mut self, phys_w: NonZeroU32, phys_h: NonZeroU32) {
    let () = self.surface.resize(&self.context, phys_w, phys_h);
  }

  /// Swap the rendering buffers to activate the one that any rendering
  /// operations occurred on.
  // This method has an exclusive receiver to prevent invocation while a
  // renderer is active, because an active renderer already has an
  // exclusive reference to the window.
  #[inline]
  pub(crate) fn swap_buffers(&mut self) {
    let () = self
      .surface
      .swap_buffers(&self.context)
      .expect("failed to swap OpenGL buffers");
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
