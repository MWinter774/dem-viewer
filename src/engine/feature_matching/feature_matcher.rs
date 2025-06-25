use crate::engine::feature_matching::{self, view};
use nalgebra_glm as glm;
use opencv::{
    core::{self, no_array},
    features2d, highgui,
    prelude::*,
};

pub struct FeatureMatcher {
    views: feature_matching::Views,
    _window_width: usize,
    window_height: usize,
}

impl FeatureMatcher {
    pub fn new(window_width: usize, window_height: usize) -> FeatureMatcher {
        FeatureMatcher {
            views: feature_matching::Views::new(),
            _window_width: window_width,
            window_height,
        }
    }

    pub fn add_view(&mut self, pixel_data: &Vec<u8>, real_camera_position: &glm::Vec3) {
        self.views
            .new_view(pixel_data, real_camera_position, self.window_height);
    }

    pub fn update_estimated_camera_position(&mut self, estimated_camera_position: &glm::Vec3) {
        self.views
            .update_estimated_camera_position(estimated_camera_position);
    }

    pub fn get_num_views(&self) -> usize {
        self.views.get_num_views()
    }

    pub fn feature_match(&self, pixel_data: &mut Vec<u8>) -> &feature_matching::View {
        let descriptors = self.detect_descriptors(pixel_data);
        let mut max_matches = 0;
        let mut matching_view = &self.views.get_views()[0];
        for view in self.views.get_views() {
            let matcher = features2d::BFMatcher::new(opencv::core::NORM_HAMMING, true).unwrap();
            let mut matches = core::Vector::new();
            matcher
                .train_match(
                    &descriptors,
                    view.get_descriptors(),
                    &mut matches,
                    &no_array(),
                )
                .unwrap();
            if matches.len() > max_matches {
                max_matches = matches.len();
                matching_view = view;
            }
        }
        matching_view
    }

    fn pixels_to_image(&self, pixels: &mut Vec<u8>) -> Mat {
        // Convert RGB to BGR (OpenCV expects BGR)
        for chunk in pixels.chunks_exact_mut(3) {
            chunk.swap(0, 2); // swap R and B
        }
        let binding = Mat::from_slice(pixels.as_slice()).unwrap();
        let mat = binding.reshape(3, self.window_height as i32).unwrap();

        let mut flipped = Mat::default();
        opencv::core::flip(&mat, &mut flipped, 0).unwrap();

        flipped
    }

    fn detect_descriptors(&self, pixel_data: &mut Vec<u8>) -> core::Mat {
        let mut keypoints = core::Vector::new();
        let mut descriptors = core::Mat::default();
        let img = self.pixels_to_image(pixel_data);
        let mut detector = features2d::ORB::create(
            500,
            1.2,
            8,
            31,
            0,
            2,
            features2d::ORB_ScoreType::HARRIS_SCORE,
            31,
            20,
        )
        .unwrap();
        detector
            .detect_and_compute(
                &img,
                &Mat::default(),
                &mut keypoints,
                &mut descriptors,
                false,
            )
            .unwrap();
        descriptors
    }
}
