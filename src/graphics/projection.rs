use crate::graphics::{self, Transform};
use glam::{Mat4, Vec2, Vec4};

/// Projects world coords into normalized device coords.
#[derive(Clone, Copy, Debug)]
pub struct Camera {
    /// World bounds captured by the camera.
    pub size: Vec2,
    /// Camera anchor point.
    pub anchor: Vec2,
}

impl Camera {
    /// Creates a camera with the given `size` and `anchor`.
    #[inline]
    pub fn new(size: Vec2, anchor: Vec2) -> Self {
        Self { size, anchor }
    }

    /// Creates a camera with the given `size` anchored at its center.
    #[inline]
    pub fn from_size(size: Vec2) -> Self {
        Self { size, anchor: Vec2::ZERO }
    }

    /// Creates an orthographic projection matrix from self's properties.
    pub fn to_ortho_mat4(&self) -> Mat4 {
        let tl_corner = self.size * (-0.5 - self.anchor);
        let br_corner = self.size * (0.5 - self.anchor);
        Mat4::orthographic_rh(tl_corner.x, br_corner.x, br_corner.y, tl_corner.y, 0.0, 1.0)
    }
}

/// Projects normalized device coords into window coords.
#[derive(Clone, Copy, Debug)]
pub struct Viewport {
    /// Offset from the left of the window.
    pub x: f32,
    /// Offset from the top of the window.
    pub y: f32,
    /// Width of the viewport.
    pub w: f32,
    /// Height of the viewport.
    pub h: f32,
}

impl Viewport {
    /// Creates a viewport from the given values.
    #[inline]
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    /// Creates a viewport with the given `size`.
    #[inline]
    pub fn fixed(size: Vec2) -> Self {
        Self { x: 0.0, y: 0.0, w: size.x, h: size.y }
    }

    /// Creates a viewport with the given aspect ratio that is contained within the provided
    /// bounds.
    pub fn fit(aspect_ratio: f32, bounds: Vec2) -> Self {
        let size = fit_aspect_ratio(aspect_ratio, bounds);
        let position = (bounds - size) * 0.5;

        Self { x: position.x, y: position.y, w: size.x, h: size.y }
    }

    /// Returns the offset of the viewport as a [Vec2].
    #[inline]
    pub fn offset(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Returns the size of the viewport as a [Vec2].
    #[inline]
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.w, self.h)
    }

    /// Returns whether the viewport contains the given window coordinates.
    #[inline]
    pub fn contains(&self, coords: Vec2) -> bool {
        self.x <= coords.x
            && coords.x <= self.x + self.w
            && self.y <= coords.y
            && coords.y <= self.y + self.h
    }
}

impl From<(f32, f32, f32, f32)> for Viewport {
    #[inline]
    fn from((x, y, w, h): (f32, f32, f32, f32)) -> Self {
        Self::new(x, y, w, h)
    }
}

impl From<[f32; 4]> for Viewport {
    #[inline]
    fn from([x, y, w, h]: [f32; 4]) -> Self {
        Self::new(x, y, w, h)
    }
}

/// Projects world coords into window coords.
#[derive(Clone, Copy, Debug)]
pub struct Projection {
    /// Camera to use.
    pub camera: Camera,
    /// Transform to apply to the camera.
    pub camera_transform: Transform,
    /// Viewport to use.
    pub viewport: Viewport,
}

impl Projection {
    /// Creates a projection that fills the given `size`.
    pub fn fill(size: Vec2) -> Self {
        Self {
            camera: Camera::new(size, graphics::ANCHOR_CENTER),
            camera_transform: Transform::default(),
            viewport: Viewport::fixed(size),
        }
    }

    pub fn fit(camera_size: Vec2, surface_size: Vec2) -> Self {
        let aspect_ratio = camera_size.x / camera_size.y;

        Self {
            camera: Camera::from_size(camera_size),
            camera_transform: Transform::default(),
            viewport: Viewport::fit(aspect_ratio, surface_size),
        }
    }

    #[inline]
    pub fn with_camera_transform(self, camera_transform: Transform) -> Self {
        Self { camera_transform, ..self }
    }

    /// Creates an orthographics projection matrix from self's properties.
    pub fn to_ortho_mat4(&self) -> Mat4 {
        let object_transform = Transform {
            translation: -self.camera_transform.translation,
            rotation: -self.camera_transform.rotation,
            scale: Vec2::ONE / self.camera_transform.scale,
        };

        self.camera.to_ortho_mat4() * object_transform.to_mat4()
    }

    /// Converts window coords to world coords.
    pub fn window_to_world(&self, window_coords: Vec2) -> Vec2 {
        let normalized_window_coords =
            (window_coords - self.viewport.offset()) / self.viewport.size();
        let ndc_window_coords = normalized_window_coords * 2.0 - Vec2::ONE;
        let ndc_window_coords = Vec2::new(ndc_window_coords.x, -ndc_window_coords.y);

        let inversed_projection_matrix = self.to_ortho_mat4().inverse();
        let ndc_window_coords = Vec4::from((ndc_window_coords, 0.0, 1.0));
        let world_coords = inversed_projection_matrix * ndc_window_coords;

        Vec2::new(world_coords.x, world_coords.y)
    }

    /// COnverts world coords to window coords.
    pub fn world_to_window(&self, world_coords: Vec2) -> Vec2 {
        let ndc_world_coords = self.to_ortho_mat4() * Vec4::from((world_coords, 0.0, 1.0));
        let ndc_world_coords = Vec2::new(ndc_world_coords.x, -ndc_world_coords.y);
        let normalized_world_coords = (ndc_world_coords + Vec2::ONE) * 0.5;
        normalized_world_coords * self.viewport.size() + self.viewport.offset()
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
