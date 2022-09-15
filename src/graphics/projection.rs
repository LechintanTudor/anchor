use crate::graphics::{self, Transform};
use glam::{Mat4, Vec2, Vec4};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub size: Vec2,
    pub anchor: Vec2,
}

impl Camera {
    #[inline]
    pub fn new(size: Vec2, anchor: Vec2) -> Self {
        Self { size, anchor }
    }

    pub fn to_ortho_mat4(&self) -> Mat4 {
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
    #[inline]
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    #[inline]
    pub fn fixed(size: Vec2) -> Self {
        Self { x: 0.0, y: 0.0, w: size.x, h: size.y }
    }

    pub fn fit(aspect_ratio: f32, box_size: Vec2) -> Self {
        let size = fit_aspect_ratio(aspect_ratio, box_size);
        let position = (box_size - size) * 0.5;

        Self { x: position.x, y: position.y, w: size.x, h: size.y }
    }

    #[inline]
    pub fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    #[inline]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.w, self.h)
    }

    #[inline]
    pub fn contains_position(&self, position: Vec2) -> bool {
        self.x <= position.x
            && position.x <= self.x + self.w
            && self.y <= position.y
            && position.y <= self.y + self.h
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Projection {
    pub camera: Camera,
    pub camera_transform: Transform,
    pub viewport: Viewport,
}

impl Projection {
    pub fn fill(size: Vec2) -> Self {
        Self {
            camera: Camera::new(size, graphics::ANCHOR_CENTER),
            camera_transform: Transform::DEFAULT,
            viewport: Viewport::fixed(size),
        }
    }

    pub fn to_ortho_mat4(&self) -> Mat4 {
        let object_transform = Transform {
            translation: -self.camera_transform.translation,
            rotation: -self.camera_transform.rotation,
            scale: self.camera_transform.scale,
        };

        self.camera.to_ortho_mat4() * object_transform.to_mat4()
    }

    pub fn window_to_world(&self, window_coords: Vec2) -> Vec2 {
        let normalized_window_coords =
            (window_coords - self.viewport.position()) / self.viewport.size();
        let ndc_window_coords = normalized_window_coords * 2.0 - Vec2::ONE;
        let ndc_window_coords = Vec2::new(ndc_window_coords.x, -ndc_window_coords.y);

        let inversed_projection_matrix = self.to_ortho_mat4().inverse();
        let ndc_window_coords = Vec4::from((ndc_window_coords, 0.0, 1.0));
        let world_coords = inversed_projection_matrix * ndc_window_coords;

        Vec2::new(world_coords.x, world_coords.y)
    }

    pub fn world_to_window(&self, world_coords: Vec2) -> Vec2 {
        let ndc_world_coords = self.to_ortho_mat4() * Vec4::from((world_coords, 0.0, 1.0));
        let ndc_world_coords = Vec2::new(ndc_world_coords.x, -ndc_world_coords.y);
        let normalized_world_coords = (ndc_world_coords + Vec2::ONE) * 0.5;
        normalized_world_coords * self.viewport.size() + self.viewport.position()
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
