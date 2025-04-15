use crate::engine::epnp;

pub struct EPnPManager {
    image_points: Vec<epnp::EPnPPicturePoint>,
    real_world_points: Vec<epnp::EPnPRealWorldPoint>,
}

impl EPnPManager {
    pub fn new() -> Self {
        Self {
            image_points: Vec::new(),
            real_world_points: Vec::new(),
        }
    }

    pub fn get_image_points(&self) -> &Vec<epnp::EPnPPicturePoint> {
        &self.image_points
    }
    pub fn set_image_points(&mut self, image_points: Vec<epnp::EPnPPicturePoint>) {
        self.image_points = image_points;
    }
    pub fn get_real_world_points(&self) -> &Vec<epnp::EPnPRealWorldPoint> {
        &self.real_world_points
    }
    pub fn get_image_points_mut(&mut self) -> &mut Vec<epnp::EPnPRealWorldPoint> {
        &mut self.real_world_points
    }
    pub fn set_real_world_points(&mut self, real_world_points: Vec<epnp::EPnPRealWorldPoint>) {
        self.real_world_points = real_world_points;
    }

    // Returns true if there is equal amount of real world points to image points
    pub fn add_real_world_points(&mut self, real_world_point: epnp::EPnPRealWorldPoint) -> bool {
        self.real_world_points.push(real_world_point);
        self.real_world_points.len() == self.image_points.len()
    }
}
