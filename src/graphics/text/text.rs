use crate::graphics::{self, Color, Font};
use glam::Vec2;
use glyph_brush::{BuiltInLineBreaker, HorizontalAlign, Layout, VerticalAlign};

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TextLineBreaker {
    Unicode,
    AnyChar,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TextWrap {
    SingleLine,
    Wrap,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TextAlign {
    TopLeft,
    Top,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Clone, Debug)]
pub struct Text {
    pub sections: Vec<TextSection>,
    pub align: TextAlign,
    pub wrap: TextWrap,
    pub line_breaker: TextLineBreaker,
    pub bounds: Vec2,
    pub anchor: Vec2,
}

impl Text {
    pub const EMPTY: Self = Self {
        sections: Vec::new(),
        align: TextAlign::TopLeft,
        wrap: TextWrap::SingleLine,
        line_breaker: TextLineBreaker::Unicode,
        bounds: Vec2::splat(f32::MAX),
        anchor: graphics::ANCHOR_TOP_LEFT,
    };

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

impl Default for Text {
    fn default() -> Self {
        Self::EMPTY
    }
}

#[derive(Clone, Debug)]
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
