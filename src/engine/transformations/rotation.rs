extern crate nalgebra_glm as glm;

pub struct Rotation {
    rotation_matrix: glm::Mat4,
}

impl Rotation {
    pub fn new(angle_in_radians: f32, axis: &glm::Vec3) -> Self {
        Self {
            rotation_matrix: glm::rotation(angle_in_radians, axis),
        }
    }

    pub fn get_matrix(&self) -> &glm::Mat4 {
        &self.rotation_matrix
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::new(0.0, &glm::vec3(0.0, 0.0, 0.0))
    }
}
