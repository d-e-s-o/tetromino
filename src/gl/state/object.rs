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
use xgl::vertex::AttribType;

use crate::gl::Mat4f;


#[derive(Debug)]
pub(crate) struct ObjectRenderState {
  /// The OpenGL context.
  context: sys::Context,
  /// The program.
  _program: Program,
  /// The location of the model-view matrix uniform.
  modelview_loc: sys::UniformLocation,
  /// The location of the projection matrix uniform.
  projection_loc: sys::UniformLocation,
  /// The location of the texture unit uniform.
  texture_unit_loc: sys::UniformLocation,
  /// The attribute indices.
  attrib_indices: [(u32, AttribType); 3],
}

impl ObjectRenderState {
  pub fn new(context: &sys::Context) -> Result<Self> {
    // The name of the model-view matrix uniform in the vertex shader.
    let modelview_uniform = "modelview";
    // The name of the projection matrix uniform in the vertex shader.
    let projection_uniform = "projection";
    // The name of the texture uniform in the fragment shader.
    let texture_unit_uniform = "texture_unit";
    // The name of the color input attribute in the vertex shader.
    let color_attrib = "color";
    // The name of the position input attribute in the vertex shader.
    let position_attrib = "position";
    // The name of the texture coordinate input attribute in the vertex shader.
    let texture_coord_attrib = "texture_coord";
    // The name of the color attribute transferred between vertex and
    // fragment shader.
    let color_in_out = "color_in_out";
    // The name of the texture coordinate attribute transferred between
    // vertex and fragment shader.
    let texture_coord_in_out = "texture_coord_in_out";

    let shader_line = line!() + 2;
    let vertex_shader_file = format!(
      r#"#version {glsl_version}
      #line {shader_line}

      uniform mat4 {modelview_uniform};
      uniform mat4 {projection_uniform};

      in vec3 {position_attrib};
      in vec4 {color_attrib};
      in vec2 {texture_coord_attrib};

      out vec4 {color_in_out};
      out vec2 {texture_coord_in_out};

      void main() {{
        gl_Position = {projection_uniform} * {modelview_uniform} * vec4({position_attrib}, 1.0);

        {color_in_out} = {color_attrib};
        {texture_coord_in_out} = {texture_coord_attrib};
      }}
      "#,
      glsl_version = Shader::glsl_version(),
    );

    let shader_line = line!() + 2;
    let fragment_shader_file = format!(
      r#"#version {glsl_version}
      #line {shader_line}

      precision highp float;
      precision highp sampler2D;

      uniform sampler2D {texture_unit_uniform};

      in vec4 {color_in_out};
      in vec2 {texture_coord_in_out};

      out vec4 fragment_color;

      vec3 linear_to_srgb(vec3 color) {{
        float gamma = 2.2;
        return pow(color, vec3(1.0 / gamma));
      }}

      void main() {{
        fragment_color = texture({texture_unit_uniform}, {texture_coord_in_out}) * {color_in_out};
        fragment_color.rgb = linear_to_srgb(fragment_color.rgb);
      }}
      "#,
      glsl_version = Shader::glsl_version(),
    );

    let vertex_shader = Shader::new(sys::ShaderType::Vertex, &vertex_shader_file, context)
      .context("failed to create vertex shader")?;
    let fragment_shader = Shader::new(sys::ShaderType::Fragment, &fragment_shader_file, context)
      .context("failed to create fragment shader")?;
    let program = Program::new(&[vertex_shader, fragment_shader], context)?;
    let texture_unit_loc = program.query_uniform_location(texture_unit_uniform)?;
    let modelview_loc = program.query_uniform_location(modelview_uniform)?;
    let projection_loc = program.query_uniform_location(projection_uniform)?;
    let color_idx = program.query_attrib_location(color_attrib)?;
    let position_idx = program.query_attrib_location(position_attrib)?;
    let texture_coord_idx = program.query_attrib_location(texture_coord_attrib)?;

    // Bind the program to make sure its in effect subsequently.
    let () = program.bind();

    let slf = Self {
      context: context.clone(),
      _program: program,
      modelview_loc,
      projection_loc,
      texture_unit_loc,
      attrib_indices: [
        (color_idx, AttribType::Color),
        (position_idx, AttribType::Position),
        (texture_coord_idx, AttribType::Texture),
      ],
    };
    Ok(slf)
  }

  #[inline]
  pub fn set_modelview(&mut self, modelview: &Mat4f) {
    self
      .context
      .set_uniform_matrix(&self.modelview_loc, modelview.as_array())
  }

  #[inline]
  pub fn set_projection(&mut self, projection: &Mat4f) {
    self
      .context
      .set_uniform_matrix(&self.projection_loc, projection.as_array())
  }

  pub fn set_texture(&mut self, texture: &Texture) {
    // All our texturing uses a single unit.
    let unit = 0;
    let () = self.context.set_active_texture_unit(unit);
    let () = texture.bind();
    let () = self
      .context
      .set_uniform_1i(&self.texture_unit_loc, unit as _);
  }

  pub fn attrib_indices(&self) -> &[(u32, AttribType)] {
    &self.attrib_indices
  }
}

impl Deref for ObjectRenderState {
  type Target = sys::Context;

  fn deref(&self) -> &Self::Target {
    &self.context
  }
}
