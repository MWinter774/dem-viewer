extern crate nalgebra_glm as glm;

pub struct Projection {
    projection_matrix: glm::Mat4,
    fovy: f32,
    window_width: usize,
    window_height: usize,
    aspect_ratio: f32,
}

impl Projection {
    pub fn new(fovy: f32, window_width: usize, window_height: usize) -> Self {
        let aspect_ratio = (window_width as f32) / (window_height as f32);
        Self {
            projection_matrix: glm::perspective(aspect_ratio, fovy.to_radians(), 0.9, 100000.0),
            fovy,
            window_width,
            window_height,
            aspect_ratio,
        }
    }

    pub fn get_matrix(&self) -> &glm::Mat4 {
        &self.projection_matrix
    }

    pub fn get_fovy(&self) -> f32 {
        self.fovy
    }
    pub fn get_window_width(&self) -> usize {
        self.window_width
    }
    pub fn get_window_height(&self) -> usize {
        self.window_height
    }
    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}

impl Default for Projection {
    fn default() -> Self {
        Projection::new(35.0, 800, 600)
    }
}
