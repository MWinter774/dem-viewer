static mut LAST_X: f64 = 400.0;
static mut LAST_Y: f64 = 300.0;

static mut X_OFFSET: f64 = 0.0;
static mut Y_OFFSET: f64 = 0.0;

static mut MOUSE_MOVED: bool = false;
static mut LEFT_MOUSE_BUTTON_PRESSED: bool = false;

pub struct Mouse {}

impl Mouse {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_delta_movement(&self) -> (f64, f64) {
        unsafe {
            if MOUSE_MOVED {
                MOUSE_MOVED = false;
                (X_OFFSET, Y_OFFSET)
            } else {
                (0.0, 0.0)
            }
        }
    }

    pub fn is_moved(&self) -> bool {
        unsafe { MOUSE_MOVED }
    }

    pub fn is_left_mouse_button_pressed(&self) -> bool {
        unsafe { LEFT_MOUSE_BUTTON_PRESSED }
    }
}

pub fn mouse_movement_callback(_: &mut glfw::Window, xpos: f64, ypos: f64) {
    unsafe {
        MOUSE_MOVED = true;
        X_OFFSET = xpos - LAST_X;
        Y_OFFSET = LAST_Y - ypos;
        LAST_X = xpos;
        LAST_Y = ypos;
    }
}

pub fn mouse_button_callback(
    _: &mut glfw::Window,
    mouse_button: glfw::MouseButton,
    action: glfw::Action,
    modifiers: glfw::Modifiers,
) {
    if mouse_button == glfw::MouseButtonLeft {
        unsafe {
            LEFT_MOUSE_BUTTON_PRESSED = action == glfw::Action::Press;
        }
    }
}
