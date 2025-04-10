use crate::engine::{models, opengl, utils};

const VERTICES_VBO_INDEX: usize = 0;
const VERTICES_IDS_VBO_INDEX: usize = 1;
const INDICES_VBO_INDEX: usize = 2;
const UV_VBO_INDEX: usize = 3;

pub struct TerrainOpenGLObject {
    terrain_vao: opengl::VAO4Buffer,
    terrain_texture: opengl::Texture,
}

impl TerrainOpenGLObject {
    pub fn new(terrain_render_data: &models::TerrainRenderData, texture_file_path: &str) -> Self {
        let terrain_vao = opengl::VAO4Buffer::new();
        Self::load_terrain_render_data_to_terrain_vao(&terrain_vao, terrain_render_data);
        let terrain_texture = opengl::Texture::new();
        Self::load_terrain_texture_to_opengl_texture_object(&terrain_texture, texture_file_path);
        Self {
            terrain_vao,
            terrain_texture,
        }
    }

    pub fn bind_vao(&self) {
        self.terrain_vao.bind_vertex_array();
    }
    pub fn bind_vertices_vbo(&self) {
        self.terrain_vao
            .bind_vbo_as_array_buffer(VERTICES_VBO_INDEX);
    }
    pub fn bind_vertices_ids_vbo(&self) {
        self.terrain_vao
            .bind_vbo_as_array_buffer(VERTICES_IDS_VBO_INDEX);
    }
    pub fn bind_indices_vbo(&self) {
        self.terrain_vao
            .bind_vbo_as_element_array_buffer(INDICES_VBO_INDEX);
    }
    pub fn bind_uv_vbo(&self) {
        self.terrain_vao.bind_vbo_as_array_buffer(UV_VBO_INDEX);
    }
    pub fn bind_texture(&self) {
        self.terrain_texture.bind();
    }

    fn load_terrain_texture_to_opengl_texture_object(
        texture: &opengl::Texture,
        texture_file_path: &str,
    ) {
        texture.bind();
        let (texture_image_data, width, height) = utils::load_texture_image(texture_file_path);
        texture.load_texture_image(&texture_image_data, width, height);
    }

    fn load_terrain_render_data_to_terrain_vao(
        terrain_vao: &opengl::VAO4Buffer,
        terrain_render_data: &models::TerrainRenderData,
    ) {
        terrain_vao.bind_vertex_array();

        // Loads uv texture coords of terrain to vbo object
        terrain_vao.bind_vbo_as_array_buffer(UV_VBO_INDEX);
        terrain_vao.load_array_buffer(UV_VBO_INDEX, terrain_render_data.get_uv());
        
        // Loads indices of terrain to vbo object
        terrain_vao.bind_vbo_as_element_array_buffer(INDICES_VBO_INDEX);
        terrain_vao.load_element_array_buffer(INDICES_VBO_INDEX, terrain_render_data.get_indices());
        
        // Loads vertices ids of terrain to vbo object
        terrain_vao.bind_vbo_as_array_buffer(VERTICES_IDS_VBO_INDEX);
        terrain_vao.load_array_buffer(
            VERTICES_IDS_VBO_INDEX,
            terrain_render_data.get_vertices_ids(),
        );

        // Loads vertices of terrain to vbo object
        terrain_vao.bind_vbo_as_array_buffer(VERTICES_VBO_INDEX);
        terrain_vao.load_array_buffer(VERTICES_VBO_INDEX, terrain_render_data.get_vertices());
    }
}
