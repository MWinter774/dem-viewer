use crate::engine::{models, opengl};

const VERTICES_VBO_INDEX: usize = 0;
const INDICES_VBO_INDEX: usize = 1;

pub struct TerrainOpenGLObject {
    terrain_vao: opengl::VAO2Buffer,
}

impl TerrainOpenGLObject {
    pub fn new(terrain_render_data: &models::TerrainRenderData) -> Self {
        let terrain_vao = opengl::VAO2Buffer::new();
        Self::load_terrain_render_data_to_terrain_vao(&terrain_vao, terrain_render_data);
        Self { terrain_vao }
    }

    pub fn bind_vao(&self) {
        self.terrain_vao.bind_vertex_array();
    }

    fn load_terrain_render_data_to_terrain_vao(
        terrain_vao: &opengl::VAO2Buffer,
        terrain_render_data: &models::TerrainRenderData,
    ) {
        terrain_vao.bind_vertex_array();

        // Loads vertices of terrain to vbo object
        terrain_vao.bind_vbo_as_array_buffer(VERTICES_VBO_INDEX);
        terrain_vao.load_array_buffer(VERTICES_VBO_INDEX, terrain_render_data.get_vertices());

        // Loads indices of terrain to vbo object
        terrain_vao.bind_vbo_as_element_array_buffer(INDICES_VBO_INDEX);
        terrain_vao.load_element_array_buffer(INDICES_VBO_INDEX, terrain_render_data.get_indices());
    }
}
