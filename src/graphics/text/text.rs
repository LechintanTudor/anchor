use crate::graphics::text::Font;
use crate::graphics::{AsDrawable, Canvas, Color, Drawable};
use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Section<'a> {
    pub content: &'a str,
    pub font: Option<&'a Font>,
    pub size: Option<f32>,
    pub color: Option<Color>,
}

impl<'a> Section<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            font: None,
            size: None,
            color: None,
        }
    }

    pub fn font(mut self, font: &'a Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
}

impl<'a> From<&'a str> for Section<'a> {
    fn from(content: &'a str) -> Self {
        Self::new(content)
    }
}

pub trait AsSection<'a> {
    fn as_section(self) -> Section<'a>;
}

impl<'a> AsSection<'a> for &'a str {
    fn as_section(self) -> Section<'a> {
        Section::new(self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum HorizontalAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum VerticalAlign {
    #[default]
    Top,
    Center,
    Bottom,
}

#[derive(Clone, Debug)]
pub struct Text<'a> {
    pub font: &'a Font,
    pub size: f32,
    pub color: Color,
    pub bounds: Vec2,
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
    pub sections: Vec<Section<'a>>,
}

impl<'a> Text<'a> {
    pub fn new(font: &'a Font) -> Self {
        Self {
            font,
            size: 32.0,
            color: Color::WHITE,
            bounds: Vec2::splat(f32::INFINITY),
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
            sections: Vec::new(),
        }
    }

    pub fn font(mut self, font: &'a Font) -> Self {
        self.font = font;
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn bounds<B>(mut self, bounds: B) -> Self
    where
        B: Into<Vec2>,
    {
        self.bounds = bounds.into();
        self
    }

    pub fn section<S>(mut self, section: S) -> Self
    where
        S: Into<Section<'a>>,
    {
        self.sections.push(section.into());
        self
    }
}

impl Drawable for Text<'_> {
    fn draw(&self, _canvas: &mut Canvas) {
        todo!()
    }
}

impl<'a> AsDrawable for &'a Font {
    type Drawable = Text<'a>;

    fn as_drawable(self) -> Self::Drawable {
        Text::new(self)
    }
}
