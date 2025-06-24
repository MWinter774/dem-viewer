use crate::engine::feature_matching;
use nalgebra_glm as glm;

pub struct FeatureMatcher {
    views: feature_matching::Views,
}

impl FeatureMatcher {
    pub fn new() -> FeatureMatcher {
        FeatureMatcher {
            views: feature_matching::Views::new(),
        }
    }

    pub fn add_view(&mut self, pixel_data: &Vec<u8>, real_camera_position: &glm::Vec3) {
        self.views.new_view(pixel_data, real_camera_position);
    }

    pub fn update_estimated_camera_position(&mut self, estimated_camera_position: &glm::Vec3) {
        self.views
            .update_estimated_camera_position(estimated_camera_position);
    }

    pub fn get_num_views(&self) -> usize {
        self.views.get_num_views()
    }

    pub fn feature_match(&self, pixel_data: &Vec<u8>) {}
}
