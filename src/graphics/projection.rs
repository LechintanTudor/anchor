use crate::graphics::{self, Transform};
use glam::{Mat4, Vec2};

#[derive(Clone, Copy, Default, Debug)]
pub enum CameraSize {
    #[default]
    Fill,
    Fixed {
        size: Vec2,
    },
    Scale {
        aspect_ratio: f32,
    },
}

impl CameraSize {
    pub fn camera_size(&self, window_size: Vec2) -> Vec2 {
        match *self {
            Self::Fill => window_size,
            Self::Fixed { size } => size,
            Self::Scale { aspect_ratio } => fit_aspect_ratio(aspect_ratio, window_size),
        }
    }

    pub fn letterbox_viewport_size(&self, window_size: Vec2) -> Vec2 {
        match *self {
            Self::Fill => window_size,
            Self::Fixed { size } => fit_aspect_ratio(size.x / size.y, window_size),
            Self::Scale { aspect_ratio } => fit_aspect_ratio(aspect_ratio, window_size),
        }
    }
}

fn fit_aspect_ratio(aspect_ratio: f32, window_size: Vec2) -> Vec2 {
    let width = window_size.y * aspect_ratio;

    if width <= window_size.x {
        Vec2::new(width, window_size.y)
    } else {
        Vec2::new(window_size.x, window_size.x / aspect_ratio)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum CameraViewport {
    #[default]
    Letterbox,
    Crop,
    Stretch,
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub size: CameraSize,
    pub viewport: CameraViewport,
    pub anchor: Vec2,
}

impl Default for Camera {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Camera {
    pub const DEFAULT: Self = Self {
        size: CameraSize::Fill,
        viewport: CameraViewport::Letterbox,
        anchor: graphics::ANCHOR_CENTER,
    };

    pub fn to_mat4(&self, window_size: Vec2) -> Mat4 {
        let size = self.size.camera_size(window_size);

        let top = size.y * (-0.5 - self.anchor.y);
        let left = size.x * (-0.5 - self.anchor.x);
        let bottom = size.y * (0.5 - self.anchor.y);
        let right = size.x * (0.5 - self.anchor.x);

        Mat4::orthographic_rh(left, right, bottom, top, 0.0, 1.0)
    }

    pub fn viewport_bounds(&self, window_size: Vec2) -> (f32, f32, f32, f32) {
        match self.viewport {
            CameraViewport::Letterbox => {
                let viewport_size = self.size.letterbox_viewport_size(window_size);
                let viewport_position = (window_size - viewport_size) / 2.0;

                (viewport_position.x, viewport_position.y, viewport_size.x, viewport_size.y)
            }
            CameraViewport::Crop => {
                let viewport_size = self.size.camera_size(window_size);
                let viewport_position = (window_size - viewport_size) / 2.0;

                (viewport_position.x, viewport_position.y, viewport_size.x, viewport_size.y)
            }
            CameraViewport::Stretch => (0.0, 0.0, window_size.x, window_size.y),
        }
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
