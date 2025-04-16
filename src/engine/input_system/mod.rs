pub use self::input_system::InputSystem;
pub use self::keyboard::Keyboard;
pub use self::keyboard_press_detector::KeyboardPressDetector;
pub use self::mouse::Mouse;
pub use self::mouse_click_detector::MouseClickDetector;

mod input_system;
mod keyboard;
mod keyboard_press_detector;
pub mod mouse;
mod mouse_click_detector;
