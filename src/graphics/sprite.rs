use crate::graphics::{Color, Vec2};
use glam::const_vec2;

#[derive(Clone, Copy, Default, Debug)]
pub struct Sprite {
    pub index: usize,
    pub color: Color,
    pub flip_x: bool,
    pub flip_y: bool,
    pub size: Option<Vec2>,
    pub anchor: Vec2,
}

impl Sprite {
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
