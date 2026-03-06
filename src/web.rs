// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;
use std::mem::forget;
use std::num::NonZeroU32;
use std::panic::set_hook;
use std::rc::Rc;
use std::time::Duration;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::anyhow;

use wasm_bindgen::JsCast as _;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

use web_sys::Document;
use web_sys::Event;
use web_sys::HtmlCanvasElement;
use web_sys::KeyboardEvent;
use web_sys::WebGl2RenderingContext;
use web_sys::Window;
use web_sys::js_sys::Object;
use web_sys::js_sys::Reflect;
use web_sys::window;

use xgl::sys;

use crate::Change;
use crate::Instant;
use crate::Renderer;
use crate::Tick;
use crate::app::App;
use crate::app::Ops;
use crate::game;
use crate::game::Game;
use crate::keys;
use crate::keys::Keys as KeysT;

type Keys<K> = KeysT<K, Instant>;
type OpsState = (Document, HtmlCanvasElement, sys::Context);


/// Set up a panic hook.
fn init_panic_hook() {
  set_hook(Box::new(move |info| {
    let msg = if let Some(s) = info.payload().downcast_ref::<&'static str>() {
      *s
    } else if let Some(s) = info.payload().downcast_ref::<String>() {
      &s[..]
    } else {
      "<unknown>"
    };

    let location = if let Some(location) = info.location() {
      format!(
        ", {}:{}:{}",
        location.file(),
        location.line(),
        location.column()
      )
    } else {
      String::new()
    };

    eprintln!("Panicked at '{msg}'{location}");
  }));
}


fn window_size(window: &Window) -> (NonZeroU32, NonZeroU32) {
  let dpr = window.device_pixel_ratio();
  // Compute display size in device pixels.
  let width = (window.inner_width().unwrap().as_f64().unwrap() * dpr).round() as u32;
  let height = (window.inner_height().unwrap().as_f64().unwrap() * dpr).round() as u32;
  let width = NonZeroU32::new(if width != 0 { width } else { 1 }).unwrap();
  let height = NonZeroU32::new(if height != 0 { height } else { 1 }).unwrap();
  (width, height)
}


impl Ops for OpsState {
  fn context(&self) -> &sys::Context {
    &self.2
  }
}


struct StateInner {
  app: App<OpsState>,
  window: Window,
  tick: Closure<dyn FnMut()>,
}

impl StateInner {
  fn new(app: App<OpsState>, window: Window) -> Rc<RefCell<Self>> {
    let slf = Self {
      app,
      window,
      tick: Closure::wrap(Box::new(|| {
        unreachable!();
        #[allow(unreachable_code)]
        ()
      }) as Box<dyn FnMut()>),
    };
    let slf = Rc::new(RefCell::new(slf));
    let state = Rc::clone(&slf);

    let tick = Closure::wrap(Box::new(move || {
      let mut state = state.borrow_mut();
      let () = state.tick_now();
    }) as Box<dyn FnMut()>);

    {
      let mut slf = slf.borrow_mut();
      slf.tick = tick;

      // Make sure to trigger a first resize event so that everything is
      // adjusted initially. This includes the canvas, which we
      // otherwise never touch.
      let () = slf.on_resize();
    }

    slf
  }

  fn on_resize(&mut self) {
    let (width, height) = window_size(&self.window);
    let (_doc, canvas, _context) = self.app.ops_mut();
    let () = canvas.set_width(width.get());
    let () = canvas.set_height(height.get());
    let () = self.app.on_window_resize(width, height);
    let () = self.app.render();
  }

  fn tick_now(&mut self) {
    self.tick(Instant::now())
  }

  fn tick(&mut self, now: Instant) {
    let (change, tick) = self.app.tick_at(now);

    match change {
      Change::Changed => {
        let () = self.app.render();
      },
      Change::Quit => return,
      Change::Unchanged => (),
    }

    match tick {
      Tick::None => (),
      Tick::At(wait_until) => {
        let delay = (wait_until - now).as_millis() as _;
        let _id = self
          .window
          .set_timeout_with_callback_and_timeout_and_arguments_0(
            self.tick.as_ref().unchecked_ref(),
            delay,
          )
          .unwrap();
      },
    }
  }

  fn start_loop(slf: &Rc<RefCell<Self>>) -> Result<()> {
    let slf = slf.borrow();
    // Kick off the game "loop".
    let _id = slf
      .window
      .set_timeout_with_callback(slf.tick.as_ref().unchecked_ref())
      .map_err(|_| anyhow!("failed to set_timeout()"))?;
    Ok(())
  }

  fn make_on_resize(slf: &Rc<RefCell<Self>>) -> Result<Closure<dyn FnMut()>> {
    let state = Rc::clone(slf);
    let on_resize = Closure::wrap(Box::new(move || {
      let () = state.borrow_mut().on_resize();
    }) as Box<dyn FnMut()>);

    let () = slf
      .borrow()
      .window
      .add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())
      .map_err(|_| anyhow!("failed to register 'resize' event listener"))?;

    Ok(on_resize)
  }

  fn make_on_key_down(slf: &Rc<RefCell<Self>>) -> Result<Closure<dyn FnMut(KeyboardEvent)>> {
    let state = Rc::clone(slf);
    let on_down = Closure::wrap(Box::new(move |event: KeyboardEvent| {
      let key = event.key();
      let now = Instant::now();
      let mut state = state.borrow_mut();
      let () = state.app.on_key_press(key, now);
      let () = state.tick(now);
    }) as Box<dyn FnMut(KeyboardEvent)>);

    let () = slf
      .borrow()
      .window
      .add_event_listener_with_callback("keydown", on_down.as_ref().unchecked_ref())
      .map_err(|_| anyhow!("failed to register 'keydown' event listener"))?;

    Ok(on_down)
  }

  fn make_on_key_up(slf: &Rc<RefCell<Self>>) -> Result<Closure<dyn FnMut(KeyboardEvent)>> {
    let state = Rc::clone(slf);
    let on_up = Closure::wrap(Box::new(move |event: KeyboardEvent| {
      let key = event.key();
      let now = Instant::now();
      let mut state = state.borrow_mut();
      let () = state.app.on_key_release(key, now);
      let () = state.tick(now);
    }) as Box<dyn FnMut(KeyboardEvent)>);

    let () = slf
      .borrow()
      .window
      .add_event_listener_with_callback("keyup", on_up.as_ref().unchecked_ref())
      .map_err(|_| anyhow!("failed to register 'keyup' event listener"))?;

    Ok(on_up)
  }

  fn make_on_focus(slf: &Rc<RefCell<Self>>) -> Result<Closure<dyn FnMut(Event)>> {
    let state = Rc::clone(slf);
    let on_focus = Closure::wrap(Box::new(move |_event: Event| {
      let () = state.borrow_mut().app.on_focus_event(true);
    }) as Box<dyn FnMut(Event)>);

    let () = slf
      .borrow()
      .window
      .add_event_listener_with_callback("focus", on_focus.as_ref().unchecked_ref())
      .map_err(|_| anyhow!("failed to register 'focus' event listener"))?;

    Ok(on_focus)
  }

  fn make_on_blur(slf: &Rc<RefCell<Self>>) -> Result<Closure<dyn FnMut(Event)>> {
    let state = Rc::clone(slf);
    let on_blur = Closure::wrap(Box::new(move |_event: Event| {
      let () = state.borrow_mut().app.on_focus_event(false);
    }) as Box<dyn FnMut(Event)>);

    let () = slf
      .borrow()
      .window
      .add_event_listener_with_callback("blur", on_blur.as_ref().unchecked_ref())
      .map_err(|_| anyhow!("failed to register 'blur' event listener"))?;

    Ok(on_blur)
  }
}


struct State {
  _state: Rc<RefCell<StateInner>>,
  _on_resize: Closure<dyn FnMut()>,
  _on_key_down: Closure<dyn FnMut(KeyboardEvent)>,
  _on_key_up: Closure<dyn FnMut(KeyboardEvent)>,
  _on_focus: Closure<dyn FnMut(Event)>,
  _on_blur: Closure<dyn FnMut(Event)>,
}

impl State {
  fn new(app: App<OpsState>, window: Window) -> Result<State> {
    let state = StateInner::new(app, window);
    let () = StateInner::start_loop(&state)?;

    let slf = Self {
      _on_resize: StateInner::make_on_resize(&state)?,
      _on_key_down: StateInner::make_on_key_down(&state)?,
      _on_key_up: StateInner::make_on_key_up(&state)?,
      _on_focus: StateInner::make_on_focus(&state)?,
      _on_blur: StateInner::make_on_blur(&state)?,
      _state: state,
    };
    Ok(slf)
  }
}


#[wasm_bindgen]
pub fn run(canvas: JsValue) -> Result<(), JsValue> {
  fn init_impl(canvas: JsValue) -> Result<()> {
    let () = init_panic_hook();

    let window = window().context("no window found; not running inside a browser?")?;

    let canvas = canvas
      .dyn_into::<HtmlCanvasElement>()
      .map_err(|_| anyhow!("'canvas' argument must be a <canvas> object"))?;
    let document = canvas
      .owner_document()
      .context("failed to find owning document for canvas")?;

    let opts = Object::new();
    let _result = Reflect::set(
      &opts,
      &JsValue::from_str("premultipliedAlpha"),
      &JsValue::from_bool(false),
    );
    let _result = Reflect::set(
      &opts,
      &JsValue::from_str("preserveDrawingBuffer"),
      // Do not preserve the drawing buffer after `glFlush`, which
      // allows for some optimizations.
      &JsValue::from_bool(false),
    );
    let _result = Reflect::set(
      &opts,
      &JsValue::from_str("antialias"),
      // We don't really have any diagonal lines and/or animations
      // moving them. As such, there really is little benefit to
      // anti-aliasing.
      &JsValue::from_bool(false),
    );
    let _result = Reflect::set(
      &opts,
      &JsValue::from_str("alpha"),
      // `alpha` here basically is only concerned with blending between
      // the canvas contents and the page. We want to treat the canvas
      // as fully opaque, or colors will be off.
      &JsValue::from_bool(false),
    );
    let _result = Reflect::set(
      &opts,
      &JsValue::from_str("depth"),
      &JsValue::from_bool(false),
    );
    let _result = Reflect::set(
      &opts,
      &JsValue::from_str("stencil"),
      &JsValue::from_bool(false),
    );

    let gl = canvas
      .get_context_with_context_options("webgl2", &JsValue::from(opts))
      .map_err(|_| anyhow!("failed to get WebGL2 context"))?
      .context("failed to get WebGL2 context: not supported?")?
      .dyn_into::<WebGl2RenderingContext>()
      .map_err(|_| anyhow!("retrieved context is not a WebGl2RenderingContext"))?;
    if cfg!(feature = "debug")
      && let Some(attrs) = gl.get_context_attributes()
    {
      eprintln!("WebGL2 context attributes: {attrs:?}");
    }

    let context = sys::Context::new(gl);

    let config = game::Config::default();
    let game = Game::with_config(&config, &context).context("failed to instantiate game object")?;
    let (width, height) = window_size(&window);
    let renderer = Renderer::new(width, height, game.width(), game.height(), &context)
      .context("failed to create WebGL renderer")?;
    let config = keys::Config::default();
    let timeout = Duration::from_millis(config.auto_repeat_timeout_ms.into());
    let interval = Duration::from_millis(config.auto_repeat_interval_ms.into());
    let keys = Keys::new(timeout, interval);

    let ops_state = (document.clone(), canvas, context);
    let app = App::new(ops_state, game, renderer, keys);
    let state = State::new(app, window).context("failed to instantiate application state")?;
    let () = forget(state);

    Ok(())
  }

  init_impl(canvas).map_err(|err| JsValue::from_str(&format!("{err:?}")))
}
