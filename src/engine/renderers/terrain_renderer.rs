use crate::engine::models;
use crate::engine::shader_programs;

pub struct TerrainRenderer<'a> {
    terrain_shader_program: shader_programs::TerrainShaderProgram<'a>,
}

impl<'a> TerrainRenderer<'a> {
    pub fn new(terrain_render_data: &'a models::TerrainRenderData) -> Self {
        let terrain_shader_program =
            shader_programs::TerrainShaderProgram::new(terrain_render_data.get_vertices());
        Self {
            terrain_shader_program,
        }
    }
}
