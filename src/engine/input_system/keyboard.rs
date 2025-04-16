use crate::engine::input_system;

pub struct Keyboard<'a> {
    glfw_window: &'a glfw::PWindow,
    keyboard_press_detector: input_system::KeyboardPressDetector,
}

impl<'a> Keyboard<'a> {
    pub fn new(
        glfw_window: &'a glfw::PWindow,
        keyboard_press_detector: input_system::KeyboardPressDetector,
    ) -> Self {
        Self {
            glfw_window,
            keyboard_press_detector,
        }
    }

    pub fn get_key_press_state(&self, key: glfw::Key) -> glfw::Action {
        self.glfw_window.get_key(key)
    }

    pub fn is_key_pressed(&self, key: glfw::Key) -> bool {
        self.keyboard_press_detector.is_key_pressed(key)
    }
}
