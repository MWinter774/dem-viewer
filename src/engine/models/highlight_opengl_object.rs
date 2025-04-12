use crate::engine::opengl;

pub struct HighlightOpenGLObject {
    highlight_vao: opengl::VAO1Buffer,
}

impl HighlightOpenGLObject {
    pub fn new() -> Self {
        let highlight_vao = opengl::VAO1Buffer::new();
        // Tells OpenGL to allocate enough space for 3 vertices of the triangle that will be highlighted
        highlight_vao.init_dynamic_array_buffer(3 * std::mem::size_of::<f32>() as isize);
        Self { highlight_vao }
    }
}
