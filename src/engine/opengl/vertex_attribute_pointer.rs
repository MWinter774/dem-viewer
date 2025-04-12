use std::os::raw::c_void;

/// Holds a reference for a data buffer to assign to VertexAttribPointer, only to represent an OpenGL VertexAttributePointer correctly
pub struct VertexAttributePointer {
    data_config: self::VertexAttributePointerConfig,
}

/// Used for creating VertexAttributePointer, specified how the OpenGL pipeline should interpret the data that's being inserted to it
pub struct VertexAttributePointerConfig {
    pub index: gl::types::GLuint,
    pub size: gl::types::GLint,
    pub type_: gl::types::GLenum,
    pub normalized: gl::types::GLboolean,
    pub stride: gl::types::GLsizei,
    pub pointer: *const c_void,
}

/// The defualt for VertexAttributePointerConfig is for vec3 at index 0
impl Default for VertexAttributePointerConfig {
    fn default() -> Self {
        Self {
            index: 0,
            size: 3,
            type_: gl::FLOAT,
            normalized: gl::FALSE,
            stride: 0,
            pointer: std::ptr::null(),
        }
    }
}

impl VertexAttributePointer {
    pub fn new_float(data_config: self::VertexAttributePointerConfig) -> Self {
        
        unsafe {
            gl::VertexAttribPointer(
                data_config.index,
                data_config.size,
                data_config.type_,
                data_config.normalized,
                data_config.stride,
                data_config.pointer,
            );
        }

        Self { data_config }
    }
    pub fn new_integer(data_config: self::VertexAttributePointerConfig) -> Self {
        unsafe {
            gl::VertexAttribIPointer(
                data_config.index,
                data_config.size,
                data_config.type_,
                data_config.stride,
                data_config.pointer,
            );
        }

        Self { data_config }
    }

    pub fn enable_vertex_attrib_array(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.data_config.index);
        }
    }
    pub fn reload_vertex_attrib_array(&self) {
        unsafe {
            gl::VertexAttribPointer(
                self.data_config.index,
                self.data_config.size,
                self.data_config.type_,
                self.data_config.normalized,
                self.data_config.stride,
                self.data_config.pointer,
            );
        }
    }

    pub fn disable_vertex_attrib_array(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.data_config.index);
        }
    }
}
