use std::cmp;

use crate::engine::models;

pub struct TerrainRenderData {
    vertices: Vec<f32>,
    vertices_ids: Vec<u32>,
    uv: Vec<f32>,
    indices: Vec<u32>,
    min_height: f32,
    max_height: f32,
}

impl TerrainRenderData {
    pub fn new(terrain_data: &models::TerrainModelData) -> Self {
        let (vertices, vertices_ids, uv, indices, min_height, max_height) =
            Self::generate_data_from_terrain_data(terrain_data);
        Self {
            vertices,
            vertices_ids,
            uv,
            indices,
            min_height,
            max_height,
        }
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }
    pub fn get_vertices_ids(&self) -> &Vec<u32> {
        &self.vertices_ids
    }
    pub fn get_uv(&self) -> &Vec<f32> {
        &self.uv
    }
    pub fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }
    pub fn get_min_height(&self) -> f32 {
        self.min_height
    }
    pub fn get_max_height(&self) -> f32 {
        self.max_height
    }

    fn generate_data_from_terrain_data(
        terrain_data: &models::TerrainModelData,
    ) -> (Vec<f32>, Vec<u32>, Vec<f32>, Vec<u32>, f32, f32) {
        let mut vertices: Vec<f32> = Vec::new();
        let mut vertices_ids: Vec<u32> = Vec::new();
        let mut uv: Vec<f32> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let (rs_width, rs_height) = terrain_data.get_size();

        // TODO: Compute scaling based on model, doesn't work well
        // let geo_transform = terrain_data.get_geo_transform();
        // let (x, y) = (geo_transform[0], geo_transform[3]);
        // let (x_right, y_bottom) = (
        //     x + (geo_transform[1] * (rs_width as f64)),
        //     y + (geo_transform[5] * (rs_height as f64)),
        // );
        // let (absolute_w, absolute_h) = (x_right - x, y - y_bottom);
        // let (x_res, y_res) = (
        //     absolute_w / (rs_width as f64),
        //     absolute_h / (rs_height as f64),
        // );
        // let (scale_x, scale_y, height_scale): (f32, f32, f32) = (x_res as f32, y_res as f32, 1.0);

        // Set fixed scaling
        let (scale_x, scale_y, height_scale) = (10.0, 10.0, 1.0);

        let buf = terrain_data.get_data();
        let (mut min_height, mut max_height) = (buf[0] as f32, buf[0] as f32);

        for j in 0..rs_height {
            for i in 0..rs_width {
                let x = (i) as f32 * scale_x;
                let y = j as f32 * scale_y;
                let z = buf[j * rs_width + i] as f32 * height_scale;
                vertices.push(x);
                vertices.push(z);
                vertices.push(y);

                vertices_ids.push((j * rs_width + i) as u32);

                min_height = z.min(min_height);
                max_height = z.max(max_height);

                let u = i as f32 / (rs_width - 1) as f32;
                let v = j as f32 / (rs_height - 1) as f32;
                uv.push(u);
                uv.push(v);
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

        (vertices, vertices_ids, uv, indices, min_height, max_height)
    }

    pub fn get_vertices_using_primitive_id(&self, primitive_id: usize) -> [f32; 9] {
        let (idx0, idx1, idx2) = (
            (self.indices[primitive_id] as usize),
            (self.indices[primitive_id] + 1) as usize,
            (self.indices[primitive_id] + 2) as usize,
        );
        [
            self.vertices[idx0],
            self.vertices[idx0 + 1],
            self.vertices[idx0 + 2],
            self.vertices[idx1],
            self.vertices[idx1 + 1],
            self.vertices[idx1 + 2],
            self.vertices[idx2],
            self.vertices[idx2 + 1],
            self.vertices[idx2 + 2],
        ]
    }
}
