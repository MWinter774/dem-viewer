use glfw::{Context, Glfw, GlfwReceiver, PWindow, WindowEvent};

use crate::engine::{self, input_system};

pub struct Window {
    glfw_window: PWindow,
    glfw_events: GlfwReceiver<(f64, WindowEvent)>,
    width: u32,
    height: u32,
}

impl Window {
    pub fn new(glfw_context: &mut Glfw, title: &str, width: u32, height: u32) -> Self {
        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw_context
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create window.");
        window.set_mouse_button_polling(true);

        Self {
            glfw_window: window,
            glfw_events: events,
            width,
            height,
        }
    }

    pub fn initialize(&mut self) {
        // Make the window's context current
        self.glfw_window.make_current();
        self.glfw_window.set_key_polling(true);
        self.glfw_window
            .set_framebuffer_size_callback(framebuffer_size_callback);

        unsafe {
            glfw::ffi::glfwSetInputMode(
                self.glfw_window.window_ptr(),
                glfw::ffi::CURSOR,
                glfw::ffi::CURSOR_DISABLED,
            );
        }
        self.glfw_window
            .set_cursor_pos(self.width as f64 / 2.0, self.height as f64 / 2.0);
        self.glfw_window
            .set_cursor_pos_callback(engine::input_system::mouse::mouse_movement_callback);
        self.glfw_window
            .set_mouse_button_callback(engine::input_system::mouse::mouse_button_callback);
    }

    pub fn swap_buffers(&mut self) {
        self.glfw_window.swap_buffers()
    }

    pub fn set_should_close(&mut self, should_close: bool) {
        self.glfw_window.set_should_close(should_close);
    }

    pub fn flush_messages(&self) -> glfw::FlushedMessages<(f64, WindowEvent)> {
        glfw::flush_messages(&self.glfw_events)
    }

    pub fn update_framebuffer_size(&self) {
        unsafe {
            gl::Viewport(0, 0, self.width as i32, self.height as i32);
        }
    }

    pub fn get_input_sytem(&self, mouse_click_detector: input_system::MouseClickDetector) -> engine::input_system::InputSystem<'_> {
        engine::InputSystem::new(&self.glfw_window, mouse_click_detector)
    }

    pub fn highlight(&mut self) {
        self.glfw_window.make_current();
        self.glfw_window.focus();
    }
}

fn framebuffer_size_callback(_glfw_window: &mut glfw::Window, width: i32, height: i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
}
