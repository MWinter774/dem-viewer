use crate::engine::models;

pub struct TerrainRenderData {
    vertices: Vec<f32>,
    indices: Vec<u32>,
}

impl TerrainRenderData {
    pub fn new(terrain_data: &models::TerrainData) -> Self {
        let (vertices, indices) = Self::generate_data_from_terrain_data(terrain_data);
        Self { vertices, indices }
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }
    
    pub fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }

    fn generate_data_from_terrain_data(terrain_data: &models::TerrainData) -> (Vec<f32>, Vec<u32>) {
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let geo_transform = terrain_data.get_geo_transform();
        let (x, y) = (geo_transform[0], geo_transform[3]);

        let (rs_width, rs_height) = terrain_data.get_size();

        let (x_right, y_bottom) = (
            x + (geo_transform[1] * (rs_width as f64)),
            y + (geo_transform[5] * (rs_height as f64)),
        );
        let (absolute_w, absolute_h) = (x_right - x, y - y_bottom);
        let (x_res, y_res) = (
            absolute_w / (rs_width as f64),
            absolute_h / (rs_height as f64),
        );

        let (scale_x, scale_y, height_scale): (f32, f32, f32) = (x_res as f32, y_res as f32, 1.0);

        let buf = terrain_data.get_data();

        for j in 0..rs_height {
            for i in 0..rs_width {
                let x = (i) as f32 * scale_x;
                let y = j as f32 * scale_y;
                let z = buf[j * rs_width + i] as f32 * height_scale;
                vertices.push(x);
                vertices.push(z);
                vertices.push(y);
            }
        }

        for j in 0..(rs_height - 1) {
            for i in 0..(rs_width - 1) {
                let top_left = j * rs_width + i;
                let top_right = top_left + 1;
                let bottom_left = (j + 1) * rs_width + i;
                let bottom_right = bottom_left + 1;

                // First triangle
                indices.push(top_left as u32);
                indices.push(bottom_left as u32);
                indices.push(top_right as u32);

                // Second triangle
                indices.push(bottom_left as u32);
                indices.push(bottom_right as u32);
                indices.push(top_right as u32);
            }
        }

        (vertices, indices)
    }
}
