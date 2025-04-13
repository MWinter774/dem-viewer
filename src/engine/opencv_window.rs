use std::str::FromStr;

use opencv::{highgui, prelude::*};

pub struct OpenCVWindow {
    window_title: String,
}

impl OpenCVWindow {
    pub fn new(window_title: String) -> Self {
        Self { window_title }
    }

    pub fn capture_points(&self, mut pixels: Vec<u8>, window_height: usize) {
        // Convert RGB to BGR (OpenCV expects BGR)
        for chunk in pixels.chunks_exact_mut(3) {
            chunk.swap(0, 2); // swap R and B
        }
        let binding = Mat::from_slice(pixels.as_slice()).unwrap();
        let mat = binding.reshape(3, window_height as i32).unwrap();

        let mut flipped = Mat::default();
        opencv::core::flip(&mat, &mut flipped, 0).unwrap();

        highgui::imshow(&self.window_title, &flipped).unwrap();
        highgui::set_mouse_callback(
            &self.window_title,
            Some(Box::new(move |event, x, y, _flags| {
                Self::mouse_callback(event, x, y, _flags);
            })),
        )
        .unwrap();
        highgui::wait_key(0).unwrap();
    }

    fn mouse_callback(event: i32, x: i32, y: i32, _flags: i32) {
        if event == highgui::EVENT_LBUTTONDOWN {}
    }
}

impl Default for OpenCVWindow {
    fn default() -> Self {
        let window_title = String::from_str("Camera view").unwrap();
        Self::new(window_title)
    }
}
