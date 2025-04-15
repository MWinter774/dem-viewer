extern crate user32;

use nalgebra_glm as glm;

use opencv::{self, core, highgui, prelude::*};
use std::sync;

use crate::engine::camera_view;

const ESCAPE_KEY: i32 = 27;
const MIN_NUM_OF_POINTS: usize = 4;

pub struct CameraViewWindow {
    window_title: String,
    points: sync::Arc<sync::Mutex<Vec<camera_view::CameraViewPoint>>>,
    window_width: usize,
    window_height: usize,
}

impl CameraViewWindow {
    pub fn new(window_title: String, window_width: usize, window_height: usize) -> Self {
        Self {
            window_title,
            points: sync::Arc::new(sync::Mutex::new(Vec::new())),
            window_width,
            window_height,
        }
    }

    pub fn capture_points(&self, pixels: Vec<u8>) -> Vec<camera_view::CameraViewPoint> {
        self.points.lock().unwrap().clear();

        let mut image = self.pixels_to_image(pixels);
        self.display_info_on_image(&mut image);

        highgui::named_window(&self.window_title, highgui::WINDOW_AUTOSIZE).unwrap();
        self.register_mouse_callback();
        self.bring_to_front();

        loop {
            self.draw_points(&mut image);
            highgui::imshow(&self.window_title, &image).unwrap();

            let key = highgui::wait_key(20).unwrap();
            if key == ESCAPE_KEY {
                // User needs to select at least MIN_NUM_OF_POINTS
                if self.points.lock().unwrap().len() < MIN_NUM_OF_POINTS {
                    self.display_error_on_image(&mut image);
                    continue;
                }
                break;
            }
        }
        let mut selected_points = Vec::<camera_view::CameraViewPoint>::new();
        for p in self.points.lock().unwrap().iter() {
            selected_points.push(*p);
        }
        selected_points
    }

    fn pixels_to_image(&self, mut pixels: Vec<u8>) -> Mat {
        // Convert RGB to BGR (OpenCV expects BGR)
        for chunk in pixels.chunks_exact_mut(3) {
            chunk.swap(0, 2); // swap R and B
        }
        let binding = Mat::from_slice(pixels.as_slice()).unwrap();
        let mat = binding.reshape(3, self.window_height as i32).unwrap();

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

    fn display_error_on_image(&self, img: &mut core::Mat) {
        let img_backup = img.clone();
        opencv::imgproc::put_text(
            img,
            "You must select at least 4 points!",
            core::Point::new(0, ((self.window_height as f32) * (100.0 / 600.0)) as i32),
            opencv::imgproc::FONT_HERSHEY_TRIPLEX,
            1.3,
            opencv::core::Scalar::new(0.0, 0.0, 255.0, 0.0),
            2,
            opencv::imgproc::LINE_4,
            false,
        )
        .unwrap();
        highgui::imshow(&self.window_title, img).unwrap();
        highgui::wait_key(2000).unwrap();
        *img = img_backup;
    }

    fn display_info_on_image(&self, img: &mut core::Mat) {
        opencv::imgproc::put_text(
            img,
            "Press ESC to exit",
            core::Point::new(
                ((self.window_width as f32) * (250.0 / 800.0)) as i32,
                ((self.window_height as f32) * (30.0 / 600.0)) as i32,
            ),
            opencv::imgproc::FONT_HERSHEY_SIMPLEX,
            1.0,
            opencv::core::Scalar::new(255.0, 255.0, 255.0, 0.0),
            1,
            opencv::imgproc::LINE_4,
            false,
        )
        .unwrap();
    }

    fn bring_to_front(&self) {
        let window_name = std::ffi::CString::new(self.window_title.as_str()).unwrap();
        unsafe {
            let window_handle = user32::FindWindowA(std::ptr::null(), window_name.as_ptr());
            if !window_handle.is_null() {
                user32::SetForegroundWindow(window_handle);
            }
        }
    }

    fn mouse_callback(
        event: i32,
        x: i32,
        y: i32,
        _flags: i32,
        points: &mut Vec<camera_view::CameraViewPoint>,
    ) {
        let id = points.len() as u8;
        if event == highgui::EVENT_LBUTTONDOWN {
            let p = camera_view::CameraViewPoint {
                point: core::Point::new(x, y),
                id,
                color: glm::DVec3::new_random() * 255.0,
            };
            points.push(p);
        }
    }
}
