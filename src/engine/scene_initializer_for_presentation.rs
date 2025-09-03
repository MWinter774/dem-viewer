use crate::engine;
use std::fs;

use nalgebra_glm as glm;

pub struct SceneInitializerForPresentation {}

impl SceneInitializerForPresentation {
    pub fn initialize(scene: &mut engine::Scene) {}

    // Save data to a file in ./presentation/
    pub fn save_to_file(
        pixel_data: &Vec<u8>,
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
        real_camera_pose: &glm::Vec3,
    ) {
        let pixel_data_serialized = serde_json::to_string(&pixel_data).unwrap();
        let picked_points_serialized = Self::serialize_epnp_picture_points(&picked_points);
        let real_camera_pose_serialized = Self::serialize_glm_vec3(&real_camera_pose);

        fs::create_dir_all("./presentations").expect("Error creating ./presentations/!");
        fs::write("./presentations/pixel_data.json", pixel_data_serialized)
            .expect("Error writing pixel_data.json");
        fs::write(
            "./presentations/picked_points.json",
            picked_points_serialized,
        )
        .expect("Error writing picked_points.json");
        fs::write(
            "./presentations/real_camera_pose.json",
            real_camera_pose_serialized,
        )
        .expect("Error writing real_camera_pose.json");
    }

    fn serialize_epnp_picture_points(
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
    ) -> String {
        let mut s = String::new();
        s.push_str("{\n");
        for p in picked_points {
            let mut p_serialized = "{".to_string();
            p_serialized.push_str(&format!("{{{}, {}}},", p.point.x, p.point.y)); // Pushes p.point
            p_serialized.push_str(&format!("{},", p.id.to_string()));
            p_serialized.push_str(&format!(
                "{{{}, {}, {}}},",
                p.opencv_color.x, p.opencv_color.y, p.opencv_color.z
            ));
            p_serialized.push_str(&format!(
                "{{{}, {}, {}}}",
                p.opengl_color.x, p.opengl_color.y, p.opengl_color.z
            ));
            p_serialized.push_str("},\n");
            s.push_str(&p_serialized);
        }
        s.push('}');
        s
    }

    fn serialize_glm_vec3(v: &glm::Vec3) -> String {
        let mut s = "{".to_string();
        s.push_str(&format!("{},{},{}", v.x, v.y, v.z));
        s.push('}');
        s
    }
}
