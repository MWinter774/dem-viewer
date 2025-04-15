use nalgebra_glm as glm;

use crate::engine::camera_view;

pub struct EPnPManager {
    image_points: Vec<camera_view::CameraViewPoint>,
    real_world_points: Vec<glm::Vec3>,
}

impl EPnPManager {
    pub fn new() -> Self {
        Self {
            image_points: Vec::new(),
            real_world_points: Vec::new(),
        }
    }

    pub fn get_image_points(&self) -> &Vec<camera_view::CameraViewPoint> {
        &self.image_points
    }
    pub fn set_image_points(&mut self, image_points: Vec<camera_view::CameraViewPoint>) {
        self.image_points = image_points;
    }
    pub fn get_real_world_points(&self) -> &Vec<glm::Vec3> {
        &self.real_world_points
    }
    pub fn set_real_world_points(&mut self, real_world_points: Vec<glm::Vec3>) {
        self.real_world_points = real_world_points;
    }

    // Returns true if there is equal amount of real world points to image points
    pub fn add_real_world_points(&mut self, real_world_point: glm::Vec3) -> bool {
        self.real_world_points.push(real_world_point);
        self.real_world_points.len() == self.image_points.len()
    }
}
