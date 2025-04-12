use crate::engine::opengl;

pub struct VAO1Buffer {
    vao_index: gl::types::GLuint,
    vbo: opengl::VBO,
}

impl VAO1Buffer {
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
        self.vbo.bind_as_array_buffer();
        self.vbo.load_data_as_array_buffer(data);
    }

    pub fn init_dynamic_array_buffer(&self, max_size_in_bytes: isize) {
        self.vbo.bind_as_array_buffer();
        self.vbo
            .init_data_as_dynamic_array_buffer(max_size_in_bytes);
    }

    pub fn load_dynamic_array_buffer<T>(&self, data: &[T]) {
        self.vbo.bind_as_array_buffer();
        self.vbo.load_data_as_dynamic_array_buffer(data);
    }

    pub fn get_vbo(&self) -> &opengl::VBO {
        &self.vbo
    }
}

impl Drop for VAO1Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao_index);
        }
    }
}
