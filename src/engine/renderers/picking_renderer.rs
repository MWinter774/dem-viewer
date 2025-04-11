use crate::engine::{models, opengl, shader_programs};
use nalgebra_glm as glm;

pub struct PickingRenderer {
    picking_shader_program: shader_programs::PickingShaderProgram,
    picking_framebuffer: opengl::Framebuffer,
    picking_color_texture: opengl::Texture,
    picking_depth_texture: opengl::Texture,
}

impl PickingRenderer {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let picking_shader_program = shader_programs::PickingShaderProgram::new();
        let picking_framebuffer = opengl::Framebuffer::new();
        let picking_color_texture = opengl::Texture::new();
        let picking_depth_texture = opengl::Texture::new();
        Self::init_framebuffer_and_textures(
            &picking_framebuffer,
            &picking_color_texture,
            &picking_depth_texture,
            window_width,
            window_height,
        );
        Self {
            picking_shader_program,
            picking_framebuffer,
            picking_color_texture,
            picking_depth_texture,
        }
    }

    pub fn render_terrain_for_picking(
        &mut self,
        terrain: &models::Terrain,
        mvp_matrix: &glm::Mat4,
    ) {
        self.picking_shader_program.use_program();
        terrain.get_terrain_opengl_object().bind_vao();

        terrain.get_terrain_opengl_object().bind_vertices_ids_vbo();
        self.picking_shader_program.enable_vertex_id_attrib_array();

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

    fn init_framebuffer_and_textures(
        framebuffer: &opengl::Framebuffer,
        color_texture: &opengl::Texture,
        depth_texture: &opengl::Texture,
        window_width: usize,
        window_height: usize,
    ) {
        framebuffer.bind_framebuffer();

        color_texture.bind();
        color_texture.load_unsigned_integer_texture(window_width, window_height);
        color_texture.attach_color_texture_to_framebuffer();

        depth_texture.bind();
        depth_texture.load_depth_texture(window_width, window_height);
        depth_texture.attach_depth_texture_to_framebuffer();

        unsafe {
            if gl::FRAMEBUFFER_COMPLETE != gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
                panic!("Error creating framebuffer for picking!");
            }
        }

        depth_texture.unbind();
        framebuffer.unbind_framebuffer();
    }
}
