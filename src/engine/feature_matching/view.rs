use nalgebra_glm as glm;
use opencv::{core, features2d, prelude::*};

pub struct View {
    pixel_data: Vec<u8>,
    real_camera_position: glm::Vec3,
    estimated_camera_position: glm::Vec3,
    descriptors: core::Mat,
    img: core::Mat,
    keypoints: core::Vector<core::KeyPoint>,
}

impl View {
    pub fn new(
        pixel_data: &Vec<u8>,
        real_camera_position: &glm::Vec3,
        window_height: usize,
    ) -> View {
        let mut pixel_data = pixel_data.clone();
        let (img, descriptors, keypoints) =
            View::detect_descriptors(&mut pixel_data, window_height);
        View {
            pixel_data,
            real_camera_position: real_camera_position.clone(),
            estimated_camera_position: glm::vec3(0.0, 0.0, 0.0),
            descriptors,
            img,
            keypoints,
        }
    }

    pub fn set_estimated_camera_position(&mut self, estimated_camera_position: &glm::Vec3) {
        self.estimated_camera_position = estimated_camera_position.clone();
    }

    pub fn get_descriptors(&self) -> &core::Mat {
        &self.descriptors
    }

    pub fn get_real_camera_position(&self) -> &glm::Vec3 {
        &self.real_camera_position
    }

    pub fn get_estimated_camera_position(&self) -> &glm::Vec3 {
        &self.estimated_camera_position
    }

    pub fn get_pixel_data(&self) -> &Vec<u8> {
        &self.pixel_data
    }

    pub fn get_img(&self) -> &core::Mat {
        &self.img
    }

    pub fn get_keypoints(&self) -> &core::Vector<core::KeyPoint> {
        &self.keypoints
    }

    fn pixels_to_image(pixels: &mut Vec<u8>, window_height: usize) -> Mat {
        // Convert RGB to BGR (OpenCV expects BGR)
        for chunk in pixels.chunks_exact_mut(3) {
            chunk.swap(0, 2); // swap R and B
        }
        let binding = Mat::from_slice(pixels.as_slice()).unwrap();
        let mat = binding.reshape(3, window_height as i32).unwrap();

        let mut flipped = Mat::default();
        opencv::core::flip(&mat, &mut flipped, 0).unwrap();

        flipped
    }

    fn detect_descriptors(
        pixel_data: &mut Vec<u8>,
        window_height: usize,
    ) -> (core::Mat, core::Mat, core::Vector<core::KeyPoint>) {
        let mut keypoints = core::Vector::new();
        let mut descriptors = core::Mat::default();
        let img = View::pixels_to_image(pixel_data, window_height);
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
        (img, descriptors, keypoints)
    }
}
