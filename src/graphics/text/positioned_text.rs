use crate::graphics::{Color, RawGlyphInstanceData};
use glam::{Affine2, Vec2};
use glyph_brush::{
    BuiltInLineBreaker, FontId, Layout, Section as GlyphBrushSection, Text as GlyphBrushText,
};

pub(crate) struct PositionedText {
    pub position: Vec2,
    pub bounds: Vec2,
    pub layout: Layout<BuiltInLineBreaker>,
    pub sections: Vec<PositionedTextSection>,
}

impl PositionedText {
    pub fn to_glyph_brush_section(&self) -> GlyphBrushSection<'_, RawGlyphInstanceData> {
        let text = self
            .sections
            .iter()
            .map(|section| GlyphBrushText {
                text: &section.content,
                font_id: section.font_id,
                scale: section.font_size.into(),
                extra: RawGlyphInstanceData { affine: section.affine, color: section.color },
            })
            .collect();

        GlyphBrushSection {
            screen_position: self.position.into(),
            bounds: self.bounds.into(),
            layout: self.layout,
            text,
        }
    }
}

pub(crate) struct PositionedTextSection {
    pub content: String,
    pub font_id: FontId,
    pub font_size: f32,
    pub affine: Affine2,
    pub color: Color,
}
