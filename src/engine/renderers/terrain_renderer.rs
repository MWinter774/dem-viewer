use crate::engine::{models, shader_programs};
use nalgebra_glm as glm;

pub struct TerrainRenderer {
    terrain_shader_program: shader_programs::TerrainShaderProgram,
}

impl TerrainRenderer {
    pub fn new() -> Self {
        let terrain_shader_program =
            shader_programs::TerrainShaderProgram::new();
        Self {
            terrain_shader_program,
        }
    }

    pub fn render_terrain(&mut self, terrain: models::Terrain, mvp_matrix: &glm::Mat4) {
        self.terrain_shader_program.use_program();
        terrain.get_terrain_opengl_object().bind_vao();
        self.terrain_shader_program.enable_vertex_attrib_array();
        self.terrain_shader_program
            .set_mvp_uniform_variable(mvp_matrix);

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                terrain.get_terrain_render_data().get_indices().len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
