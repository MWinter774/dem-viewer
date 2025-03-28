use super::models;

pub struct Scene {
    terrain: models::Terrain,
}

impl Scene {
    pub fn new(geotiff_file_path: &str) -> Self {
        Self {
            terrain: models::Terrain::from_geotiff_file(geotiff_file_path),
        }
    }
}
