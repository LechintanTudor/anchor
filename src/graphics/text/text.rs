use crate::graphics::{self, Color, Font};
use glam::Vec2;
use glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, VerticalAlign};

/// Text line breaking logic.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TextLineBreaker {
    /// Breaks are inserted after specific unicode characters.
    Unicode,
    /// Breaks can be inserted after ny character.
    AnyChar,
}

/// Text wrap mode.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TextWrap {
    /// Text is displayed on a single line.
    SingleLine,
    /// Text wraps when it goes out of bounds.
    Wrap,
}

/// Alignment of text inside its bounding box.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TextAlign {
    /// Align text to the top left of the bounding box.
    TopLeft,
    /// Align text to the top of the bounding box.
    Top,
    /// Align text to the top right of the bounding box.
    TopRight,
    /// Align text to the center left of the bounding box.
    CenterLeft,
    /// Align text to the center of the bounding box.
    Center,
    /// Align text to the center right of the bounding box.
    CenterRight,
    /// Align text to the bottom left of the bounding box.
    BottomLeft,
    /// Align text to the bottom of the bounding box.
    Bottom,
    /// Align text to the bottom right of the bounding box.
    BottomRight,
}

/// Text that can be drawn to the screen.
#[derive(Clone, Debug)]
pub struct Text {
    /// Text sections that make up the text.
    pub sections: Vec<TextSection>,
    /// Text alignment inside its bounding box.
    pub align: TextAlign,
    /// Text wrap mode.
    pub wrap: TextWrap,
    /// Text line breaking logic.
    pub line_breaker: TextLineBreaker,
    /// Text bounds. Glyphs that go out of bounds are clipped.
    pub bounds: Vec2,
    /// Anchor point for applying transforms.
    pub anchor: Vec2,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            sections: Vec::new(),
            align: TextAlign::TopLeft,
            wrap: TextWrap::SingleLine,
            line_breaker: TextLineBreaker::Unicode,
            bounds: Vec2::splat(f32::MAX),
            anchor: graphics::ANCHOR_TOP_LEFT,
        }
    }
}

impl Text {
    /// Creates text with a single section aligned and anchored to the top left.
    pub fn new<S>(content: S, font: Font, font_size: f32, color: Color) -> Self
    where
        S: Into<String>,
    {
        Self {
            sections: vec![TextSection::new(content.into(), font, font_size, color)],
            align: TextAlign::TopLeft,
            anchor: graphics::ANCHOR_TOP_LEFT,
            ..Default::default()
        }
    }

    /// Creates text with a single centered section aligned and anchored to the center.
    pub fn centered<S>(content: S, font: Font, font_size: f32, color: Color) -> Self
    where
        S: Into<String>,
    {
        Self {
            sections: vec![TextSection::new(content.into(), font, font_size, color)],
            align: TextAlign::Center,
            anchor: graphics::ANCHOR_CENTER,
            ..Default::default()
        }
    }

    #[inline]
    pub(crate) fn layout(&self) -> Layout<BuiltInLineBreaker> {
        let (h_align, v_align) = match self.align {
            TextAlign::TopLeft => (HorizontalAlign::Left, VerticalAlign::Top),
            TextAlign::Top => (HorizontalAlign::Center, VerticalAlign::Top),
            TextAlign::TopRight => (HorizontalAlign::Right, VerticalAlign::Top),
            TextAlign::CenterLeft => (HorizontalAlign::Left, VerticalAlign::Center),
            TextAlign::Center => (HorizontalAlign::Center, VerticalAlign::Center),
            TextAlign::CenterRight => (HorizontalAlign::Right, VerticalAlign::Center),
            TextAlign::BottomLeft => (HorizontalAlign::Left, VerticalAlign::Bottom),
            TextAlign::Bottom => (HorizontalAlign::Center, VerticalAlign::Bottom),
            TextAlign::BottomRight => (HorizontalAlign::Right, VerticalAlign::Bottom),
        };

        let line_breaker = match self.line_breaker {
            TextLineBreaker::Unicode => BuiltInLineBreaker::UnicodeLineBreaker,
            TextLineBreaker::AnyChar => BuiltInLineBreaker::AnyCharLineBreaker,
        };

        match self.wrap {
            TextWrap::SingleLine => Layout::SingleLine { h_align, v_align, line_breaker },
            TextWrap::Wrap => Layout::Wrap { h_align, v_align, line_breaker },
        }
    }
}

/// Section that is part of a [Text].
#[derive(Clone, Debug)]
pub struct TextSection {
    /// Content of the section.
    pub content: String,
    /// Font to use for the section.
    pub font: Font,
    /// Font size in pixels.
    pub font_size: f32,
    /// Color of the section.
    pub color: Color,
}

impl TextSection {
    /// Creates a section with the given properties.
    pub fn new<S>(content: S, font: Font, font_size: f32, color: Color) -> Self
    where
        S: Into<String>,
    {
        Self { content: content.into(), font, font_size, color }
    }
}
