extern crate nalgebra_glm as glm;

pub struct Translation {
    translation_matrix: glm::Mat4,
}

impl Translation {
    pub fn new(position: &glm::Vec3) -> Self {
        Self {
            translation_matrix: glm::translation(position),
        }
    }

    pub fn get_matrix(&self) -> &glm::Mat4 {
        &self.translation_matrix
    }

    pub fn set_position(&mut self, new_position: &glm::Vec3) {
        self.translation_matrix = glm::translation(new_position);
    }
}

impl Default for Translation {
    fn default() -> Self {
        Translation::new(&glm::vec3(0.0, 0.0, 0.0))
    }
}
