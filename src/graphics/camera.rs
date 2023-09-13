use glam::{Mat4, Vec2};

#[derive(Clone, Debug)]
pub struct Camera {
    pub size: Vec2,
    pub anchor: Vec2,
}

impl Camera {
    pub fn from_size(size: Vec2) -> Self {
        Self { size, anchor: Vec2::ZERO }
    }

    pub fn ortho_matrix(&self) -> Mat4 {
        let tl = self.size * (0.0 - self.anchor);
        let br = self.size * (1.0 - self.anchor);
        Mat4::orthographic_rh(tl.x, br.x, br.y, tl.y, 0.0, 1.0)
    }
}

impl From<&Camera> for Mat4 {
    fn from(camera: &Camera) -> Self {
        camera.ortho_matrix()
    }
}
