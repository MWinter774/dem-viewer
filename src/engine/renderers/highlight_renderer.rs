use crate::engine::{models, shader_programs};
use nalgebra_glm as glm;

pub struct HighlightRenderer {
    highlight_shader_program: shader_programs::HighlightShaderProgram,
}

impl HighlightRenderer {
    pub fn new() -> Self {
        let highlight_shader_program = shader_programs::HighlightShaderProgram::new();

        Self {
            highlight_shader_program,
        }
    }

    pub fn render_highlight_on_terrain(
        &mut self,
        terrain: &models::Terrain,
        highlight: &models::Highlight,
        mvp_matrix: &glm::Mat4,
        vid: u32,
    ) {
        self.highlight_shader_program.use_program();
        highlight.get_highlight_opengl_object().bind_vao();

        let (v1, v2, v3) =
            Self::get_triangle_vertices(terrain.get_terrain_render_data().get_vertices(), vid);
        highlight
            .get_highlight_opengl_object()
            .load_highlight_data(&v1, &v2, &v3);

        self.highlight_shader_program
            .set_mvp_uniform_variable(mvp_matrix);

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
