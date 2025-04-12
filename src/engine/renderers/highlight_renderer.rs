use crate::engine::{models, shader_programs};
use nalgebra_glm as glm;

pub struct HighlightRenderer {
    triangle_shader_program: shader_programs::HighlightShaderProgram,
}

impl HighlightRenderer {
    pub fn new() -> Self {
        let triangle_shader_program = shader_programs::HighlightShaderProgram::new();

        Self {
            triangle_shader_program,
        }
    }

    pub fn render_triangle_on_terrain(
        &mut self,
        terrain: &models::Terrain,
        mvp_matrix: &glm::Mat4,
        vid: u32,
    ) {
        self.triangle_shader_program.use_program();
        terrain.get_terrain_opengl_object().bind_vao();

        self.triangle_shader_program
            .set_mvp_uniform_variable(mvp_matrix);

        let (v1, v2, v3) =
            Self::get_triangle_vertices(terrain.get_terrain_render_data().get_vertices(), vid);

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }

    fn get_triangle_vertices(vertices: &Vec<f32>, vid: u32) -> (glm::Vec3, glm::Vec3, glm::Vec3) {
        let v1 = glm::vec3(
            vertices[(vid) as usize],
            vertices[(vid + 1) as usize],
            vertices[(vid + 2) as usize],
        );
        let v2 = glm::vec3(
            vertices[(vid + 3) as usize],
            vertices[(vid + 4) as usize],
            vertices[(vid + 5) as usize],
        );
        let v3 = glm::vec3(
            vertices[(vid + 6) as usize],
            vertices[(vid + 7) as usize],
            vertices[(vid + 8) as usize],
        );
        (v1, v2, v3)
    }
}
