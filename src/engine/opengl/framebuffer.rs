pub struct Framebuffer {
    framebuffer_id: gl::types::GLuint,
}

impl Framebuffer {
    pub fn new() -> Self {
        let mut framebuffer_id: gl::types::GLuint = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut framebuffer_id);
        }
        Self { framebuffer_id }
    }

    pub fn bind_framebuffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.framebuffer_id);
        }
    }
    pub fn bind_draw_framebuffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.framebuffer_id);
        }
    }
    pub fn bind_read_framebuffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.framebuffer_id);
        }
    }

    pub fn unbind_framebuffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
    pub fn unbind_read_framebuffer(&self) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, 0);
        }
    }
}
