use crate::engine::{models, shader_programs};
use nalgebra_glm as glm;

pub struct PickingRenderer {
    picking_shader_program: shader_programs::PickingShaderProgram,
}

impl PickingRenderer {
    pub fn new() -> Self {
        let picking_shader_program = shader_programs::PickingShaderProgram::new();
        Self {
            picking_shader_program,
        }
    }

    pub fn render_terrain_for_picking(
        &mut self,
        terrain: &models::Terrain,
        mvp_matrix: &glm::Mat4,
    ) {
        self.picking_shader_program.use_program();
        terrain.get_terrain_opengl_object().bind_vao();

        terrain.get_terrain_opengl_object().bind_vertices_vbo();
        self.picking_shader_program.enable_vertex_attrib_array();

        self.picking_shader_program
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
