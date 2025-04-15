use crate::engine::{camera_view, models, shader_programs};
use nalgebra_glm as glm;

pub struct HighlightRenderer {
    highlight_shader_program: shader_programs::HighlightShaderProgram,
}

impl HighlightRenderer {
    pub fn new(terrain: &models::Terrain) -> Self {
        let highlight_shader_program = shader_programs::HighlightShaderProgram::new(
            terrain
                .get_terrain_opengl_object()
                .get_terrain_vertices_vbo(),
        );

        Self {
            highlight_shader_program,
        }
    }

    pub fn render_highlight_on_terrain(
        &mut self,
        terrain: &models::Terrain,
        mvp_matrix: &glm::Mat4,
        primitive_id: u32,
        picked_points: &Vec<camera_view::CameraViewPoint>,
    ) {
        self.highlight_shader_program.use_program();
        terrain.get_terrain_opengl_object().bind_vao();

        self.highlight_shader_program
            .set_mvp_uniform_variable(mvp_matrix);

        self.highlight_shader_program
            .set_highlight_color_uniform_variable(&Self::get_opengl_color_from_camera_view_point(
                &picked_points[0],
            ));

        unsafe {
            gl::Clear(gl::DEPTH_BUFFER_BIT);
            gl::DrawElementsBaseVertex(
                gl::TRIANGLES,
                3,
                gl::UNSIGNED_INT,
                (primitive_id * 3 * std::mem::size_of::<u32>() as u32) as *const std::ffi::c_void,
                0,
            );
        }
    }

    fn get_opengl_color_from_camera_view_point(
        camera_view_point: &camera_view::CameraViewPoint,
    ) -> glm::Vec3 {
        glm::Vec3::new(
            (camera_view_point.color.z / 255.0) as f32,
            (camera_view_point.color.y / 255.0) as f32,
            (camera_view_point.color.x / 255.0) as f32,
        )
    }
}
