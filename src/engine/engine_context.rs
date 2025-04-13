use crate::engine;

pub struct FrameData<'a> {
    pub delta_time: f64,
    pub input_system: engine::InputSystem<'a>,
    pub messages: glfw::FlushedMessages<'a, (f64, glfw::WindowEvent)>,
}

pub struct EngineContext {
    glfw_context: engine::GLFWContext,
    window: engine::Window,
    pub delta_time: f64,
}

impl EngineContext {
    pub fn new(window_title: &str, window_width: u32, window_height: u32) -> Self {
        let mut glfw_context = engine::GLFWContext::new();
        let window = glfw_context.create_window(window_title, window_width, window_height);
        Self {
            glfw_context,
            window,
            delta_time: 0.0,
        }
    }
    pub fn initialize(&mut self) {
        self.window.initialize();
        self.glfw_context.initialize();
        self.window.update_framebuffer_size();
    }

    pub fn get_input_sytem(&self) -> engine::input_system::InputSystem<'_> {
        self.window.get_input_sytem()
    }

    fn swap_buffers_and_poll_events(&mut self) {
        self.window.swap_buffers();

        self.glfw_context.poll_events();
    }

    /// Returns messages(also called events) of the window.
    ///
    /// The return is an iterator of the events that happend.
    fn flush_messages(&self) -> glfw::FlushedMessages<(f64, glfw::WindowEvent)> {
        self.window.flush_messages()
    }

    pub fn next_frame(&mut self) -> FrameData<'_> {
        self.delta_time = self.glfw_context.get_time() - self.delta_time;
        self.swap_buffers_and_poll_events();

        FrameData {
            delta_time: self.delta_time,
            input_system: self.get_input_sytem(),
            messages: self.flush_messages(),
        }
    }

    pub fn set_should_terminate(&mut self, should_terminate: bool) {
        self.window.set_should_close(should_terminate)
    }

    pub fn display_pixels(&self, pixel_data: &Vec<u8>) {}
}
