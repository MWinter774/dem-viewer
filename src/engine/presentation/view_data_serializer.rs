use crate::engine;
use nalgebra_glm as glm;

pub struct ViewDataSerializer {}

impl ViewDataSerializer {
    pub fn serialize_pixel_data(pixel_data: &Vec<u8>) -> String {
        serde_json::to_string(&pixel_data).unwrap()
    }

    pub fn serialize_epnp_picture_points(
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
    ) -> String {
        let mut s = String::new();
        s.push_str("[\n");
        for p in picked_points {
            let mut p_serialized = "{".to_string();
            p_serialized.push_str(&format!("\"point\":[{},{}],", p.point.x, p.point.y)); // Pushes p.point
            p_serialized.push_str(&format!("\"id\":{},", p.id.to_string()));
            p_serialized.push_str(&format!(
                "\"opencv_color\":[{},{},{}],",
                p.opencv_color.x, p.opencv_color.y, p.opencv_color.z
            ));
            p_serialized.push_str(&format!(
                "\"opengl_color\":[{}, {}, {}]",
                p.opengl_color.x, p.opengl_color.y, p.opengl_color.z
            ));
            p_serialized.push_str("},\n");
            s.push_str(&p_serialized);
        }
        s.remove(s.len() - 2);
        s.push(']');
        s
    }

    pub fn serialize_glm_vec3(v: &glm::Vec3) -> String {
        let mut s = "[".to_string();
        s.push_str(&format!("{},{},{}", v.x, v.y, v.z));
        s.push(']');
        s
    }
}
