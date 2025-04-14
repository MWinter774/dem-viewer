use nalgebra_glm as glm;

use std::{str::FromStr, sync};

use opencv::{self, core, highgui, prelude::*};

const ESCAPE_KEY: i32 = 27;

pub struct Point {
    pub point: core::Point,
    pub id: u8,
    pub color: glm::DVec3,
}

pub struct OpenCVWindow {
    window_title: String,
    points: sync::Arc<sync::Mutex<Vec<Point>>>,
}

impl OpenCVWindow {
    pub fn new(window_title: String) -> Self {
        Self {
            window_title,
            points: sync::Arc::new(sync::Mutex::new(Vec::new())),
        }
    }

    pub fn capture_points(&self, pixels: Vec<u8>, window_height: usize) {
        let mut image = self.pixels_to_image(pixels, window_height);

        highgui::named_window(&self.window_title, highgui::WINDOW_AUTOSIZE).unwrap();
        self.register_mouse_callback();

        loop {
            self.draw_points(&mut image);
            highgui::imshow(&self.window_title, &image).unwrap();

            let key = highgui::wait_key(20).unwrap();
            if key == ESCAPE_KEY {
                break;
            }
        }

        highgui::destroy_window(&self.window_title).unwrap();
    }

    pub fn get_points(&self) -> &sync::Arc<sync::Mutex<Vec<Point>>> {
        &self.points
    }

    fn pixels_to_image(&self, mut pixels: Vec<u8>, window_height: usize) -> Mat {
        // Convert RGB to BGR (OpenCV expects BGR)
        for chunk in pixels.chunks_exact_mut(3) {
            chunk.swap(0, 2); // swap R and B
        }
        let binding = Mat::from_slice(pixels.as_slice()).unwrap();
        let mat = binding.reshape(3, window_height as i32).unwrap();

        let mut flipped = Mat::default();
        opencv::core::flip(&mat, &mut flipped, 0).unwrap();

        flipped
    }

    fn register_mouse_callback(&self) {
        let points = self.points.clone();
        highgui::set_mouse_callback(
            &self.window_title,
            Some(Box::new(move |event, x, y, _flags| {
                Self::mouse_callback(event, x, y, _flags, &mut points.lock().unwrap());
            })),
        )
        .unwrap();
    }

    fn draw_points(&self, display_img: &mut Mat) {
        for point in self.points.lock().unwrap().iter() {
            opencv::imgproc::circle(
                display_img,
                point.point,
                5,
                opencv::core::Scalar::new(point.color.x, point.color.y, point.color.z, 0.0),
                -1,
                opencv::imgproc::LINE_8,
                0,
            )
            .unwrap();
            opencv::imgproc::put_text_def(
                display_img,
                point.id.to_string().as_str(),
                point.point,
                opencv::imgproc::FONT_HERSHEY_DUPLEX,
                1.0,
                opencv::core::Scalar::new(255.0, 255.0, 255.0, 0.0),
            )
            .unwrap();
        }
    }

    fn mouse_callback(event: i32, x: i32, y: i32, _flags: i32, points: &mut Vec<Point>) {
        let id = points.len() as u8;
        if event == highgui::EVENT_LBUTTONDOWN {
            points.push(Point {
                point: core::Point::new(x, y),
                id,
                color: glm::DVec3::new_random() * 255.0,
            });
        }
    }
}

impl Default for OpenCVWindow {
    fn default() -> Self {
        let window_title = String::from_str("Camera view").unwrap();
        Self::new(window_title)
    }
}
