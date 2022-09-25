use crate::graphics::Color;
use glam::Vec2;

/// Part of a sprite sheet to draw to the screen.
#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    /// Index of the sprite in the sprite sheet. Index `0` always refers to the entire image.
    pub index: usize,
    /// Color used to tint the sprite.
    pub color: Color,
    /// Whether to flip the sprite horizontally.
    pub flip_x: bool,
    /// Whether to flip the sprite verically.
    pub flip_y: bool,
    /// Custom size for the sprite. Use `None` to use the size defined by the sprite sheet.
    pub size: Option<Vec2>,
    /// Anchor point for applying transforms.
    pub anchor: Vec2,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            index: 0,
            color: Color::WHITE,
            flip_x: false,
            flip_y: false,
            size: None,
            anchor: Vec2::ZERO,
        }
    }
}

impl Sprite {
    /// Creates a sprite with the given `index`.
    #[inline]
    pub fn from_index(index: usize) -> Self {
        Self { index, ..Default::default() }
    }
}
