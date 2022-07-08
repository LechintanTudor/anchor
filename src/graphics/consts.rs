use glam::{const_vec2, Vec2};

pub use self::anchors::*;

pub mod anchors {
    use super::*;

    pub const ANCHOR_TOP_LEFT: Vec2 = const_vec2!([-0.5, 0.5]);
    pub const ANCHOR_TOP_CENTER: Vec2 = const_vec2!([0.0, 0.5]);
    pub const ANCHOR_TOP_RIGHT: Vec2 = const_vec2!([0.5, 0.5]);
    pub const ANCHOR_CENTER_LEFT: Vec2 = const_vec2!([-0.5, 0.0]);
    pub const ANCHOR_CENTER: Vec2 = const_vec2!([0.0, 0.0]);
    pub const ANCHOR_CENTER_RIGHT: Vec2 = const_vec2!([0.5, 0.0]);
    pub const ANCHOR_BOTTOM_LEFT: Vec2 = const_vec2!([-0.5, -0.5]);
    pub const ANCHOR_BOTTOM_CENTER: Vec2 = const_vec2!([0.0, -0.5]);
    pub const ANCHOR_BOTTOM_RIGHT: Vec2 = const_vec2!([0.5, -0.5]);
}
