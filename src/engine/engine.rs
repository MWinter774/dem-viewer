use crate::engine;

pub struct Engine {
    context: engine::EngineContext,
    camera: engine::Camera,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            context: engine::EngineContext::new("DEM Viewer", 800, 600),
            camera: engine::Camera::default(),
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

        while !window_should_close {
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
                .get_key_press_state(glfw::Key::B)
                == glfw::Action::Press
            {
                let _pixel_data = scene.take_screenshot(&self.camera);
            }

            scene.render_picking_frame(&self.camera);
            let pixel_data = scene.read_color_at_pixel(400, 300);
            let (object_index, primitive_id) = (pixel_data.x, pixel_data.z);
            scene.render(&self.camera);
            if object_index != 0 {
                scene.render_picking_highlight(&self.camera, primitive_id);
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
