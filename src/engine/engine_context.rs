use crate::engine::{self, input_system};

pub struct FrameData<'a> {
    pub delta_time: f64,
    pub input_system: engine::InputSystem<'a>,
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

    pub fn get_input_sytem(
        &self,
        mouse_click_detector: input_system::MouseClickDetector,
        keyboard_press_detector: input_system::KeyboardPressDetector,
    ) -> engine::input_system::InputSystem<'_> {
        self.window.get_input_sytem(mouse_click_detector, keyboard_press_detector)
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
        self.delta_time = (self.glfw_context.get_time() - self.delta_time).min(1.3);
        self.swap_buffers_and_poll_events();

        let messages = self.flush_messages();
        let mut mouse_click_detector = input_system::MouseClickDetector::new();
        let mut keyboard_press_detector = input_system::KeyboardPressDetector::new();
        for (_, e) in messages {
            match e {
                glfw::WindowEvent::MouseButton(mouse_button, action, modifiers) => {
                    mouse_click_detector.update(mouse_button, action, modifiers);
                }
                glfw::WindowEvent::Key(key, scancode, action, modifiers) => {
                    keyboard_press_detector.update(key, scancode, action, modifiers);
                }
                _ => {}
            }
        }

        FrameData {
            delta_time: self.delta_time,
            input_system: self.get_input_sytem(mouse_click_detector, keyboard_press_detector),
        }
    }

    pub fn set_should_terminate(&mut self, should_terminate: bool) {
        self.window.set_should_close(should_terminate)
    }

    pub fn highlight_window(&mut self) {
        self.window.highlight();
    }
}
