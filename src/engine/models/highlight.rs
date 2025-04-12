use crate::engine::models;

pub struct Highlight {
    highlight_render_data: models::HighlightRenderData,
    highlight_opengl_object: models::HighlightOpenGLObject,
}

impl Highlight {
    pub fn new() -> Self {
        let highlight_render_data = models::HighlightRenderData::new();
        let highlight_opengl_object = models::HighlightOpenGLObject::new();
        Self {
            highlight_render_data,
            highlight_opengl_object,
        }
    }
}
