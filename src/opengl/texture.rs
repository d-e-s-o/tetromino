// Copyright (C) 2023 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ptr;
use std::rc::Rc;

use anyhow::ensure;
use anyhow::Result;

use image::DynamicImage;

use super::gl;


fn create_texture(image: image::DynamicImage) -> Result<()> {
  let width = image.width() as _;
  let height = image.height() as _;

  // We can only handle certain formats out of the box. If the image
  // is not in one of those (unlikely, but possible), convert it
  // accordingly.
  let image = match image {
    DynamicImage::ImageRgb8(..)
    | DynamicImage::ImageRgba8(..)
    | DynamicImage::ImageRgb16(..)
    | DynamicImage::ImageRgba16(..) => image,
    _ => DynamicImage::ImageRgba16(image.into_rgba16()),
  };

  let (intern_format, pixel_format, color_count, color_format) = match image {
    DynamicImage::ImageRgb8(..) => (gl::RGB8, gl::RGB, 3, gl::UNSIGNED_BYTE),
    DynamicImage::ImageRgba8(..) => (gl::RGBA8, gl::RGBA, 4, gl::UNSIGNED_BYTE),
    DynamicImage::ImageRgb16(..) => (gl::RGB16, gl::RGB, 3, gl::UNSIGNED_SHORT),
    DynamicImage::ImageRgba16(..) => (gl::RGBA16, gl::RGBA, 4, gl::UNSIGNED_SHORT),
    _ => unreachable!(),
  };

  let bytes = image.into_bytes();
  let pixels = bytes.as_ptr().cast();

  unsafe {
    // `gl::GetIntegerv` with `gl::MAX_TEXTURE_SIZE` seems to be buggy
    // (it simply returns 0). So we use texture proxies instead.
    gl::TexImage2D(
      gl::PROXY_TEXTURE_2D,
      0,
      color_count,
      width,
      height,
      0,
      pixel_format,
      color_format,
      ptr::null(),
    );

    let mut w = 0;
    let mut h = 0;
    gl::GetTexLevelParameteriv(gl::PROXY_TEXTURE_2D, 0, gl::TEXTURE_WIDTH, &mut w);
    gl::GetTexLevelParameteriv(gl::PROXY_TEXTURE_2D, 0, gl::TEXTURE_HEIGHT, &mut h);
    debug_assert_eq!(gl::GetError(), gl::NO_ERROR);

    ensure!(
      w != 0 && h != 0,
      "texture size {width}x{height} is unsupported"
    );

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as _);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as _);

    gl::PushAttrib(gl::PIXEL_MODE_BIT);
    // TODO: Probably not the full story.
    gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);

    gl::TexImage2D(
      gl::TEXTURE_2D,
      0,
      intern_format as _,
      width,
      height,
      0,
      pixel_format,
      color_format,
      pixels,
    );

    gl::PopAttrib();
    debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
  }
  Ok(())
}


#[derive(Clone, Debug)]
pub(crate) struct Texture {
  /// The texture ID.
  id: Rc<gl::GLuint>,
}

impl Texture {
  /// Create a new `Texture` from the provided image.
  pub fn with_image(image: DynamicImage) -> Result<Self> {
    let mut id = 0;

    unsafe {
      gl::GenTextures(1, &mut id);
      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }

    ensure!(id != 0, "failed to generate OpenGL texture ID");

    let texture = Self { id: Rc::new(id) };
    let () = texture.bind();
    let () = create_texture(image)?;
    let () = texture.unbind();

    Ok(texture)
  }

  pub(super) fn invalid() -> Self {
    Self { id: Rc::new(0) }
  }

  pub(super) fn bind(&self) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, *self.id);
      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
  }

  fn unbind(&self) {
    unsafe {
      gl::BindTexture(gl::TEXTURE_2D, 0);
      debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
    }
  }
}

impl Drop for Texture {
  fn drop(&mut self) {
    if let Some(id) = Rc::get_mut(&mut self.id) {
      unsafe {
        gl::DeleteTextures(1, id);

        debug_assert_eq!(gl::GetError(), gl::NO_ERROR);
      }
    }
  }
}
