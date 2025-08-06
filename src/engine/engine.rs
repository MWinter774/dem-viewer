use nalgebra_glm as glm;

use crate::engine::{self, camera_view, feature_matching, transformations::view};

pub struct Engine {
    context: engine::EngineContext,
    camera: engine::Camera,
    camera_view_application: camera_view::CameraViewApp,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            context: engine::EngineContext::new("DEM Viewer", 800, 600),
            camera: engine::Camera::default(),
            camera_view_application: camera_view::CameraViewApp::new(800, 600),
        }
    }

    pub fn run(&mut self) {
        let mut window_should_close = false;

        self.context.initialize();

        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LESS);
        }

        let mut scene = engine::Scene::new("DEM\\1.tif", "textures\\gray.jpeg", 800, 600);

        let mut should_refocus_window = false;
        let mut picking_phase = false;
        let mut real_camera_pose = glm::vec3(0.0, 0.0, 0.0);
        let feature_match = true;

        while !window_should_close {
            if should_refocus_window {
                self.context.highlight_window();
                should_refocus_window = false;
            }
            // Swap the buffers of the opengl window and updates event pipeline
            let frame_data = self.context.next_frame();
            self.camera
                .update_input(&frame_data.input_system, frame_data.delta_time as f32);

            unsafe {
                gl::Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
                gl::ClearColor(0.0, 0.0, 0.5, 0.0);
            }

            if frame_data
                .input_system
                .keyboard
                .is_key_pressed(glfw::Key::P)
            {
                let pixel_data = scene.take_screenshot(&self.camera);
                let picked_points = self
                    .camera_view_application
                    .capture_clicked_points(pixel_data.clone());
                scene.set_image_points(picked_points);
                scene.clear_real_world_points();
                should_refocus_window = true;
                picking_phase = true;
                real_camera_pose = self.camera.get_position().clone();

                // Add the new view to the views system
                scene.add_view_to_feature_matching(&pixel_data, &real_camera_pose);
            }
            if frame_data
                .input_system
                .keyboard
                .is_key_pressed(glfw::Key::B)
            {
                if feature_match {
                    let pixel_data = scene.take_screenshot(&self.camera);
                    let camera_position_result = scene.feature_match(&pixel_data);
                    match camera_position_result {
                        Ok(pos) => println!("{pos}"),
                        Err(e) => println!("{e}"),
                    }
                } else {
                    match scene.compute_camera_pose(&self.camera) {
                        Ok(computed_camera_pose) => {
                            println!(
                                "Computed camera pose: ({}, {}, {})",
                                computed_camera_pose.x,
                                computed_camera_pose.y,
                                computed_camera_pose.z
                            );
                            println!(
                                "Real camera pose: ({}, {}, {})",
                                real_camera_pose.x, real_camera_pose.y, real_camera_pose.z
                            );
                            println!(
                                "Error: {}",
                                glm::l2_norm(
                                    &(computed_camera_pose.normalize()
                                        - real_camera_pose.normalize())
                                )
                            );
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                }
            }

            if picking_phase {
                // If user chose enough real world points, then cease the picking phase
                picking_phase = scene.render_picking_phase(
                    &self.camera,
                    frame_data.input_system.mouse.is_left_mouse_button_clicked(),
                );
                if false == picking_phase
                // If finished picking the last point for current view
                {
                    match scene.compute_camera_pose(&self.camera) {
                        Ok(computed_camera_pose) => {
                            scene.update_estimated_camera_position_for_feature_matching(
                                &computed_camera_pose,
                            );
                            println!("Views count: {}", scene.get_num_view_of_feature_matcher());
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                }
            } else {
                scene.render(&self.camera);
            }

            if frame_data
                .input_system
                .keyboard
                .get_key_press_state(glfw::Key::Escape)
                == glfw::Action::Press
            {
                window_should_close = true;
            }
        }
        self.context.set_should_terminate(true);
    }
}
