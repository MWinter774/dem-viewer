extern crate nalgebra_glm as glm;

pub struct View {
    view_matrix: glm::Mat4,
}

impl View {
    pub fn new(
        camera_position: &glm::Vec3,
        camera_target: &glm::Vec3,
        up_vector: &glm::Vec3,
    ) -> Self {
        Self {
            view_matrix: glm::look_at(camera_position, camera_target, up_vector),
        }
    }

    pub fn get_matrix(&self) -> &glm::Mat4 {
        &self.view_matrix
    }
}

impl Default for View {
    fn default() -> Self {
        View::new(
            &glm::vec3(0.0, 0.0, 0.0),
            &glm::vec3(0.0, 0.0, 1.0),
            &glm::vec3(0.0, 1.0, 0.0),
        )
    }
}
