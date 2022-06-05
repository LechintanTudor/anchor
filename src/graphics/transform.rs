use crate::graphics::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub offset: Vec2,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Transform {
    pub const DEFAULT: Self =
        Self { offset: Vec2::ZERO, position: Vec2::ZERO, scale: Vec2::ONE, rotation: 0.0 };
}
