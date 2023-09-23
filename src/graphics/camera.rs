use crate::graphics::Bounds;
use glam::{Mat4, Vec2, Vec4};

#[derive(Clone, Debug)]
pub struct Camera {
    pub size: Vec2,
    pub anchor_offset: Vec2,
}

impl Camera {
    pub fn from_size<S>(size: S) -> Self
    where
        S: Into<Vec2>,
    {
        Self {
            size: size.into(),
            anchor_offset: Vec2::ZERO,
        }
    }

    pub fn anchor_offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Vec2>,
    {
        self.anchor_offset = offset.into();
        self
    }

    pub fn anchor_center(mut self) -> Self {
        self.anchor_offset = self.size * 0.5;
        self
    }

    pub fn ortho_matrix(&self) -> Mat4 {
        let tl = -self.anchor_offset;
        let br = self.size - self.anchor_offset;
        Mat4::orthographic_rh(tl.x, br.x, br.y, tl.y, 0.0, 1.0)
    }
}

impl From<&Camera> for Mat4 {
    fn from(camera: &Camera) -> Self {
        camera.ortho_matrix()
    }
}

pub fn world_coords(
    viewport_coords: impl Into<Vec2>,
    viewport: impl Into<Bounds>,
    projection: impl Into<Mat4>,
) -> Vec2 {
    let viewport_coords = viewport_coords.into();
    let viewport = viewport.into();
    let projection = projection.into();

    let normalized_viewport_coords = (viewport_coords - viewport.position()) / viewport.size();
    let ndc_viewport_coords = normalized_viewport_coords * 2.0 - Vec2::ONE;
    let ndc_viewport_coords = Vec4::new(ndc_viewport_coords.x, -ndc_viewport_coords.y, 0.0, 1.0);
    let viewport_coords = projection.inverse() * ndc_viewport_coords;
    Vec2::new(viewport_coords.x, viewport_coords.y)
}

pub fn viewport_coords(
    world_coords: impl Into<Vec2>,
    viewport: impl Into<Bounds>,
    projection: impl Into<Mat4>,
) -> Vec2 {
    let world_coords = world_coords.into();
    let viewport = viewport.into();
    let projection = projection.into();

    let ndc_world_coords = projection * Vec4::from((world_coords, 0.0, 1.0));
    let ndc_world_coords = Vec2::new(ndc_world_coords.x, -ndc_world_coords.y);
    let normalized_world_coords = (ndc_world_coords + Vec2::ONE) * 0.5;
    normalized_world_coords * viewport.size() + viewport.position()
}
