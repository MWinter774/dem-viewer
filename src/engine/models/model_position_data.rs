use crate::engine::transformations;
use nalgebra_glm as glm;

pub struct ModelPositionData {
    translation_matrix: transformations::Translation,
    rotation_matrix: transformations::Rotation,
    scaling_matrix: transformations::Scaling,
}

impl ModelPositionData {
    pub fn new(
        translation_matrix: transformations::Translation,
        rotation_matrix: transformations::Rotation,
        scaling_matrix: transformations::Scaling,
    ) -> Self {
        Self {
            translation_matrix,
            rotation_matrix,
            scaling_matrix,
        }
    }

    pub fn get_model_matrix(&self) -> glm::Mat4 {
        self.translation_matrix.get_matrix()
            * self.rotation_matrix.get_matrix()
            * self.scaling_matrix.get_matrix()
    }
}

impl Default for ModelPositionData {
    fn default() -> Self {
        let translation_matrix = transformations::Translation::default();
        let rotation_matrix = transformations::Rotation::default();
        let scaling_matrix = transformations::Scaling::default();
        ModelPositionData::new(translation_matrix, rotation_matrix, scaling_matrix)
    }
}
