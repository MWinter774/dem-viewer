use crate::engine;
use rand::{Rng, distributions::Alphanumeric};
use std::fs;

use nalgebra_glm as glm;

pub struct SceneInitializerForPresentation {}

impl SceneInitializerForPresentation {
    pub fn initialize(scene: &mut engine::Scene) {
        for file in fs::read_dir("./presentations").unwrap() {
            let (
                deserialized_pixel_data,
                deserialized_picked_points,
                deserialized_real_world_points,
                deserialized_real_camera_pose,
            ) = engine::presentation::ViewDataDeserializer::deserial_view_data_json_file(
                file.unwrap().path().to_str().unwrap(),
            );
            scene.add_defined_view_to_feature_matching(
                &deserialized_pixel_data,
                &deserialized_picked_points,
                &deserialized_real_world_points,
                &deserialized_real_camera_pose,
            );
        }
    }

    // Save view data to a file in ./presentation/
    pub fn save_to_file(
        pixel_data: &Vec<u8>,
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
        real_world_points: &Vec<engine::epnp::EPnPRealWorldPoint>,
        real_camera_pose: &glm::Vec3,
    ) {
        let pixel_data_serialized =
            engine::presentation::ViewDataSerializer::serialize_pixel_data(&pixel_data);
        let picked_points_serialized =
            engine::presentation::ViewDataSerializer::serialize_epnp_picture_points(&picked_points);
        let real_world_points_serialized =
            engine::presentation::ViewDataSerializer::serialize_epnp_real_world_points(
                &real_world_points,
            );
        let real_camera_pose_serialized =
            engine::presentation::ViewDataSerializer::serialize_glm_vec3(&real_camera_pose);
        let view_data_json = format!(
            "{{\"pixel_data\":{},\"picked_points\":{},\"real_camera_pose\":{},\"real_world_points\":{}}}",
            pixel_data_serialized,
            picked_points_serialized,
            real_camera_pose_serialized,
            real_world_points_serialized
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
    }
}
