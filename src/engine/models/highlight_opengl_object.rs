use nalgebra_glm as glm;

use crate::engine::opengl;

type HighlightData = [f32; 9];

pub struct HighlightOpenGLObject {
    highlight_vao: opengl::VAO1Buffer,
}

impl HighlightOpenGLObject {
    pub fn new() -> Self {
        let highlight_vao = opengl::VAO1Buffer::new();
        // Tells OpenGL to allocate enough space for 3 vertices of the triangle that will be highlighted
        highlight_vao.init_dynamic_array_buffer(std::mem::size_of::<HighlightData>() as isize);
        Self { highlight_vao }
    }

    pub fn load_highlight_data(&self, v1: &glm::Vec3, v2: &glm::Vec3, v3: &glm::Vec3) {
        self.highlight_vao
            .load_dynamic_array_buffer(&Self::vertices_to_highlight_data(v1, v2, v3));
    }

    fn vertices_to_highlight_data(v1: &glm::Vec3, v2: &glm::Vec3, v3: &glm::Vec3) -> HighlightData {
        [v1.x, v1.y, v1.z, v2.x, v2.y, v2.z, v3.x, v3.y, v3.z]
    }
}
