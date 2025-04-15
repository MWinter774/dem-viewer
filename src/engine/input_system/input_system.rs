use crate::engine::input_system;

pub struct InputSystem<'a> {
    pub keyboard: input_system::Keyboard<'a>,
    pub mouse: input_system::Mouse,
}

impl<'a> InputSystem<'a> {
    pub fn new(glfw_window: &'a glfw::PWindow, mouse_click_detector: input_system::MouseClickDetector) -> Self {
        Self {
            keyboard: input_system::Keyboard::new(glfw_window),
            mouse: input_system::Mouse::new(mouse_click_detector),
        }
    }
}
