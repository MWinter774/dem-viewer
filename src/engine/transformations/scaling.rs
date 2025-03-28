extern crate nalgebra_glm as glm;

pub struct Scaling {
    scaling_matrix: glm::Mat4,
}

impl Scaling {
    pub fn new(scale: f32) -> Self {
        Self {
            scaling_matrix: glm::scaling(&glm::vec3(scale, scale, scale)),
        }
    }

    pub fn get_matrix(&self) -> &glm::Mat4 {
        &self.scaling_matrix
    }
}

impl Default for Scaling {
    fn default() -> Self {
        Scaling::new(1.0)
    }
}
