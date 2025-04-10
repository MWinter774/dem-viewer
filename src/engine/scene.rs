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

    pub fn picking_phase(&self) {
    }
}
