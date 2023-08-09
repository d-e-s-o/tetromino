// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::num::NonZeroU32;

use anyhow::Context as _;
use anyhow::Result;

use glutin::config::ConfigTemplateBuilder;
use glutin::context::ContextApi;
use glutin::context::ContextAttributesBuilder;
use glutin::context::NotCurrentGlContextSurfaceAccessor as _;
use glutin::context::Version;
use glutin::display::Display;
use glutin::display::DisplayApiPreference;
use glutin::display::GlDisplay as _;
use glutin::platform::x11::X11GlConfigExt as _;
use glutin::surface::SurfaceAttributesBuilder;
use glutin::surface::WindowSurface;

use raw_window_handle::HasRawDisplayHandle as _;
use raw_window_handle::HasRawWindowHandle as _;

use winit::event_loop::EventLoop;
use winit::platform::x11::register_xlib_error_hook;
use winit::platform::x11::WindowBuilderExtX11 as _;
use winit::window::Window as WinitWindow;
use winit::window::WindowBuilder;


/// The Tetris window.
pub(crate) struct Window {
  /// The underlying `winit` window.
  _window: WinitWindow,
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
      window.with_x11_visual(x11_visual.into_raw())
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
    let size = window.inner_size();
    let phys_w = NonZeroU32::new(size.width).context("window width is zero")?;
    let phys_h = NonZeroU32::new(size.height).context("window height is zero")?;
    let attrs =
      SurfaceAttributesBuilder::<WindowSurface>::default().build(raw_window_handle, phys_w, phys_h);
    let surface = unsafe { display.create_window_surface(&config, &attrs) }
      .context("failed to create window surface")?;
    // It is essential to make the context current before calling `gl::load_with`.
    let context = unsafe { display.create_context(&config, &context_attributes) }
      .context("failed to create context")?
      .make_current(&surface)
      .context("failed to make context current")?;

    let slf = Self { _window: window };
    Ok(slf)
  }
}
