use std::collections;

pub struct KeyboardPressDetector {
    key_states: collections::HashMap<glfw::Key, bool>,
}

impl KeyboardPressDetector {
    pub fn new() -> Self {
        Self {
            key_states: collections::HashMap::new(),
        }
    }

    pub fn is_key_pressed(&self, key: glfw::Key) -> bool {
        *self.key_states.get(&key).unwrap_or(&false)
    }

    pub fn update(
        &mut self,
        key: glfw::Key,
        scancode: glfw::Scancode,
        action: glfw::Action,
        modifiers: glfw::Modifiers,
    ) {
        self.key_states.insert(key, action == glfw::Action::Press);
    }
}
