use crate::engine;

pub struct GLFWContext {
    glfw: glfw::Glfw,
}

impl GLFWContext {
    pub fn new() -> Self {
        use glfw::fail_on_errors;
        Self {
            glfw: glfw::init(glfw::fail_on_errors!()).unwrap(),
        }
    }

    pub fn initialize(&mut self) {
        // Setting the loading function must be after window initialization
        gl::load_with(|s| self.glfw.get_proc_address_raw(s));

        // Set window and OpenGL settings
        self.glfw
            .window_hint(glfw::WindowHint::ContextVersion(3, 3));
        self.glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
    }

    pub fn create_window(
        &mut self,
        window_title: &str,
        window_width: u32,
        window_height: u32,
    ) -> engine::Window {
        engine::Window::new(&mut self.glfw, window_title, window_width, window_height)
    }

    /// This function poll events - meaning updating the events pipeline and getting new events.
    pub fn poll_events(&mut self) {
        // Poll for and process events
        self.glfw.poll_events();
    }

    pub fn get_time(&self) -> f64 {
        self.glfw.get_time()
    }
}
