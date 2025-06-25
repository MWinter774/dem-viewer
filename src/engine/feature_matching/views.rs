use crate::engine::feature_matching;
use nalgebra_glm as glm;

pub struct Views {
    views: Vec<feature_matching::View>,
}

impl Views {
    pub fn new() -> Views {
        Views { views: Vec::new() }
    }

    pub fn new_view(
        &mut self,
        pixel_data: &Vec<u8>,
        real_camera_position: &glm::Vec3,
        window_height: usize,
    ) {
        self.views.push(feature_matching::View::new(
            pixel_data,
            real_camera_position,
            window_height,
        ));
    }

    pub fn update_estimated_camera_position(&mut self, estimated_camera_position: &glm::Vec3) {
        self.views
            .last_mut()
            .unwrap()
            .set_estimated_camera_position(estimated_camera_position);
    }

    pub fn get_num_views(&self) -> usize {
        self.views.len()
    }

    pub fn get_views(&self) -> &Vec<feature_matching::View> {
        &self.views
    }
}
