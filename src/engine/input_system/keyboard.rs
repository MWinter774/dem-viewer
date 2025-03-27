pub struct Keyboard<'a> {
    glfw_window: &'a glfw::PWindow,
}

impl<'a> Keyboard<'a> {
    pub fn new(glfw_window: &'a glfw::PWindow) -> Self {
        Self { glfw_window }
    }

    pub fn get_key_press_state(&self, key: glfw::Key) -> glfw::Action {
        self.glfw_window.get_key(key)
    }
}
