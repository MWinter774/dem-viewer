use crate::engine::camera_view;

pub struct CameraViewApp {
    opencv_window: camera_view::OpenCVWindow,
}

impl CameraViewApp {
    pub fn new() -> Self {
        let opencv_window = camera_view::OpenCVWindow::default();
        Self { opencv_window }
    }

    pub fn capture_clicked_points(&self, pixels: Vec<u8>, window_height: usize) {
        self.opencv_window.capture_points(pixels, window_height);
    }
}
