static mut LEFT_MOUSE_BUTTON_OLD_PRESSED_STATE: bool = false;

pub struct MouseClickDetector {
    is_clicked: bool,
}

impl MouseClickDetector {
    pub fn new() -> Self {
        Self { is_clicked: false }
    }

    pub fn is_left_mouse_button_clicked(&self) -> bool {
        self.is_clicked
    }

    pub fn update(
        &mut self,
        mouse_button: glfw::MouseButton,
        action: glfw::Action,
        _: glfw::Modifiers,
    ) {
        if mouse_button == glfw::MouseButtonLeft {
            unsafe {
                if LEFT_MOUSE_BUTTON_OLD_PRESSED_STATE == true && action == glfw::Action::Release {
                    self.is_clicked = true;
                }
                LEFT_MOUSE_BUTTON_OLD_PRESSED_STATE = action == glfw::Action::Press;
            }
        }
    }
}
