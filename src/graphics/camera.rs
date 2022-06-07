use glam::Mat4;

use crate::graphics::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Vec2,
    pub size: Vec2,
}

impl Camera {
    pub fn ortho_matrix(&self) -> Mat4 {
        let half_size = self.size / 2.0;

        Mat4::orthographic_rh(
            self.position.x - half_size.x,
            self.position.x + half_size.x,
            self.position.y + half_size.y,
            self.position.y - half_size.y,
            0.0,
            1.0,
        )
    }
}
