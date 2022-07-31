use glam::Vec2;

pub mod anchors {
    use super::*;

    pub const ANCHOR_TOP_LEFT: Vec2 = Vec2::new(-0.5, -0.5);
    pub const ANCHOR_TOP_CENTER: Vec2 = Vec2::new(0.0, -0.5);
    pub const ANCHOR_TOP_RIGHT: Vec2 = Vec2::new(0.5, -0.5);
    pub const ANCHOR_CENTER_LEFT: Vec2 = Vec2::new(-0.5, 0.0);
    pub const ANCHOR_CENTER: Vec2 = Vec2::new(0.0, 0.0);
    pub const ANCHOR_CENTER_RIGHT: Vec2 = Vec2::new(0.5, 0.0);
    pub const ANCHOR_BOTTOM_LEFT: Vec2 = Vec2::new(-0.5, 0.5);
    pub const ANCHOR_BOTTOM_CENTER: Vec2 = Vec2::new(0.0, 0.5);
    pub const ANCHOR_BOTTOM_RIGHT: Vec2 = Vec2::new(0.5, 0.5);
}
