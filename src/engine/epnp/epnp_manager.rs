use nalgebra_glm as glm;

use crate::engine::{epnp, transformations};

use opencv::{calib3d, core};

pub struct EPnPManager {
    picture_points: Vec<epnp::EPnPPicturePoint>,
    real_world_points: Vec<epnp::EPnPRealWorldPoint>,
}

impl EPnPManager {
    pub fn new() -> Self {
        Self {
            picture_points: Vec::new(),
            real_world_points: Vec::new(),
        }
    }

    pub fn get_image_points(&self) -> &Vec<epnp::EPnPPicturePoint> {
        &self.picture_points
    }
    pub fn set_image_points(&mut self, image_points: Vec<epnp::EPnPPicturePoint>) {
        self.picture_points = image_points;
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
        self.real_world_points.len() == self.picture_points.len()
    }

    pub fn compute_camera_pose(
        &self,
        projection_matrix: &transformations::Projection,
    ) -> glm::Vec3 {
        let object_points = Self::get_object_points_from_real_world_points(&self.real_world_points);
        let image_points = Self::get_image_points_from_picture_points(&self.picture_points);
        let camera_matrix = Self::get_camera_matrix_from_projection_matrix(projection_matrix);
        // calib3d::solve_pnp(object_points, image_points, camera_matrix, dist_coeffs, rvec, tvec, use_extrinsic_guess, flags)
        glm::vec3(0.0, 0.0, 0.0)
    }

    fn get_object_points_from_real_world_points(
        real_world_points: &Vec<epnp::EPnPRealWorldPoint>,
    ) -> Vec<core::Point3f> {
        let mut object_points = Vec::<core::Point3f>::new();
        for real_world_point in real_world_points {
            object_points.push(core::Point3f::new(
                real_world_point.point.x,
                real_world_point.point.y,
                real_world_point.point.z,
            ));
        }
        object_points
    }

    fn get_image_points_from_picture_points(
        picture_points: &Vec<epnp::EPnPPicturePoint>,
    ) -> Vec<core::Point2f> {
        let mut image_points = Vec::<core::Point2f>::new();
        for picture_point in picture_points {
            image_points.push(core::Point2f::new(
                picture_point.point.x as f32,
                picture_point.point.y as f32,
            ));
        }
        image_points
    }

    fn get_camera_matrix_from_projection_matrix(
        projection_matrix: &transformations::Projection,
    ) -> core::Mat {
        let fovy = projection_matrix.get_fovy().to_radians() as f64;
        let fy = ((projection_matrix.get_window_height() as f64) / 2.0) / f64::tan(fovy / 2.0);
        let fx = fy * (projection_matrix.get_aspect_ratio() as f64);
        let cx = (projection_matrix.get_window_width() as f64) / 2.0;
        let cy = (projection_matrix.get_window_height() as f64) / 2.0;
        let camera_matrix =
            core::Mat::from_slice_2d(&[[fx, 0.0, cx], [0.0, fy, cy], [0.0, 0.0, 1.0]]).unwrap();
        camera_matrix
    }
}
