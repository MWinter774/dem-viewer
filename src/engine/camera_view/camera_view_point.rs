use nalgebra_glm as glm;

use opencv::core;

pub struct CameraViewPoint {
    pub point: core::Point,
    pub id: u8,
    pub color: glm::DVec3,
}
