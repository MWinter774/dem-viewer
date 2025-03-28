use super::models;

pub struct Scene {
    terrain: models::Terrain,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            terrain: models::Terrain::new(),
        }
    }
}
