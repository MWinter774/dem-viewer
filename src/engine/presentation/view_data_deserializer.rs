use crate::engine;
use nalgebra_glm as glm;
use opencv::core;
use serde_json::Value;
use std::{fs, io::BufReader};

pub struct ViewDataDeserializer {}

impl ViewDataDeserializer {
    pub fn deserial_view_data_json_file(
        file_name: &str,
    ) -> (
        Vec<u8>,
        Vec<engine::epnp::EPnPPicturePoint>,
        Vec<engine::epnp::EPnPRealWorldPoint>,
        glm::Vec3,
    ) {
        let f = fs::File::open(file_name).unwrap();
        let rdr = BufReader::new(f);
        let v: Value = serde_json::from_reader(rdr).unwrap();
        let pixel_data = Self::desearialize_pixel_data(&v["pixel_data"]);
        let picked_points = Self::desearialize_picked_points(&v["picked_points"]);
        let real_world_points = Self::desearialize_real_world_points_pose(&v["real_world_points"]);
        let real_camera_pose = Self::desearialize_real_camera_pose(&v["real_camera_pose"]);
        (
            pixel_data,
            picked_points,
            real_world_points,
            real_camera_pose,
        )
    }

    fn desearialize_pixel_data(v: &Value) -> Vec<u8> {
        let mut pixel_data: Vec<u8> = Vec::new();
        for v in v.as_array().unwrap() {
            pixel_data.push(v.as_i64().unwrap() as u8);
        }
        pixel_data
    }

    fn desearialize_picked_points(v: &Value) -> Vec<engine::epnp::EPnPPicturePoint> {
        let mut picked_points: Vec<engine::epnp::EPnPPicturePoint> = Vec::new();
        for v in v.as_array().unwrap() {
            let id = v["id"].as_i64().unwrap() as u8;
            let point = &v["point"];
            let point = core::Point::new(
                point[0].as_i64().unwrap() as i32,
                point[1].as_i64().unwrap() as i32,
            );
            let opencv_color = &v["opencv_color"];
            let opencv_color = glm::DVec3::new(
                opencv_color[0].as_f64().unwrap(),
                opencv_color[1].as_f64().unwrap(),
                opencv_color[2].as_f64().unwrap(),
            );
            let opengl_color = &v["opengl_color"];
            let opengl_color = glm::Vec3::new(
                opengl_color[0].as_f64().unwrap() as f32,
                opengl_color[1].as_f64().unwrap() as f32,
                opengl_color[2].as_f64().unwrap() as f32,
            );
            let epnp_picture_point = engine::epnp::EPnPPicturePoint {
                point,
                id,
                opencv_color,
                opengl_color,
            };
            picked_points.push(epnp_picture_point);
        }
        picked_points
    }

    fn desearialize_real_camera_pose(v: &Value) -> glm::Vec3 {
        glm::vec3(
            v[0].as_f64().unwrap() as f32,
            v[1].as_f64().unwrap() as f32,
            v[2].as_f64().unwrap() as f32,
        )
    }

    fn desearialize_real_world_points_pose(v: &Value) -> Vec<engine::epnp::EPnPRealWorldPoint> {
        let mut real_world_points: Vec<engine::epnp::EPnPRealWorldPoint> = Vec::new();
        for v in v.as_array().unwrap() {
            let primitive_id = v["primitive_id"].as_i64().unwrap() as u32;
            let point = &v["point"];
            let point = glm::Vec3::new(
                point[0].as_f64().unwrap() as f32,
                point[1].as_f64().unwrap() as f32,
                point[2].as_f64().unwrap() as f32,
            );
            let color = &v["color"];
            let color = glm::Vec3::new(
                color[0].as_f64().unwrap() as f32,
                color[1].as_f64().unwrap() as f32,
                color[2].as_f64().unwrap() as f32,
            );
            let epnp_real_world_point = engine::epnp::EPnPRealWorldPoint {
                point,
                color,
                primitive_id,
            };
            real_world_points.push(epnp_real_world_point);
        }
        real_world_points
    }
}
