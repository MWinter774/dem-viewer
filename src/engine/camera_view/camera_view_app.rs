use crate::engine::camera_view;

pub struct CameraViewApp {
    camera_view_window: camera_view::CameraViewWindow,
}

impl CameraViewApp {
    pub fn new() -> Self {
        let camera_view_window = camera_view::CameraViewWindow::default();
        Self { camera_view_window }
    }

    pub fn capture_clicked_points(&self, pixels: Vec<u8>, window_height: usize) {
        self.camera_view_window.capture_points(pixels, window_height);
    }
}
