use crate::graphics::anchors::ANCHOR_TOP_LEFT;
use crate::graphics::Transform;
use glam::{Mat4, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub size: Option<Vec2>,
    pub anchor: Vec2,
}

impl Default for Camera {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Camera {
    pub const DEFAULT: Self = Self { size: None, anchor: ANCHOR_TOP_LEFT };

    #[inline]
    pub const fn new(size: Option<Vec2>, anchor: Vec2) -> Self {
        Self { size, anchor }
    }

    pub fn to_mat4(&self, window_size: Vec2) -> Mat4 {
        let size = self.size.unwrap_or(window_size);

        let top = size.y * (-0.5 - self.anchor.y);
        let left = size.x * (-0.5 - self.anchor.x);
        let bottom = size.y * (0.5 - self.anchor.y);
        let right = size.x * (0.5 - self.anchor.x);

        Mat4::orthographic_rh(left, right, bottom, top, 0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Projection {
    pub camera: Camera,
    pub transform: Transform,
}

impl Default for Projection {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Projection {
    pub const DEFAULT: Self = Self { camera: Camera::DEFAULT, transform: Transform::DEFAULT };

    #[inline]
    pub const fn new(camera: Camera, transform: Transform) -> Self {
        Self { camera, transform }
    }

    #[inline]
    pub fn to_mat4(&self, window_size: Vec2) -> Mat4 {
        self.camera.to_mat4(window_size) * self.transform.to_mat4()
    }
}

impl From<Camera> for Projection {
    #[inline]
    fn from(camera: Camera) -> Self {
        Self { camera, transform: Transform::DEFAULT }
    }
}

impl From<(Camera, Transform)> for Projection {
    #[inline]
    fn from((camera, transform): (Camera, Transform)) -> Self {
        Self { camera, transform }
    }
}
