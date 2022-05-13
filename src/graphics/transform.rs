use glam::f32::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub origin: Vec2,
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
        Self { origin: Vec2::ZERO, position: Vec2::ZERO, scale: Vec2::ONE, rotation: 0.0 };
}
