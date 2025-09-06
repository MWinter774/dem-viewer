use crate::engine::{self, feature_matching};
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

    pub fn add_view(
        &mut self,
        pixel_data: &Vec<u8>,
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
        real_world_points: &Vec<engine::epnp::EPnPRealWorldPoint>,
        real_camera_position: &glm::Vec3,
    ) {
        self.views.new_view(
            pixel_data,
            picked_points,
            real_world_points,
            real_camera_position,
            self.window_height,
        );
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
    pub fn feature_match(&mut self, pixel_data: &Vec<u8>) -> Result<&feature_matching::View, &str> {
        if self.views.get_num_views() == 0 {
            return Err("You must pick at least 1 view in order to perform feature matching!");
        }
        let (img, descriptors, keypoints) = self.detect_descriptors(&mut pixel_data.clone());
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
        Ok(matching_view)
    }

    pub fn estimate_picked_points(
        &mut self,
        pixel_data: &Vec<u8>,
    ) -> Result<
        (
            Vec<engine::epnp::EPnPPicturePoint>,
            &feature_matching::view::View,
        ),
        &str,
    > {
        let qry_img_bgr = self.pixels_to_image(&mut pixel_data.clone());
        // Best matching view to pixel_data
        let matching_view = self.feature_match(pixel_data)?;
        let ref_landmarks =
            Self::picked_points_to_point2f_vector(matching_view.get_picked_points());

        let mut ref_gray = Mat::default();
        let mut qry_gray = Mat::default();
        let ref_img_bgr = matching_view.get_img();
        opencv::imgproc::cvt_color(
            &ref_img_bgr,
            &mut ref_gray,
            opencv::imgproc::COLOR_BGR2GRAY,
            0,
            core::AlgorithmHint::ALGO_HINT_DEFAULT,
        )
        .unwrap();
        opencv::imgproc::cvt_color(
            &qry_img_bgr,
            &mut qry_gray,
            opencv::imgproc::COLOR_BGR2GRAY,
            0,
            core::AlgorithmHint::ALGO_HINT_DEFAULT,
        )
        .unwrap();

        let mut sift = features2d::SIFT::create(0, 3, 0.04, 10.0, 1.6, false).unwrap();
        let (mut kps_ref, mut desc_ref) = (core::Vector::new(), Mat::default());
        let (mut kps_qry, mut desc_qry) = (core::Vector::new(), Mat::default());
        sift.detect_and_compute(
            &ref_gray,
            &core::no_array(),
            &mut kps_ref,
            &mut desc_ref,
            false,
        )
        .unwrap();
        sift.detect_and_compute(
            &qry_gray,
            &core::no_array(),
            &mut kps_qry,
            &mut desc_qry,
            false,
        )
        .unwrap();

        if desc_ref.empty() || desc_qry.empty() {
            return Err("Not enough data for feature matching");
        }

        let matcher = features2d::DescriptorMatcher::create("BruteForce").unwrap();
        let mut knn_matches = core::Vector::new();
        matcher
            .knn_train_match(
                &desc_ref,
                &desc_qry,
                &mut knn_matches,
                2,
                &core::no_array(),
                false,
            )
            .unwrap();

        let mut good_matches: core::Vector<core::DMatch> = core::Vector::new();
        for pair in knn_matches.iter() {
            if pair.len() >= 2 {
                let m = pair.get(0).unwrap();
                let n = pair.get(1).unwrap();
                if m.distance < 0.75 * n.distance {
                    good_matches.push(m);
                }
            }
        }
        if good_matches.len() < 4 {
            return Err("Not enough data for feature matching");
        }

        let mut pts_ref: core::Vector<core::Point2f> = core::Vector::new();
        let mut pts_qry: core::Vector<core::Point2f> = core::Vector::new();
        for m in good_matches {
            let pr = kps_ref.get(m.query_idx as usize).unwrap().pt();
            let pq = kps_qry.get(m.train_idx as usize).unwrap().pt();
            pts_ref.push(core::Point2f::new(pr.x, pr.y));
            pts_qry.push(core::Point2f::new(pq.x, pq.y));
        }

        let h = opencv::calib3d::find_homography(
            &pts_ref,
            &pts_qry,
            &mut opencv::core::no_array(),
            opencv::calib3d::RANSAC,
            3.0,
        )
        .unwrap();
        if h.empty() {
            return Err("Not enough data for feature matching");
        }

        let mut ref_pts_mat = Mat::zeros(ref_landmarks.len() as i32, 1, opencv::core::CV_32FC2)
            .unwrap()
            .to_mat()
            .unwrap();
        for (i, p) in ref_landmarks.iter().enumerate() {
            *ref_pts_mat
                .at_2d_mut::<opencv::core::Point2f>(i as i32, 0)
                .unwrap() = p.clone();
        }
        let mut pred_pts_mat = Mat::default();
        opencv::core::perspective_transform(&ref_pts_mat, &mut pred_pts_mat, &h).unwrap();

        let mut prev_pts: core::Vector<core::Point2f> = core::Vector::new();
        let mut next_pts: core::Vector<core::Point2f> = core::Vector::new();
        let mut status: core::Vector<u8> = core::Vector::new();
        let mut err: core::Vector<f32> = core::Vector::new();

        for i in 0..pred_pts_mat.rows() {
            let p = *pred_pts_mat.at_2d::<opencv::core::Point2f>(i, 0).unwrap();
            prev_pts.push(p); // initial guess in query
        }

        opencv::video::calc_optical_flow_pyr_lk(
            &qry_gray,
            &qry_gray, // same image to locally refine
            &prev_pts,
            &mut next_pts,
            &mut status,
            &mut err,
            opencv::core::Size::new(21, 21),
            3,
            opencv::core::TermCriteria::new(
                opencv::core::TermCriteria_Type::COUNT as i32
                    | opencv::core::TermCriteria_Type::EPS as i32,
                30,
                0.01,
            )
            .unwrap(),
            0,
            1e-4,
        )
        .unwrap();

        let refined: Vec<core::Point2f> = (0..next_pts.len())
            .map(|i| next_pts.get(i).unwrap())
            .collect();

        let mut estimated_picked_points = Vec::new();
        for i in 0..refined.len() {
            estimated_picked_points.push(engine::epnp::EPnPPicturePoint {
                point: core::Point::new(refined[i].x as i32, refined[i].y as i32),
                id: i as u8,
                opencv_color: glm::vec3(0.0, 0.0, 0.0),
                opengl_color: glm::vec3(0.0, 0.0, 0.0),
            });
        }

        Ok((estimated_picked_points, matching_view))
    }

    fn picked_points_to_point2f_vector(
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
    ) -> core::Vector<core::Point2f> {
        let mut v = core::Vector::new();
        for p in picked_points {
            v.push(core::Point2f::new(p.point.x as f32, p.point.y as f32));
        }
        v
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
