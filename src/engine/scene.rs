use crate::engine::{models, renderers};

pub struct Scene {
    terrain: models::Terrain,
    terrain_renderer: renderers::TerrainRenderer,
}

impl Scene {
    pub fn new(geotiff_file_path: &str) -> Self {
        let terrain = models::Terrain::from_geotiff_file(geotiff_file_path);
        let terrain_renderer = renderers::TerrainRenderer::new();
        Self {
            terrain,
            terrain_renderer,
        }
    }
}
