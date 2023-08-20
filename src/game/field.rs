// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ActiveRenderer as Renderer;
use crate::Color;
use crate::Point;
use crate::Rect;
use crate::Texture;

use super::Matrix;


/// The width of each wall.
const WALL_WIDTH: u16 = 1;


pub(crate) struct Field {
  /// The location of the lower left corner of the field, in game units.
  location: Point<u16>,
  /// The inner field area, containing dropped pieces.
  pieces: PieceField,
  /// The texture to use for one unit of wall.
  wall: Texture,
}

impl Field {
  pub(super) fn new(location: Point<u16>, width: u16, height: u16, piece: Texture, back: Texture) -> Self {
    let pieces = PieceField::new(width, height, back);

    Self {
      location,
      pieces,
      // The walls just use the "piece" texture.
      wall: piece,
    }
  }

  /// Render the walls of the field.
  fn render_walls(&self, renderer: &Renderer) {
    let _guard = renderer.set_texture(&self.wall);
    let _guard = renderer.set_color(Color::orange());

    let left = Rect::new(0, 0, WALL_WIDTH, self.height());
    let () = renderer.render_rect_with_tex_coords(left, left.into_other());

    let bottom = Rect::new(0, 0, self.width(), WALL_WIDTH);
    let () = renderer.render_rect_with_tex_coords(bottom, bottom.into_other());

    let right = Rect::new(
      WALL_WIDTH + self.pieces.width(),
      0,
      WALL_WIDTH,
      self.height(),
    );
    let () = renderer.render_rect_with_tex_coords(right, right.into_other());
  }


  /// Render the Tetris field.
  pub(super) fn render(&self, renderer: &Renderer) {
    let _guard = renderer.set_origin(self.location);

    {
      let _guard = renderer.set_origin(Point::new(WALL_WIDTH, WALL_WIDTH));
      let () = self.pieces.render(renderer);
    }

    let () = self.render_walls(renderer);
  }

  #[inline]
  fn width(&self) -> u16 {
    2 * WALL_WIDTH + self.pieces.width()
  }

  #[inline]
  fn height(&self) -> u16 {
    WALL_WIDTH + self.pieces.height()
  }
}


struct PieceField {
  /// The matrix (2D array) of pieces.
  matrix: Matrix<()>,
  /// The texture to use for the entire inner back area.
  back: Texture,
}

impl PieceField {
  fn new(width: u16, height: u16, back: Texture) -> Self {
    Self {
      matrix: Matrix::new(width, height),
      back,
    }
  }

  /// Render the background of the field and draw vertical lines.
  fn render_back(&self, renderer: &Renderer) {
    // Render background image.
    {
      // TODO: Make the color configurable.
      let _guard = renderer.set_texture(&self.back);
      let _guard = renderer.set_color(Color::white());

      let () = renderer.render_rect(Rect::new(0, 0, self.width(), self.height()));
    }

    // Render the vertical lines in the background.
    {
      let _guard = renderer.set_no_texture();
      let _guard = renderer.set_color(Color::black());

      for i in 0..=self.width() {
        let () = renderer.render_line(Point::new(i, 0), Point::new(i, self.height()));
      }
    }
  }

  fn render(&self, renderer: &Renderer) {
    let () = self.render_back(renderer);
  }

  #[inline]
  fn width(&self) -> u16 {
    self.matrix.width()
  }

  #[inline]
  fn height(&self) -> u16 {
    self.matrix.height()
  }
}
