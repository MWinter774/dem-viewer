use nalgebra_glm as glm;

use crate::{
    engine,
    engine::{camera_view, epnp, models, renderers},
};

use super::epnp::EPnPRealWorldPoint;

pub struct Scene {
    terrain: models::Terrain,
    terrain_renderer: renderers::TerrainRenderer,
    picking_renderer: renderers::PickingRenderer,
    highlight_renderer: renderers::HighlightRenderer,
    screenshot_renderer: renderers::ScreenshotRenderer,
    epnp_manager: epnp::EPnPManager,
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
        let epnp_manager = epnp::EPnPManager::new();
        Self {
            terrain,
            terrain_renderer,
            picking_renderer,
            highlight_renderer,
            screenshot_renderer,
            epnp_manager,
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

    pub fn render_picking_highlight(
        &mut self,
        camera: &engine::Camera,
        primitive_id: u32,
        highlight_color: &glm::Vec3,
    ) {
        self.highlight_renderer.render_highlight_on_terrain(
            &self.terrain,
            &(camera.get_pv_matrix()
                * self
                    .terrain
                    .get_terrain_model_position_data()
                    .get_model_matrix()),
            primitive_id,
            highlight_color,
        );
    }

    pub fn render_picked_points(&mut self, camera: &engine::Camera) {
        for picked_point in self.epnp_manager.get_real_world_points() {
            self.highlight_renderer.render_highlight_on_terrain(
                &self.terrain,
                &(camera.get_pv_matrix()
                    * self
                        .terrain
                        .get_terrain_model_position_data()
                        .get_model_matrix()),
                picked_point.primitive_id,
                &picked_point.color,
            );
        }
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

    // Returns true if user picked enough real world points
    pub fn render_picking_phase(
        &mut self,
        camera: &engine::Camera,
        should_pick_point: bool,
    ) -> bool {
        let mut need_more_real_world_points = true;

        // Gets the exact vertices the user is looking at
        self.render_picking_frame(camera);
        let pixel_data = self.read_color_at_pixel(400, 300);
        let (object_index, primitive_id) = (pixel_data.x, pixel_data.z);

        // Renders scene normally
        self.render(camera);

        // If the user looks at the terrain and didn't choose enough real world points, then render highlight of what he is looking at
        if object_index != 0
            && self.epnp_manager.get_real_world_points().len()
                < self.epnp_manager.get_image_points().len()
        {
            // Renders the highlight to be same color as the corresponding image point
            let highlight_color = &self.epnp_manager.get_image_points()
                [self.epnp_manager.get_real_world_points().len()]
            .opengl_color
            .clone();
            self.render_picking_highlight(camera, primitive_id, highlight_color);

            // If user already chose some points, draw an indicator
            self.render_picked_points(camera);

            // If should_pick_point(Usually is true when the mouse is clicked), then add it to the EPnPManager
            if should_pick_point {
                let real_world_point = EPnPRealWorldPoint {
                    point: self.pick_real_world_point_using_primitive_id(primitive_id),
                    color: highlight_color.clone(),
                    primitive_id,
                };
                need_more_real_world_points =
                    !self.epnp_manager.add_real_world_points(real_world_point);
            }
        }
        need_more_real_world_points
    }

    pub fn set_image_points(&mut self, image_points: Vec<epnp::EPnPPicturePoint>) {
        self.epnp_manager.set_image_points(image_points);
    }
    pub fn clear_real_world_points(&mut self) {
        self.epnp_manager.get_image_points_mut().clear();
    }

    fn pick_real_world_point_using_primitive_id(&self, primitive_id: u32) -> glm::Vec3 {
        let v = self
            .terrain
            .get_terrain_render_data()
            .get_vertices_using_primitive_id(primitive_id as usize);
        glm::vec3(v[0], v[1], v[2])
    }
}
