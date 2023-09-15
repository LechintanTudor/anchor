use glam::{Vec2, Vec4};

#[derive(Clone, Copy, Default, Debug)]
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

    pub fn to_edges_vec4(&self) -> Vec4 {
        Vec4::new(self.y, self.x, self.y + self.h, self.x + self.w)
    }
}

impl From<[f32; 4]> for Bounds {
    fn from([x, y, w, h]: [f32; 4]) -> Self {
        Self { x, y, w, h }
    }
}
