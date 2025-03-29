use crate::engine::opengl;

pub struct TerrainOpenGLObject {
    terrain_vao: opengl::VAO,
}

impl TerrainOpenGLObject {
    pub fn new() -> Self {
        let terrain_vao = opengl::VAO::new();
        Self { terrain_vao }
    }
}
