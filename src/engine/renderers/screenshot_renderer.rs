use crate::engine::opengl;

pub struct ScreenshotRenderer {
    fbo: opengl::Framebuffer,
    texture: opengl::Texture,
}

impl ScreenshotRenderer {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let fbo = opengl::Framebuffer::new();
        let texture = opengl::Texture::new();

        fbo.bind_framebuffer();
        texture.bind();
        texture.load_unsigned_integer_texture(window_width, window_height);
        texture.attach_color_texture_to_framebuffer();
        unsafe {
            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Error creating framebuffer for screenshots!");
            }
        }

        Self { fbo, texture }
    }

    pub fn take_screenshot(&mut self) {
        self.fbo.bind_framebuffer();
    }
}
