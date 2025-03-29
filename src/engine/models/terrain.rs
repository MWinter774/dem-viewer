use crate::engine::models;

pub struct Terrain {
    terrain_data: models::TerrainData,
    terrain_render_data: models::TerrainRenderData,
    terrain_opengl_object: models::TerrainOpenGLObject,
}

impl Terrain {
    pub fn from_geotiff_file(geotiff_file_path: &str) -> Self {
        let terrain_data = models::TerrainData::from_geotiff_file(geotiff_file_path);
        let terrain_render_data = models::TerrainRenderData::new(&terrain_data);
        let terrain_opengl_object = models::TerrainOpenGLObject::new();
        Self {
            terrain_data,
            terrain_render_data,
            terrain_opengl_object,
        }
    }
}
