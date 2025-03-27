use crate::engine;

pub struct Engine {
    context: engine::EngineContext,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            context: engine::EngineContext::new("DEM Viewer", 800, 600),
        }
    }

    pub fn run(&mut self) {}
}
