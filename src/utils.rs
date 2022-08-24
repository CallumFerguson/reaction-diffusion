// macro_rules! console_log {
//     ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
// }

// pub struct Utils {}

use glam::{Mat4, Vec3, Vec4};

pub struct Utils {
    orthographic_size: f32,
    camera_pos: Vec3,
    // view: Mat4,
    // projection: Mat4
}

impl Utils {
    pub fn new(orthographic_size: f32, camera_pos: Vec3) -> Self {
        return Self {
            orthographic_size,
            camera_pos,
        };
    }
}

#[macro_export] macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}
