use crate::graphics::anchors::ANCHOR_CENTER;
use crate::graphics::Color;
use glam::Vec2;

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
    pub const DEFAULT: Self = Self {
        index: 0,
        color: Color::WHITE,
        flip_x: false,
        flip_y: false,
        size: None,
        anchor: ANCHOR_CENTER,
    };

    #[inline]
    pub fn from_index(index: usize) -> Self {
        Self { index, ..Self::DEFAULT }
    }
}
