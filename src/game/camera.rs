// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::num::NonZeroU16;
use std::num::NonZeroU32;

use xgl::sys;
use xgl::sys::Gl as _;

use crate::gl::Mat4f;
use crate::gl::ObjectRenderState;


#[derive(Debug)]
pub(crate) struct Camera {
  /// The physical width of the window to which this renderer belongs.
  phys_w: NonZeroU32,
  /// The physical height of the window to which this renderer belongs.
  phys_h: NonZeroU32,
  /// The projection matrix we use.
  projection: Mat4f,
}

impl Camera {
  pub fn new(
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) -> Self {
    Self {
      phys_w,
      phys_h,
      projection: Self::calculate_view(phys_w, phys_h, logic_w, logic_h),
    }
  }

  fn calculate_view(
    phys_w: NonZeroU32,
    phys_h: NonZeroU32,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) -> Mat4f {
    let phys_w = phys_w.get() as f32;
    let phys_h = phys_h.get() as f32;
    let logic_w = logic_w.get() as f32;
    let logic_h = logic_h.get() as f32;

    let phys_ratio = phys_w / phys_h;
    let logic_ratio = logic_w / logic_h;

    let mut x = 0.0;
    let mut y = 0.0;

    // Our goal is to make the two ratios equal in order to preserve the
    // physical aspect ratio. That means:
    //
    // phys_w   logic_w + x
    // ------ = -----------
    // phys_h   logic_h + y
    //
    // where `x` is zero if `logic_ratio` > `phys_ratio`, otherwise `y`
    // is zero. Resolve it to `x` or `y` to get the equation from above.
    if logic_ratio > phys_ratio {
      y = logic_w * phys_h / phys_w - logic_h;
    } else {
      x = logic_h * phys_w / phys_h - logic_w;
    };

    // Calculate the offsets to use to center the view properly.
    let off_x = -0.5 * x;
    let off_y = -0.5 * y;

    // Our renderer will render everything with z-coordinate of 0.0f,
    // this must lie inside the range [znear, zfar].
    let znear = -0.5;
    let zfar = 0.5;

    Mat4f::orthographic(
      off_x,
      off_x + x + logic_w,
      off_y,
      off_y + y + logic_h,
      znear,
      zfar,
    )
  }

  /// Update the view after the containing window or contained logical
  /// dimensions have changed.
  pub fn update_view(
    &mut self,
    phys_w: Option<NonZeroU32>,
    phys_h: Option<NonZeroU32>,
    logic_w: NonZeroU16,
    logic_h: NonZeroU16,
  ) {
    let phys_w = phys_w.unwrap_or(self.phys_w);
    let phys_h = phys_h.unwrap_or(self.phys_h);

    self.projection = Self::calculate_view(phys_w, phys_h, logic_w, logic_h);

    self.phys_w = phys_w;
    self.phys_h = phys_h;
  }

  /// Set the viewport to the window's dimensions.
  pub fn set_viewport(&self, context: &sys::Context) {
    let () = context.set_viewport(0, 0, self.phys_w.get() as _, self.phys_h.get() as _);
  }

  pub fn render_scene<F>(&self, state: &mut ObjectRenderState, f: F)
  where
    F: FnOnce(&mut ObjectRenderState),
  {
    let () = state.set_projection(&self.projection);
    let () = f(state);
  }
}
