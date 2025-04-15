use nalgebra_glm as glm;

use crate::{
    engine,
    engine::{models, renderers, camera_view},
};

pub struct Scene {
    terrain: models::Terrain,
    terrain_renderer: renderers::TerrainRenderer,
    picking_renderer: renderers::PickingRenderer,
    highlight_renderer: renderers::HighlightRenderer,
    screenshot_renderer: renderers::ScreenshotRenderer,
}

impl Scene {
    pub fn new(
        geotiff_file_path: &str,
        terrain_texture_file_path: &str,
        window_width: usize,
        window_height: usize,
    ) -> Self {
        let terrain =
            models::Terrain::from_geotiff_file(geotiff_file_path, terrain_texture_file_path);

        let terrain_renderer = renderers::TerrainRenderer::new(&terrain);
        let highlight_renderer = renderers::HighlightRenderer::new(&terrain);
        let picking_renderer =
            renderers::PickingRenderer::new(&terrain, window_width, window_height);
        let screenshot_renderer = renderers::ScreenshotRenderer::new(window_width, window_height);
        Self {
            terrain,
            terrain_renderer,
            picking_renderer,
            highlight_renderer,
            screenshot_renderer,
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

    pub fn render_picking_highlight(&mut self, camera: &engine::Camera, primitive_id: u32, picked_points: &Vec<camera_view::CameraViewPoint>) {
        self.highlight_renderer.render_highlight_on_terrain(
            &self.terrain,
            &(camera.get_pv_matrix()
                * self
                    .terrain
                    .get_terrain_model_position_data()
                    .get_model_matrix()),
            primitive_id,
            picked_points,
        );
    }

    pub fn take_screenshot(&mut self, camera: &engine::Camera) -> Vec<u8> {
        self.screenshot_renderer.start_record_screenshot();
        self.render(camera);
        let screenshot = self.screenshot_renderer.screenshot();
        self.screenshot_renderer.stop_record_screenshot();
        screenshot
    }

    pub fn read_color_at_pixel(&self, x: gl::types::GLint, y: gl::types::GLint) -> glm::UVec3 {
        self.picking_renderer.read_pixel_at(x, y)
    }
}
