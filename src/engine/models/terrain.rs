use crate::engine::models;

pub struct Terrain {
    terrain_data: models::TerrainData,
}

impl Terrain {
    pub fn from_geotiff_file(geotiff_file_path: &str) -> Self {
        Self {
            terrain_data: models::TerrainData::from_geotiff_file(geotiff_file_path),
        }
    }
}
