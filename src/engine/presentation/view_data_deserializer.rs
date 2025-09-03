use crate::engine;
use nalgebra_glm as glm;
use serde_json::{Result, Value};
use std::{fs, io::BufReader};

pub struct ViewDataDeserializer {}

impl ViewDataDeserializer {
    pub fn deserial_view_data_json_file(
        file_name: &str,
    ) -> (Vec<u8>, Vec<engine::epnp::EPnPPicturePoint>, glm::Vec3) {
        let f = fs::File::open(file_name).unwrap();
        let rdr = BufReader::new(f);
        let v: Value = serde_json::from_reader(rdr).unwrap();
        let pixel_data = Self::desearialize_pixel_data(&v["pixel_data"]);
        let picked_points = Self::desearialize_picked_points(&v["picked_points"]);
        let real_camera_pose = Self::desearialize_real_camera_pose(&v["real_camera_pose"]);
        (pixel_data, picked_points, real_camera_pose)
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
        for v in v.as_array().unwrap() {}
        picked_points
    }

    fn desearialize_real_camera_pose(v: &Value) -> glm::Vec3 {
        glm::vec3(
            v[0].as_f64().unwrap() as f32,
            v[1].as_f64().unwrap() as f32,
            v[2].as_f64().unwrap() as f32,
        )
    }
}
