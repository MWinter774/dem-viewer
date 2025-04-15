use nalgebra_glm as glm;

#[derive(Debug, Copy, Clone)]
pub struct EPnPRealWorldPoint {
    pub point: glm::Vec3,
    pub color: glm::Vec3,
    pub primitive_id: u32,
}
