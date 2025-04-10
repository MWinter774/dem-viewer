use nalgebra_glm as glm;

use crate::{
    engine,
    engine::{models, renderers},
};

pub struct Scene {
    terrain: models::Terrain,
    terrain_renderer: renderers::TerrainRenderer,
    picking_renderer: renderers::PickingRenderer,
}

impl Scene {
    pub fn new(geotiff_file_path: &str, terrain_texture_file_path: &str) -> Self {
        let terrain =
            models::Terrain::from_geotiff_file(geotiff_file_path, terrain_texture_file_path);
        let terrain_renderer = renderers::TerrainRenderer::new();
        let picking_renderer = renderers::PickingRenderer::new();
        Self {
            terrain,
            terrain_renderer,
            picking_renderer,
        }
    }

    pub fn render(&mut self, camera: &engine::Camera) {
        self.terrain_renderer.render_terrain(
            &self.terrain,
            &(camera.get_pv_matrix()
                * self
                    .terrain
                    .get_terrain_model_position_data()
                    .get_model_matrix()),
        );
    }

    pub fn render_picking_frame(&mut self, camera: &engine::Camera) {
        self.picking_renderer.render_terrain_for_picking(
            &self.terrain,
            &(camera.get_pv_matrix()
                * self
                    .terrain
                    .get_terrain_model_position_data()
                    .get_model_matrix()),
        );
    }

    pub fn read_color_under_mouse(&self) -> glm::U8Vec4 {
        let mut color: glm::U8Vec4 = glm::U8Vec4::new_random();
        unsafe {
            gl::Flush();
            gl::Finish();
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::ReadPixels(
                400,
                300,
                1,
                1,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                color.as_mut_ptr().cast(),
            );
        }
        color
    }
}
