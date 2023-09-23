use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TextInstance {
    pub size: Vec2,
    pub text_index: u32,
    pub _padding: u32,
    pub scale_rotation_x_axis: Vec2,
    pub scale_rotation_y_axis: Vec2,
    pub translation: Vec2,
    pub anchor_offset: Vec2,
    pub uv_edges: Vec4, // top, left, bottom, right
    pub linear_color: Vec4,
}

impl Default for TextInstance {
    fn default() -> Self {
        Self {
            size: Vec2::ZERO,
            text_index: 0,
            _padding: 0,
            scale_rotation_x_axis: Vec2::new(1.0, 0.0),
            scale_rotation_y_axis: Vec2::new(0.0, 1.0),
            translation: Vec2::ZERO,
            anchor_offset: Vec2::ZERO,
            uv_edges: Vec4::new(0.0, 0.0, 1.0, 1.0),
            linear_color: Vec4::ONE,
        }
    }
}
