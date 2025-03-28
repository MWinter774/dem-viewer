use crate::engine;
use engine::transformations;

extern crate nalgebra_glm as glm;

pub struct Camera {
    position: glm::Vec3,
    front: glm::Vec3,
    up: glm::Vec3,
    view_matrix: transformations::View,
    step_size: f32,
    yaw: f32,
    pitch: f32,
    sensitivity: f32,
}

impl Camera {
    pub fn new(position: glm::Vec3, front: glm::Vec3, up: glm::Vec3, step_size: f32) -> Self {
        Self {
            position,
            front,
            up,
            view_matrix: transformations::View::new(&position, &(position + front), &up),
            step_size,
            yaw: -90.0,
            pitch: 0.0,
            sensitivity: 0.1,
        }
    }

    pub fn get_view_matrix(&self) -> &glm::Mat4 {
        self.view_matrix.get_matrix()
    }

    pub fn update_input(&mut self, input_system: &engine::InputSystem, delta_time: f32) {
        let (is_keyboard_used, is_mouse_moved) = (
            self.handle_keyboard_input(&input_system.keyboard, delta_time),
            self.handle_mouse_input(&input_system.mouse, delta_time),
        );
        if is_keyboard_used || is_mouse_moved {
            self.update_view_matrix();
        }
    }

    fn update_view_matrix(&mut self) {
        self.view_matrix =
            transformations::View::new(&self.position, &(self.position + self.front), &self.up);
    }

    fn get_right_vector(&self) -> glm::Vec3 {
        self.front.cross(&self.up).normalize()
    }

    fn get_left_vector(&self) -> glm::Vec3 {
        self.up.cross(&self.front).normalize()
    }

    fn handle_mouse_input(&mut self, mouse: &engine::input_system::Mouse, delta_time: f32) -> bool {
        if mouse.is_moved() {
            let (mut xoffset, mut yoffset) = mouse.get_delta_movement();
            xoffset *= (self.sensitivity * delta_time) as f64;
            yoffset *= (self.sensitivity * delta_time) as f64;

            self.yaw += xoffset as f32;
            self.pitch += yoffset as f32;

            self.pitch = f32::min(self.pitch, 89.0);
            self.pitch = f32::max(self.pitch, -89.0);

            self.front = glm::vec3(0.0, 0.0, 0.0);
            self.front.x =
                f32::cos(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch));
            self.front.y = f32::sin(f32::to_radians(self.pitch));
            self.front.z =
                f32::sin(f32::to_radians(self.yaw)) * f32::cos(f32::to_radians(self.pitch));
            self.front = self.front.normalize();
            true
        } else {
            false
        }
    }

    fn handle_keyboard_input(
        &mut self,
        keyboard: &engine::input_system::Keyboard,
        delta_time: f32,
    ) -> bool {
        let mut is_updated = false;
        if keyboard.get_key_press_state(glfw::Key::W) == glfw::Action::Press {
            self.position += self.step_size * self.front * delta_time;
            is_updated = true;
        }
        if keyboard.get_key_press_state(glfw::Key::S) == glfw::Action::Press {
            self.position -= self.step_size * self.front * delta_time;
            is_updated = true;
        }
        if keyboard.get_key_press_state(glfw::Key::A) == glfw::Action::Press {
            self.position += self.get_left_vector() * self.step_size * delta_time;
            is_updated = true;
        }
        if keyboard.get_key_press_state(glfw::Key::D) == glfw::Action::Press {
            self.position += self.get_right_vector() * self.step_size * delta_time;
            is_updated = true;
        }
        if keyboard.get_key_press_state(glfw::Key::Space) == glfw::Action::Press {
            self.position.y += self.step_size * delta_time;
            is_updated = true;
        }
        if keyboard.get_key_press_state(glfw::Key::LeftShift) == glfw::Action::Press {
            self.position.y -= self.step_size * delta_time;
            is_updated = true;
        }
        is_updated
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(
            glm::vec3(0.0, 2000.0, 0.0),
            glm::vec3(0.0, 0.0, 1.0),
            glm::vec3(0.0, 1.0, 0.0),
            1.0,
        )
    }
}
