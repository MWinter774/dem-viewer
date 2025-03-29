pub struct Texture {
    texture: gl::types::GLuint,
}

impl Texture {
    pub fn new() -> Self {
        let mut texture: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
        }
        Self { texture }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }

    // Loads texture image as an array of bytes
    pub fn load_texture_image(&self, texture_image_data: &Vec<u8>, width: usize, height: usize) {
        self.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32, // internal format
                width as i32,
                height as i32,
                0,
                gl::RGB, // format of the provided data
                gl::UNSIGNED_BYTE,
                texture_image_data.as_ptr() as *const std::ffi::c_void,
            );
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }
    }
}
