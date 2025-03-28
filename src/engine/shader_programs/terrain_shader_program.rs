use crate::engine::opengl;

pub struct TerrainShaderProgram {
    shader_program: opengl::ShaderProgram,
}

impl TerrainShaderProgram {
    pub fn new() -> Self {
        let shader_program =
            opengl::ShaderProgram::new("shaders\\terrain_shader.vs", "shaders\\terrain_shader.fs");
        Self { shader_program }
    }
}
