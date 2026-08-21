// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Deref;

use anyhow::Result;

use xgl::sys;

use super::blur::BlurRenderState;
use super::object::ObjectRenderState;


/// An enumeration describing the currently active state.
#[derive(Clone, Copy, Debug)]
enum Active {
  Blur,
  Object,
}


/// A type encapsulating GL state.
#[derive(Debug)]
pub(crate) struct State {
  /// The GL context.
  context: sys::Context,
  /// The currently active state.
  active: Active,
  /// The blur rendering state.
  blur: BlurRenderState,
  /// The general object rendering state.
  object: ObjectRenderState,
}

impl State {
  pub(crate) fn new(context: &sys::Context) -> Result<Self> {
    let mut slf = Self {
      active: Active::Object,
      blur: BlurRenderState::new(context)?,
      object: ObjectRenderState::new(context)?,
      context: context.clone(),
    };

    let () = slf.object.activate();

    Ok(slf)
  }

  pub fn blur(&mut self) -> &mut BlurRenderState {
    match self.active {
      Active::Blur => &mut self.blur,
      _ => {
        let () = self.blur.activate();
        self.active = Active::Blur;
        &mut self.blur
      },
    }
  }

  pub fn object(&mut self) -> &mut ObjectRenderState {
    match self.active {
      Active::Object => &mut self.object,
      _ => {
        let () = self.object.activate();
        self.active = Active::Object;
        &mut self.object
      },
    }
  }
}

impl Deref for State {
  type Target = sys::Context;

  fn deref(&self) -> &Self::Target {
    &self.context
  }
}
