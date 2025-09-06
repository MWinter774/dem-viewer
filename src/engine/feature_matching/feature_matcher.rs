use crate::engine::feature_matching;
use nalgebra_glm as glm;
use opencv::{
    core::{self, no_array},
    features2d,
    prelude::*,
};

pub struct FeatureMatcher {
    views: feature_matching::Views,
    _window_width: usize,
    window_height: usize,
    detector: core::Ptr<features2d::ORB>,
}

impl FeatureMatcher {
    pub fn new(window_width: usize, window_height: usize) -> FeatureMatcher {
        FeatureMatcher {
            views: feature_matching::Views::new(),
            _window_width: window_width,
            window_height,
            detector: features2d::ORB::create(
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
            .unwrap(),
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

    // Returns either:
    // 1. The view that the feature matching algorithm found the closest to pixel_data
    // 2. Error if not enough view were picked
    pub fn feature_match(
        &mut self,
        pixel_data: &mut Vec<u8>,
    ) -> Result<&feature_matching::View, &str> {
        if self.views.get_num_views() == 0 {
            return Err("You must pick at least 1 view in order to perform feature matching!");
        }
        let (img, descriptors, keypoints) = self.detect_descriptors(pixel_data);
        let mut max_matches_len = 0;
        let mut max_matches = core::Vector::new();
        let mut matching_view = &self.views.get_views()[0];
        let matcher = features2d::BFMatcher::new(opencv::core::NORM_HAMMING, true).unwrap();
        for view in self.views.get_views() {
            let mut matches = core::Vector::new();
            matcher
                .train_match(
                    &descriptors,
                    view.get_descriptors(),
                    &mut matches,
                    &no_array(),
                )
                .unwrap();

            if matches.len() > max_matches_len {
                max_matches_len = matches.len();
                matching_view = view;
                max_matches = matches;
            }
        }
        Self::draw_matches(
            &img,
            matching_view.get_img(),
            &keypoints,
            matching_view.get_keypoints(),
            &max_matches,
        );
        Ok(matching_view)
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

    fn detect_descriptors(
        &mut self,
        pixel_data: &mut Vec<u8>,
    ) -> (core::Mat, core::Mat, core::Vector<core::KeyPoint>) {
        let mut keypoints = core::Vector::new();
        let mut descriptors = core::Mat::default();
        let img = self.pixels_to_image(pixel_data);
        self.detector
            .detect_and_compute(
                &img,
                &Mat::default(),
                &mut keypoints,
                &mut descriptors,
                false,
            )
            .unwrap();
        (img, descriptors, keypoints)
    }

    fn draw_matches(
        img1: &core::Mat,
        img2: &core::Mat,
        kps1: &core::Vector<core::KeyPoint>,
        kps2: &core::Vector<core::KeyPoint>,
        matches: &core::Vector<core::DMatch>,
    ) {
        let mut out = core::Mat::default();
        let mask: core::Vector<i8> = core::Vector::new();
        let mut good: core::Vector<core::DMatch> = core::Vector::new();
        for m in matches {
            if m.distance < 40.0 {
                good.push(m);
            }
        }
        features2d::draw_matches(
            &img1,
            &kps1,
            &img2,
            &kps2,
            &good,
            &mut out,
            core::Scalar::all(-1.0),
            core::Scalar::all(-1.0),
            &mask,
            features2d::DrawMatchesFlags::DEFAULT,
        )
        .unwrap();
        opencv::highgui::imshow("BFMatcher Result", &out).unwrap();
        opencv::highgui::wait_key(0).unwrap();
    }
}
