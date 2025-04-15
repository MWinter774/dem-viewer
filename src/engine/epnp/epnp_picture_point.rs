use nalgebra_glm as glm;

use opencv::core;

#[derive(Debug, Copy, Clone)]
pub struct EPnPPicturePoint {
    pub point: core::Point,
    pub id: u8,
    pub opencv_color: glm::DVec3,
    pub opengl_color: glm::Vec3,
}
