use crate::graphics::{Color, Font};
use glam::Vec2;

pub type LineBreaker = glyph_brush::BuiltInLineBreaker;
pub type TextLayout = glyph_brush::Layout<LineBreaker>;
pub type HorizontalAlign = glyph_brush::HorizontalAlign;
pub type VerticalAlign = glyph_brush::VerticalAlign;

#[derive(Clone)]
pub struct Text {
    pub sections: Vec<TextSection>,
    pub layout: TextLayout,
    pub bounds: Vec2,
}

impl Text {
    pub fn simple<S>(content: S, font: Font, font_size: f32, color: Color) -> Self
    where
        S: Into<String>,
    {
        Self {
            sections: vec![TextSection::new(content, font, font_size, color)],
            ..Default::default()
        }
    }

    #[inline]
    pub fn add_section(&mut self, section: TextSection) -> &mut Self {
        self.sections.push(section);
        self
    }

    #[inline]
    pub(crate) fn aligns(&self) -> (HorizontalAlign, VerticalAlign) {
        match self.layout {
            TextLayout::SingleLine { h_align, v_align, .. } => (h_align, v_align),
            TextLayout::Wrap { h_align, v_align, .. } => (h_align, v_align),
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Self {
            sections: Default::default(),
            layout: Default::default(),
            bounds: Vec2::new(f32::INFINITY, f32::INFINITY),
        }
    }
}

#[derive(Clone)]
pub struct TextSection {
    pub content: String,
    pub font: Font,
    pub font_size: f32,
    pub color: Color,
}

impl TextSection {
    pub fn new<S>(content: S, font: Font, font_size: f32, color: Color) -> Self
    where
        S: Into<String>,
    {
        Self { content: content.into(), font, font_size, color }
    }
}
