use glam::{Mat4, Vec3, Vec4};

pub struct Camera {
    orthographic_size: f32,
    camera_pos: Vec3,
    // view: Mat4,
    // projection: Mat4
}

impl Camera {
    pub fn new(orthographic_size: f32, camera_pos: Vec3) -> Self {
        return Self {
            orthographic_size,
            camera_pos
        }
    }
}
