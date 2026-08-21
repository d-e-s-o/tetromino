// Copyright (C) 2026 Daniel Mueller <deso@posteo.net>
// SPDX-License-Identifier: GPL-3.0-or-later

use std::ops::Deref;

use anyhow::Context as _;
use anyhow::Result;

use xgl::Program;
use xgl::Shader;
use xgl::Texture;
use xgl::sys;
use xgl::sys::Gl as _;

use crate::gl::GLSL_LINEAR_TO_SRGB;


#[derive(Debug)]
pub(crate) struct BlurRenderState {
  /// The GL context.
  context: sys::Context,
  /// The program.
  program: Program,
  /// The location of the texture unit uniform.
  texture_unit_loc: sys::UniformLocation,
  /// The location of the position multiplier uniform.
  pos_multiplier_loc: sys::UniformLocation,
  /// The location of the sigma value uniform.
  sigma_loc: sys::UniformLocation,
}

impl BlurRenderState {
  pub(super) fn new(context: &sys::Context) -> Result<Self> {
    // The name of the texture uniform in the fragment shader.
    let texture_unit_uniform = "texture_unit";
    // The name of the position multiplier uniform in the fragment shader.
    let pos_multiplier_uniform = "pos_multiplier";
    // The name of the sigma uniform in the fragment shader.
    let sigma_uniform = "sigma";
    // The name of the texture coordinate attribute transferred between
    // vertex and fragment shader.
    let texture_coord_in_out = "texture_coord_in_out";

    let shader_line = line!() + 2;
    let vertex_shader_file = format!(
      r#"#version {glsl_version}
      #line {shader_line}

      // A single triangle in normalized device coordinates, covering
      // the entire screen.
      const vec2 vertices[3] = vec2[](
        vec2(-1.0, -1.0),
        vec2( 3.0, -1.0),
        vec2(-1.0,  3.0)
      );

      out vec2 {texture_coord_in_out};

      void main() {{
        vec2 pos = vertices[gl_VertexID];
        {texture_coord_in_out} = pos * 0.5 + 0.5;
        gl_Position = vec4(pos, 0.0, 1.0);
      }}
      "#,
      glsl_version = Shader::glsl_version(),
    );

    // TODO: This fragment shader is not optimized. We could
    //       pre-calculate weights as well as texture size reciprocal,
    //       as well as use separate up & down scans.
    let shader_line = line!() + 2;
    let fragment_shader_file = format!(
      r#"#version {glsl_version}
      #line {shader_line}

      precision highp float;

      uniform sampler2D {texture_unit_uniform};
      uniform float {pos_multiplier_uniform};
      uniform float {sigma_uniform};

      in vec2 {texture_coord_in_out};

      out vec4 fragment_color;

      const int kernel_size = 3;

      float gaussian_weight(float x, float y, float sigma) {{
        return exp(-(x * x + y * y) / (2.0 * sigma * sigma));
      }}

      {GLSL_LINEAR_TO_SRGB}

      void main() {{
        vec4 sum = vec4(0.0);
        float weight_sum = 0.0;
        vec2 texel_size = vec2(1.0) / vec2(textureSize({texture_unit_uniform}, 0));

        for (int y = -kernel_size; y <= kernel_size; ++y) {{
          for (int x = -kernel_size; x <= kernel_size; ++x) {{
            float w = gaussian_weight(float(x), float(y), {sigma_uniform});
            vec2 offset = vec2(x, y) * {pos_multiplier_uniform} * texel_size;
            sum += texture({texture_unit_uniform}, {texture_coord_in_out} + offset) * w;
            weight_sum += w;
          }}
        }}

        fragment_color = sum / weight_sum;
        fragment_color.rgb = linear_to_srgb(fragment_color.rgb);
      }}
      "#,
      glsl_version = Shader::glsl_version(),
    );

    let vertex_shader = Shader::new(sys::ShaderType::Vertex, &vertex_shader_file, context)
      .context("failed to create blur vertex shader")?;
    let fragment_shader = Shader::new(sys::ShaderType::Fragment, &fragment_shader_file, context)
      .context("failed to create blur fragment shader")?;
    let program = Program::new(&[vertex_shader, fragment_shader], context)?;
    let texture_unit_loc = program.query_uniform_location(texture_unit_uniform)?;
    let pos_multiplier_loc = program.query_uniform_location(pos_multiplier_uniform)?;
    let sigma_loc = program.query_uniform_location(sigma_uniform)?;

    let slf = Self {
      context: context.clone(),
      program,
      texture_unit_loc,
      pos_multiplier_loc,
      sigma_loc,
    };
    Ok(slf)
  }

  /// Activate the state and the associated GL program.
  pub fn activate(&mut self) {
    self.program.bind()
  }

  pub fn set_texture(&mut self, texture: &Texture) {
    let unit = 1;
    let () = self.context.set_active_texture_unit(unit);
    let () = texture.bind();
    let () = self
      .context
      .set_uniform_1i(&self.texture_unit_loc, unit as _);
  }

  pub fn set_blur_opts(&mut self, pos_multiplier: f32, sigma: f32) {
    let () = self
      .context
      .set_uniform_1f(&self.pos_multiplier_loc, pos_multiplier);
    let () = self.context.set_uniform_1f(&self.sigma_loc, sigma);
  }
}

impl Deref for BlurRenderState {
  type Target = sys::Context;

  fn deref(&self) -> &Self::Target {
    &self.context
  }
}
