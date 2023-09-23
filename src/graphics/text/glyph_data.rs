use crate::graphics::text::TextInstance;
use glam::{Affine2, Vec2, Vec4};
use glyph_brush::GlyphVertex;
use ordered_float::OrderedFloat;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug)]
pub struct GlyphData {
    pub text_index: u32,
    pub affine2: Affine2,
    pub linear_color: Vec4,
}

impl GlyphData {
    fn props_as_ordered_floats(&self) -> [OrderedFloat<f32>; 10] {
        [
            OrderedFloat(self.affine2.x_axis.x),
            OrderedFloat(self.affine2.x_axis.y),
            OrderedFloat(self.affine2.y_axis.x),
            OrderedFloat(self.affine2.y_axis.y),
            OrderedFloat(self.affine2.translation.x),
            OrderedFloat(self.affine2.translation.y),
            OrderedFloat(self.linear_color.x),
            OrderedFloat(self.linear_color.y),
            OrderedFloat(self.linear_color.z),
            OrderedFloat(self.linear_color.w),
        ]
    }
}

impl PartialEq for GlyphData {
    fn eq(&self, other: &Self) -> bool {
        self.text_index == other.text_index
            && self.props_as_ordered_floats() == other.props_as_ordered_floats()
    }
}

impl Eq for GlyphData {
    // Empty
}

impl Hash for GlyphData {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.text_index.hash(state);
        self.props_as_ordered_floats().hash(state);
    }
}

pub fn convert_to_text_instance(vertex: GlyphVertex<GlyphData>) -> TextInstance {
    let position = Vec2::new(vertex.pixel_coords.min.x, vertex.pixel_coords.min.y);

    TextInstance {
        size: Vec2::new(vertex.pixel_coords.width(), vertex.pixel_coords.height()),
        text_index: vertex.extra.text_index,
        scale_rotation_x_axis: vertex.extra.affine2.matrix2.x_axis,
        scale_rotation_y_axis: vertex.extra.affine2.matrix2.y_axis,
        translation: position + vertex.extra.affine2.translation,
        anchor_offset: Vec2::ZERO,
        uv_edges: Vec4::new(
            vertex.tex_coords.min.y,
            vertex.tex_coords.min.x,
            vertex.tex_coords.max.y,
            vertex.tex_coords.max.x,
        ),
        linear_color: vertex.extra.linear_color,
        ..Default::default()
    }
}
