use crate::graphics::{Color, Font};
use glam::Vec2;

pub use glyph_brush::{BuiltInLineBreaker as LineBreaker, HorizontalAlign, VerticalAlign};
pub type TextLayout = glyph_brush::Layout<LineBreaker>;

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
