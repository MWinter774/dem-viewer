use std::str::FromStr;

use crate::engine::camera_view;

pub struct CameraViewApp {
    camera_view_window: camera_view::CameraViewWindow,
}

impl CameraViewApp {
    pub fn new(window_width: usize, window_height: usize) -> Self {
        let camera_view_window = camera_view::CameraViewWindow::new(
            String::from_str("Camera View").unwrap(),
            window_width,
            window_height,
        );
        Self { camera_view_window }
    }

    pub fn capture_clicked_points(&self, pixels: Vec<u8>) {
        let _selected_points = self.camera_view_window.capture_points(pixels);        
    }
}
