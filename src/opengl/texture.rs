// Copyright (C) 2023-2025 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use anyhow::bail;
use anyhow::Error;
use anyhow::Result;

use image::DynamicImage;

use xgl::sys;
use xgl::TextureBuilder;
use xgl::TextureInfo;

pub(crate) use xgl::Texture;


pub(crate) trait TextureBuilderExt {
  #[expect(clippy::wrong_self_convention)]
  fn from_dynamic_image(&self, image: &DynamicImage) -> Result<Texture>;
}

impl TextureBuilderExt for TextureBuilder<sys::Context> {
  fn from_dynamic_image(&self, image: &DynamicImage) -> Result<Texture> {
    let info = dynamic_image_to_tex_info(image)?;
    self.from_image(image.as_bytes(), &info)
  }
}


fn dynamic_image_to_tex_info(image: &DynamicImage) -> Result<TextureInfo, Error> {
  let (intern_format, pixel_format, color_format) = match image {
    DynamicImage::ImageRgb8(..) => (
      sys::TextureInternalFormat::RGB8,
      sys::TexturePixelFormat::RGB,
      sys::Type::UnsignedByte,
    ),
    DynamicImage::ImageRgba8(..) => (
      sys::TextureInternalFormat::RGBA8,
      sys::TexturePixelFormat::RGBA,
      sys::Type::UnsignedByte,
    ),
    _ => bail!("image format is not supported"),
  };

  let slf = TextureInfo {
    width: image.width(),
    height: image.height(),
    intern_format,
    pixel_format,
    color_format,
  };
  Ok(slf)
}


/// Create a white ("empty") 1x1 texture.
pub(crate) fn empty_texture(context: &sys::Context) -> Result<Texture> {
  let info = TextureInfo {
    width: 1,
    height: 1,
    intern_format: sys::TextureInternalFormat::RGBA8,
    pixel_format: sys::TexturePixelFormat::RGBA,
    color_format: sys::Type::UnsignedByte,
  };
  let white = Texture::builder()
    .set_context(context)
    .from_image(&[255, 255, 255, 255], &info)?;
  Ok(white)
}
