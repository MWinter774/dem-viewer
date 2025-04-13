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
    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    // Loads texture image as an array of bytes
    pub fn load_texture_image(&self, texture_image_data: &Vec<u8>, width: usize, height: usize) {
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

    // Creates an unsigned integer texture, used for picking.
    pub fn load_unsigned_integer_texture(&self, width: usize, height: usize) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB32UI as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB_INTEGER,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
    }

    // Used for taking screenshot
    pub fn load_rgb_texture(&self, width: usize, height: usize) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width as i32,
                height as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
    }

    // Binds color texture to a bound framebuffer. Framebuffer must be bound before call to this. Used for picking.
    pub fn attach_color_texture_to_framebuffer(&self) {
        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                self.texture,
                0,
            );
        }
    }

    // Creates a depth texture, used for picking.
    pub fn load_depth_texture(&self, width: usize, height: usize) {
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH_COMPONENT as i32,
                width as i32,
                height as i32,
                0,
                gl::DEPTH_COMPONENT,
                gl::FLOAT,
                std::ptr::null(),
            );
        }
    }

    // Binds depth texture to a bound framebuffer. Framebuffer must be bound before call to this. Used for picking.
    pub fn attach_depth_texture_to_framebuffer(&self) {
        unsafe {
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                self.texture,
                0,
            );
        }
    }
}
