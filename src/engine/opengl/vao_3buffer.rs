use crate::engine::opengl;

pub struct VAO3Buffer {
    vao_index: gl::types::GLuint,
    vbos: [opengl::VBO; 3],
}

impl VAO3Buffer {
    pub fn new() -> Self {
        let mut vao_index: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao_index);
        }
        let vbo1 = opengl::VBO::new();
        let vbo2 = opengl::VBO::new();
        let vbo3 = opengl::VBO::new();
        Self {
            vao_index,
            vbos: [vbo1, vbo2, vbo3],
        }
    }

    pub fn bind_vertex_array(&self) {
        unsafe {
            gl::BindVertexArray(self.vao_index);
        }
    }
    pub fn bind_vbo_as_element_array_buffer(&self, vbo_index: usize) {
        self.vbos[vbo_index].bind_as_element_array_buffer();
    }
    pub fn bind_vbo_as_array_buffer(&self, vbo_index: usize) {
        self.vbos[vbo_index].bind_as_array_buffer();
    }

    pub fn load_array_buffer<T>(&self, vbo_index: usize, data: &[T]) {
        self.bind_vertex_array();
        self.vbos[vbo_index].bind_as_array_buffer();
        self.vbos[vbo_index].load_data_as_array_buffer(data);
    }

    pub fn load_element_array_buffer<T>(&self, vbo_index: usize, data: &[T]) {
        self.bind_vertex_array();
        self.vbos[vbo_index].bind_as_element_array_buffer();
        self.vbos[vbo_index].load_data_as_element_array_buffer(data);
    }
}

impl Drop for VAO3Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao_index);
        }
    }
}
