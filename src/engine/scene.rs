use nalgebra_glm as glm;

use crate::engine::{self, epnp, feature_matching, models, renderers};

use super::epnp::EPnPRealWorldPoint;

pub struct Scene {
    terrain: models::Terrain,
    terrain_renderer: renderers::TerrainRenderer,
    picking_renderer: renderers::PickingRenderer,
    highlight_renderer: renderers::HighlightRenderer,
    screenshot_renderer: renderers::ScreenshotRenderer,
    epnp_manager: epnp::EPnPManager,
    feature_matcher: feature_matching::FeatureMatcher,
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
        let feature_matcher = feature_matching::FeatureMatcher::new(window_width, window_height);
        Self {
            terrain,
            terrain_renderer,
            picking_renderer,
            highlight_renderer,
            screenshot_renderer,
            epnp_manager,
            feature_matcher,
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
    pub fn get_real_world_points(&self) -> &Vec<epnp::EPnPRealWorldPoint> {
        self.epnp_manager.get_real_world_points()
    }
    pub fn set_real_world_points(&mut self, real_world_points: Vec<epnp::EPnPRealWorldPoint>) {
        self.epnp_manager.set_real_world_points(real_world_points);
    }

    pub fn compute_camera_pose(&self, camera: &engine::Camera) -> Result<glm::Vec3, &str> {
        self.epnp_manager
            .compute_camera_pose(camera.get_projection_matrix_object())
    }

    fn pick_real_world_point_using_primitive_id(&self, primitive_id: u32) -> glm::Vec3 {
        // Get vertex position in mesh coords
        let mesh_vertex = self
            .terrain
            .get_terrain_render_data()
            .get_mesh_vertex_using_primitive_id(primitive_id as usize);

        // Transforms the mesh vertex to real world coords
        let real_world_vertex = glm::vec4(mesh_vertex.x, mesh_vertex.y, mesh_vertex.z, 1.0);
        let real_world_vertex = self
            .terrain
            .get_terrain_model_position_data()
            .get_model_matrix()
            * real_world_vertex;

        real_world_vertex.xyz()
    }

    pub fn add_view_to_feature_matching(
        &mut self,
        pixel_data: &Vec<u8>,
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
        real_camera_position: &glm::Vec3,
    ) {
        self.feature_matcher
            .add_view(pixel_data, picked_points, &Vec::new(), real_camera_position);
    }
    pub fn add_defined_view_to_feature_matching(
        &mut self,
        pixel_data: &Vec<u8>,
        picked_points: &Vec<engine::epnp::EPnPPicturePoint>,
        real_world_points: &Vec<epnp::EPnPRealWorldPoint>,
        real_camera_position: &glm::Vec3,
    ) {
        self.feature_matcher.add_view(
            pixel_data,
            picked_points,
            real_world_points,
            real_camera_position,
        );
    }

    pub fn update_estimated_camera_position_for_feature_matching(
        &mut self,
        estimated_camera_position: &glm::Vec3,
    ) {
        self.feature_matcher
            .update_estimated_camera_position(estimated_camera_position);
    }

    pub fn get_num_view_of_feature_matcher(&self) -> usize {
        self.feature_matcher.get_num_views()
    }

    pub fn estimate_camera_position_using_feature_match(
        &mut self,
        pixel_data: &Vec<u8>,
        camera: &engine::Camera,
    ) -> Result<glm::Vec3, &str> {
        let (estimated_picked_points, matching_view) =
            self.feature_matcher.estimate_picked_points(pixel_data)?;

        self.epnp_manager.set_image_points(estimated_picked_points);
        self.epnp_manager
            .set_real_world_points(matching_view.get_real_world_points().clone());
        self.epnp_manager
            .compute_camera_pose(camera.get_projection_matrix_object())
    }
}
