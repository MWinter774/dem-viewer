use crate::engine;
use rand::{Rng, distributions::Alphanumeric};
use std::fs;

use nalgebra_glm as glm;

pub struct SceneInitializerForPresentation {}

impl SceneInitializerForPresentation {
    pub fn initialize(scene: &mut engine::Scene) {
        for file in fs::read_dir("./presentations").unwrap() {
            engine::presentation::ViewDataDeserializer::deserial_view_data_json_file(
                file.unwrap().path().to_str().unwrap(),
            );
        }
    }

    // Save view data to a file in ./presentation/
    pub fn save_to_file(
        pixel_data: &Vec<u8>,
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
        real_camera_pose: &glm::Vec3,
    ) {
        let pixel_data_serialized =
            engine::presentation::ViewDataSerializer::serialize_pixel_data(&pixel_data);
        let picked_points_serialized =
            engine::presentation::ViewDataSerializer::serialize_epnp_picture_points(&picked_points);
        let real_camera_pose_serialized =
            engine::presentation::ViewDataSerializer::serialize_glm_vec3(&real_camera_pose);
        let view_data_json = format!(
            "{{\"pixel_data\":{},\"picked_points\":{},\"real_camera_pose\":{}}}",
            pixel_data_serialized, picked_points_serialized, real_camera_pose_serialized
        );

        let random_filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let full_filename = format!("./presentations/{}.json", random_filename);
        fs::create_dir_all("./presentations").expect("Error creating ./presentations/!");
        fs::write(full_filename.clone(), view_data_json)
            .expect(&format!("Error writing to {}!", full_filename));

        let (desiralized_pixel_data, desiralized_picked_points, desiralized_real_camera_pose) =
            engine::presentation::ViewDataDeserializer::deserial_view_data_json_file(
                &full_filename,
            );

        println!("Check desiralization...");
        for i in 0..pixel_data.len() {
            if pixel_data[i] != desiralized_pixel_data[i] {
                panic!("Bug in desrialization!");
            }
        }
        println!("Done check serialization! Everything is working!");
    }
}
