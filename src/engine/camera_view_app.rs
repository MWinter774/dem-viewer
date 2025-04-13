use crate::engine;

pub struct CameraViewApp {
    opencv_window: engine::OpenCVWindow,
}

impl CameraViewApp {
    pub fn new() -> Self {
        let opencv_window = engine::OpenCVWindow::default();
        Self { opencv_window }
    }

    pub fn display_pixels(&self, pixels: Vec<u8>, window_height: usize) {
        self.opencv_window.display_pixels(pixels, window_height);
    }
}
