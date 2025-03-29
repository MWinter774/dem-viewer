pub struct VBO {
    vbo_index: gl::types::GLuint,
}

impl VBO {
    pub fn new() -> Self {
        let mut vbo_index: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo_index);
        }
        Self { vbo_index }
    }

    pub fn bind_as_array_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_index);
        }
    }

    pub fn bind_as_element_array_buffer(&self) {
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.vbo_index);
        }
    }

    pub fn load_data_as_element_array_buffer<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }

    pub fn load_data_as_array_buffer<T>(&self, data: &[T]) {
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

impl Drop for VBO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo_index);
        }
    }
}
