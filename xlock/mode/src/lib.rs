// Copyright (C) 2023-2024 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

#![allow(clippy::let_unit_value)]

use std::mem::size_of;
use std::mem::transmute;
use std::num::NonZeroU32;
use std::os::raw::c_char;
use std::ptr;
use std::time::Instant;

use raw_window_handle::XlibDisplayHandle;
use raw_window_handle::XlibWindowHandle;

use winit::event_loop::EventLoop;
use winit::event_loop::EventLoopBuilder;
use winit::platform::x11::EventLoopBuilderExtX11 as _;

use tetromino_impl::Change;
use tetromino_impl::Game;
use tetromino_impl::GameConfig;
use tetromino_impl::Renderer;
use tetromino_impl::Window;


// SAFETY: `ModeSpecOpt` is just a C-style POD with all bit patterns
//         being valid.
static TETROMINO_OPTS: xlock::ModeSpecOpt =
  unsafe { transmute([0u8; size_of::<xlock::ModeSpecOpt>()]) };

/// The "description" of the module as required by xlock proper. It
/// describes relevant callbacks and contains some other data.
#[no_mangle]
static tetromino_description: xlock::ModStruct = xlock::ModStruct {
  cmdline_arg: b"tetromino\0" as *const _ as *const c_char,
  init_name: b"init_tetromino\0" as *const _ as *const c_char,
  callback_name: b"render_tetromino\0" as *const _ as *const c_char,
  release_name: b"release_tetromino\0" as *const _ as *const c_char,
  refresh_name: b"refresh_tetromino\0" as *const _ as *const c_char,
  change_name: b"change_tetromino\0" as *const _ as *const c_char,
  unused_name: ptr::null(),
  msopt: &TETROMINO_OPTS as *const _ as *mut _,
  // The number of microseconds between callbacks. 20ms means up to 50
  // changes could be displayed fluently per second. That is sufficient
  // for fluidity up to very high levels of game play.
  def_delay: 20000,
  def_count: 0,
  def_cycles: 0,
  def_size: 0,
  def_ncolors: 64,
  def_saturation: 0.0,
  def_bitmap: b"\0" as *const _ as *const c_char,
  desc: b"Plays Tetromino -- a Tetris clone\0" as *const _ as *const c_char,
  flags: 0,
  userdata: ptr::null_mut(),
};


/// Our "mode's" main state object.
struct State {
  /// The event loop we use.
  event_loop: EventLoop<()>,
  /// Relevant Tetromino related data.
  data: Option<(Window, Game, Renderer)>,
}


/// Handler for the "init" callback.
#[no_mangle]
extern "C" fn init_tetromino(mode_info: *const xlock::ModeInfo) {
  // SAFETY: The hook is always called with a valid `ModeInfo` object.
  let mode_info = unsafe { &*mode_info };
  // SAFETY: The hook is always called with a valid `lockstruct` object.
  let lock_struct = unsafe { &mut *mode_info.lockstruct };

  if lock_struct.userdata.is_null() {
    let event_loop = EventLoopBuilder::new()
      .with_any_thread(true)
      .build()
      .unwrap();

    let state = State {
      event_loop,
      data: None,
    };
    lock_struct.userdata = Box::into_raw(Box::new(state)).cast();
  }

  // We only ever set up shop on the very first screen.
  if mode_info.windowinfo.screen == 0 {
    // SAFETY: We are sure that `userdata` points to a valid `State` at
    //         this point.
    let state = unsafe { lock_struct.userdata.cast::<State>().as_mut().unwrap() };
    // At this point the Tetromino `Window` type doesn't support
    // creation of multiple instances at the same time. Make sure to
    // drop any previous data before we start over.
    state.data = None;

    // TODO: We certainly should not re-create the game when one is
    //       already present. It's unclear how to handle the window,
    //       because textures may logically "belong" to it (or at least
    //       the associated OpenGL context). We may not *have to*
    //       recreate it, conceptually. If we do, we may need to
    //       serialize the game and restore it to keep the state.
    let mut display = XlibDisplayHandle::empty();
    display.display = unsafe { transmute(mode_info.windowinfo.display) };
    display.screen = mode_info.windowinfo.screen;

    let mut window = XlibWindowHandle::empty();
    window.window = mode_info.windowinfo.window;
    window.visual_id = unsafe { (*(*mode_info.screeninfo).visual).visualid };

    let phys_w = NonZeroU32::new(u32::try_from(mode_info.windowinfo.width).unwrap_or_default())
      .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
    let phys_h = NonZeroU32::new(u32::try_from(mode_info.windowinfo.height).unwrap_or_default())
      .unwrap_or_else(|| unsafe { NonZeroU32::new_unchecked(1) });
    let window = Window::from_xlib_data(&state.event_loop, display, window as _).unwrap();

    let mut config = GameConfig::default();
    config.start_level = 200;
    config.enable_ai = true;

    let game = Game::with_config(&config).unwrap();
    let renderer = Renderer::new(phys_w, phys_h, game.width(), game.height());

    state.data = Some((window, game, renderer));
  } else {
    // TODO: We probably still want to be sure to clear the window on
    //       all other screens.
  }
}

/// "Tick" the game.
fn tick(state: &mut State, force_render: bool) {
  if let Some((window, game, renderer)) = &mut state.data {
    let now = Instant::now();
    let (change, _wait) = game.tick(now);

    if change == Change::Changed || force_render {
      let context = window.context_mut();
      let renderer = renderer.on_pre_render(context);
      let () = game.render(&renderer);
      let () = drop(renderer);
      let () = context.swap_buffers();
    }
  }
}

/// Handler for the "render" callback.
#[no_mangle]
extern "C" fn render_tetromino(mode_info: *const xlock::ModeInfo) {
  // SAFETY: The hook is always called with a valid `ModeInfo` object.
  let mode_info = unsafe { &*mode_info };
  // SAFETY: The hook is always called with a valid `lockstruct` object.
  let lock_struct = unsafe { &mut *mode_info.lockstruct };
  // SAFETY The "render" callback is only ever invoked after "init", so
  //        we are sure we have a `State` object set.
  let state = unsafe { lock_struct.userdata.cast::<State>().as_mut().unwrap() };

  let force_render = false;
  tick(state, force_render)
}

/// Handler for the "refresh" callback.
#[no_mangle]
extern "C" fn refresh_tetromino(mode_info: *const xlock::ModeInfo) {
  // SAFETY: The hook is always called with a valid `ModeInfo` object.
  let mode_info = unsafe { &*mode_info };
  // SAFETY: The hook is always called with a valid `lockstruct` object.
  let lock_struct = unsafe { &mut *mode_info.lockstruct };
  // SAFETY The "refresh" callback is only ever invoked after "init", so
  //        we are sure we have a `State` object set.
  let state = unsafe { lock_struct.userdata.cast::<State>().as_mut().unwrap() };

  let force_render = true;
  tick(state, force_render)
}

/// Handler for the "change" callback.
///
/// We restart the game when receiving this callback.
#[no_mangle]
extern "C" fn change_tetromino(mode_info: *const xlock::ModeInfo) {
  // SAFETY: The hook is always called with a valid `ModeInfo` object.
  let mode_info = unsafe { &*mode_info };
  // SAFETY: The hook is always called with a valid `lockstruct` object.
  let lock_struct = unsafe { &mut *mode_info.lockstruct };
  // SAFETY The "change" callback is only ever invoked after "init", so
  //         we are sure we have a `State` object set.
  let state = unsafe { lock_struct.userdata.cast::<State>().as_mut().unwrap() };

  if let Some((_window, game, _renderer)) = &mut state.data {
    let _change = game.restart();
  }
}

/// Handler for the "release" callback.
#[no_mangle]
extern "C" fn release_tetromino(mode_info: *const xlock::ModeInfo) {
  // SAFETY: The hook is always called with a valid `ModeInfo` object.
  let mode_info = unsafe { &*mode_info };
  // SAFETY: The hook is always called with a valid `lockstruct` object.
  let lock_struct = unsafe { &mut *mode_info.lockstruct };
  if !lock_struct.userdata.is_null() {
    // SAFETY: If `userdata` is set it points to a valid boxed up
    //         `State` object.
    let _state = unsafe { Box::<State>::from_raw(lock_struct.userdata.cast()) };
    lock_struct.userdata = ptr::null_mut();
  }
}
