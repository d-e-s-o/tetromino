// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Deref as _;

use crate::ActiveRenderer as Renderer;
use crate::Point;
use crate::Rect;
use crate::Texture;

use super::raster;

/// The coordinates of the "pixels" making up a single glyph including
/// the space to advance on the x-axis.
type Glyph = (Box<[Point<u8>]>, u8);


#[derive(Debug)]
pub(crate) struct Font {
  /// The list of glyphs making up the font.
  glyphs: Box<[Glyph]>,
  /// The point size of each glyph.
  point_size: u8,
  /// The offset at which glyphs start in the ASCII alphabet. E.g., pass
  /// in b' ' (space) if the first the first glyph represents the space
  /// character.
  offset: u8,
  /// The index of the glyph used for rendering an unsupported
  /// character.
  invalid: usize,
  /// The texture to use for each and every "pixel".
  texture: Texture,
}

impl Font {
  /// Load a font from an array of bitmaps of glyphs.
  fn load<const SIZE: usize, const N: usize>(
    glyphs: &[[u8; SIZE]; N],
    spaces: &[u8; N],
    ascii_offset: u8,
    invalid_idx: usize,
    texture: Texture,
  ) -> Self {
    assert!(invalid_idx < N, "{invalid_idx} : {N}");

    let mut glyph_coords = Vec::with_capacity(SIZE);

    for (glyph, space) in glyphs.iter().zip(spaces) {
      let mut coords = Vec::new();
      // Iterate over the bytes of the glyph, starting with the ones
      // corresponding to y=0.
      for (y, byte) in glyph.iter().enumerate() {
        // Iterate over bits of the byte, starting with highest bit.
        // TODO: May be possible to speed up with usage of
        //       `u8::leading_ones()` and similar functions, or even
        //       more by using SIMD.
        for (x, i) in (0..u8::BITS).rev().enumerate() {
          // For each bit set we remember the coordinate for later use.
          if byte & (0b1 << i) != 0 {
            let () = coords.push(Point::new(x as u8, y as u8));
          }
        }
      }

      let () = glyph_coords.push((coords.into_boxed_slice(), *space));
    }

    Self {
      glyphs: glyph_coords.into_boxed_slice(),
      point_size: SIZE as u8,
      offset: ascii_offset,
      invalid: invalid_idx,
      texture,
    }
  }

  /// Instantiate the built-in font.
  pub(crate) fn builtin(texture: Texture) -> Self {
    let invalid_idx = raster::GLYPHS.len() - 1;
    Self::load(&raster::GLYPHS, &raster::SPACES, b' ', invalid_idx, texture)
  }

  /// Renderer a string using the provided font size, in game units.
  ///
  /// # Notes
  /// If the string contains non-ASCII characters the result may not be
  /// as expected.
  pub(crate) fn render_str(
    &self,
    renderer: &Renderer,
    location: Point<f32>,
    s: &[u8],
    size: f32,
  ) -> (f32, f32) {
    let mut location = location;
    let start_x = location.x;
    let factor = size / f32::from(self.point_size);
    let _guard = renderer.set_texture(&self.texture);

    for c in s {
      let (glyph, space) = c
        .checked_sub(self.offset)
        .and_then(|idx| self.glyphs.get(usize::from(idx)))
        .or_else(|| self.glyphs.get(self.invalid))
        .unwrap();

      for coord in glyph.deref() {
        let () = renderer.render_rect_f32(Rect::new(
          location.x + f32::from(coord.x) * factor,
          location.y + f32::from(coord.y) * factor,
          factor,
          factor,
        ));
      }

      location.x += f32::from(*space) * factor;
    }

    let w = location.x - start_x;
    (w, size)
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  use test_fork::fork;

  use crate::opengl::with_opengl_context;


  /// Make sure that we can load a font correctly by spot-checking some
  /// calculated glyph coordinates.
  #[fork]
  #[test]
  fn font_loading() {
    with_opengl_context(|| {
      let font = Font::builtin(Texture::invalid());
      // Space has no coordinates to render.
      let (glyph, space) = &font.glyphs[0];
      assert!(glyph.is_empty());
      assert_eq!(*space, 3);

      // Apostrophe.
      let (glyph, space) = &font.glyphs[7];
      assert_eq!(
        glyph.deref(),
        &[Point::new(0, 8), Point::new(0, 9), Point::new(0, 10)],
      );
      assert_eq!(*space, 2);
    })
  }
}
