use nalgebra_glm as glm;

pub struct View {
    pixel_data: Vec<u8>,
    real_camera_position: glm::Vec3,
    estimated_camera_position: glm::Vec3,
}

impl View {
    pub fn new(pixel_data: &Vec<u8>, real_camera_position: &glm::Vec3) -> View {
        View {
            pixel_data: pixel_data.clone(),
            real_camera_position: real_camera_position.clone(),
            estimated_camera_position: glm::vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn set_estimated_camera_position(&mut self, estimated_camera_position: &glm::Vec3) {
        self.estimated_camera_position = estimated_camera_position.clone();
    }
}
