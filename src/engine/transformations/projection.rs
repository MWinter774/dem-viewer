extern crate nalgebra_glm as glm;

pub struct Projection {
    projection_matrix: glm::Mat4,
}

impl Projection {
    pub fn new(fov: f32, aspect_ratio: f32) -> Self {
        Self {
            projection_matrix: glm::perspective(aspect_ratio, fov.to_radians(), 0.9, 100000.0),
        }
    }

    pub fn get_matrix(&self) -> &glm::Mat4 {
        &self.projection_matrix
    }
}

impl Default for Projection {
    fn default() -> Self {
        Projection::new(35.0, 4.0 / 3.0)
    }
}
