use crate::engine::opengl;

pub struct ScreenshotRenderer {
    fbo: opengl::Framebuffer,
    texture: opengl::Texture,
    depth_texture: opengl::Texture,
    window_width: usize,
    window_height: usize,
}

impl ScreenshotRenderer {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let fbo = opengl::Framebuffer::new();
        let texture = opengl::Texture::new();
        let depth_texture = opengl::Texture::new();

        fbo.bind_framebuffer();
        texture.bind();
        texture.load_rgb_texture(window_width, window_height);
        texture.attach_color_texture_to_framebuffer();
        depth_texture.bind();
        depth_texture.load_depth_texture(window_width, window_height);
        depth_texture.attach_depth_texture_to_framebuffer();
        unsafe {
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Error creating framebuffer for screenshots!");
            }
        }
        fbo.unbind_framebuffer();

        Self {
            fbo,
            texture,
            depth_texture,
            window_width,
            window_height,
        }
    }

    pub fn start_record_screenshot(&self) {
        self.fbo.bind_framebuffer();
        unsafe {
            gl::Viewport(
                0,
                0,
                self.window_width as gl::types::GLsizei,
                self.window_height as gl::types::GLsizei,
            );
            gl::ClearColor(0.0, 0.0, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn screenshot(&self) -> Vec<u8> {
        let mut pixel_data = vec![0u8; (self.window_width * self.window_height * 3) as usize];

        unsafe {
            gl::ReadPixels(
                0,
                0,
                self.window_width as gl::types::GLsizei,
                self.window_height as gl::types::GLsizei,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                pixel_data.as_mut_ptr() as *mut std::ffi::c_void,
            );
        }
        pixel_data
    }

    pub fn stop_record_screenshot(&self) {
        self.fbo.unbind_framebuffer();
    }
}
