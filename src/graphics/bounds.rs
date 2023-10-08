use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Debug, Deserialize, Serialize)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Bounds {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub const fn position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub const fn size(&self) -> Vec2 {
        Vec2::new(self.w, self.h)
    }

    pub fn top_left(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn bottom_right(&self) -> Vec2 {
        Vec2::new(self.x + self.w, self.y + self.h)
    }
}

impl From<[f32; 4]> for Bounds {
    fn from([x, y, w, h]: [f32; 4]) -> Self {
        Self { x, y, w, h }
    }
}
