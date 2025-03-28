use crate::engine::opengl;

pub struct VAO {
    vao_index: gl::types::GLuint,
    vbo: opengl::VBO,
}

impl VAO {
    pub fn new() -> Self {
        let mut vao_index: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_index);
        }
        Self {
            vao_index,
            vbo: opengl::VBO::new(),
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_index);
        }
    }

    pub fn load_array_buffer<T>(&self, data: &[T]) {
        self.bind();
        self.vbo.bind_array_buffer();
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao_index);
        }
    }
}
