use crate::graphics::{self, Transform};
use glam::{Mat4, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub size: Vec2,
    pub anchor: Vec2,
}

impl Camera {
    pub fn to_mat4(&self) -> Mat4 {
        let top = self.size.y * (-0.5 - self.anchor.y);
        let left = self.size.x * (-0.5 - self.anchor.x);
        let bottom = self.size.y * (0.5 - self.anchor.y);
        let right = self.size.x * (0.5 - self.anchor.x);

        Mat4::orthographic_rh(left, right, bottom, top, 0.0, 1.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Viewport {
    pub fn fixed(size: Vec2) -> Self {
        Self { x: 0.0, y: 0.0, w: size.x, h: size.y }
    }

    pub fn fit(aspect_ratio: f32, box_size: Vec2) -> Self {
        let size = fit_aspect_ratio(aspect_ratio, box_size);
        let position = (box_size - size) * 0.5;

        Self { x: position.x, y: position.y, w: size.x, h: size.y }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Projection {
    pub camera: Camera,
    pub transform: Transform,
    pub viewport: Viewport,
}

impl Projection {
    pub fn to_mat4(&self) -> Mat4 {
        self.camera.to_mat4() * self.transform.to_mat4()
    }
}

pub trait ProjectionBuilder: 'static {
    fn build_projection(&self, window_size: Vec2) -> Projection;
}

impl<F> ProjectionBuilder for F
where
    F: Fn(Vec2) -> Projection + 'static,
{
    fn build_projection(&self, window_size: Vec2) -> Projection {
        self(window_size)
    }
}

pub fn projection_builder_fill() -> impl ProjectionBuilder {
    |window_size: Vec2| -> Projection {
        Projection {
            camera: Camera { size: window_size, anchor: graphics::ANCHOR_CENTER },
            transform: Transform::DEFAULT,
            viewport: Viewport::fixed(window_size),
        }
    }
}

pub fn projection_builder_fixed(size: Vec2, keep_aspect_ratio: bool) -> impl ProjectionBuilder {
    move |window_size: Vec2| -> Projection {
        Projection {
            camera: Camera { size, anchor: graphics::ANCHOR_CENTER },
            transform: Transform::DEFAULT,
            viewport: {
                if keep_aspect_ratio {
                    Viewport::fit(size.x / size.y, window_size)
                } else {
                    Viewport::fixed(window_size)
                }
            },
        }
    }
}

pub fn projection_builder_scaled(aspect_ratio: f32) -> impl ProjectionBuilder {
    move |window_size: Vec2| -> Projection {
        let camera_size = fit_aspect_ratio(aspect_ratio, window_size);

        Projection {
            camera: Camera { size: camera_size, anchor: graphics::ANCHOR_CENTER },
            transform: Transform::DEFAULT,
            viewport: Viewport::fit(aspect_ratio, window_size),
        }
    }
}

fn fit_aspect_ratio(aspect_ratio: f32, box_size: Vec2) -> Vec2 {
    let width = box_size.y * aspect_ratio;

    if width <= box_size.x {
        Vec2::new(width, box_size.y)
    } else {
        Vec2::new(box_size.x, box_size.x / aspect_ratio)
    }
}
