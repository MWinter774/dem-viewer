use crate::{
    engine,
    engine::{models, renderers},
};

pub struct Scene {
    terrain: models::Terrain,
    terrain_renderer: renderers::TerrainRenderer,
}

impl Scene {
    pub fn new(geotiff_file_path: &str) -> Self {
        let terrain = models::Terrain::from_geotiff_file(geotiff_file_path, "textures\\grass.jpg");
        let terrain_renderer = renderers::TerrainRenderer::new();
        Self {
            terrain,
            terrain_renderer,
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
}
