// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
use serde::Serialize;

use crate::game;
use crate::keys;


#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Config {
  /// Settings pertaining keyboard handling.
  pub keyboard: keys::Config,
  /// Configuration of the game itself.
  pub game: game::Config,
}
