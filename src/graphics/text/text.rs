use crate::graphics::text::Font;
use crate::graphics::{impl_drawable_methods, AsDrawable, Canvas, Color, Drawable, Transform};
use glam::Vec2;

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
    pub font_size: f32,
    pub color: Color,
    pub bounds: Vec2,
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
    pub transform: Transform,
    pub anchor_offset: Vec2,
    pub sections: Vec<Section<'a>>,
}

impl_drawable_methods!(Text<'_>);

impl<'a> Text<'a> {
    pub fn new(font: &'a Font) -> Self {
        Self {
            font,
            font_size: 32.0,
            color: Color::WHITE,
            bounds: Vec2::splat(f32::MAX),
            h_align: HorizontalAlign::Left,
            v_align: VerticalAlign::Top,
            transform: Default::default(),
            anchor_offset: Vec2::ZERO,
            sections: Vec::new(),
        }
    }

    pub fn font(mut self, font: &'a Font) -> Self {
        self.font = font;
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn bounds<B>(mut self, bounds: B) -> Self
    where
        B: Into<Vec2>,
    {
        self.bounds = bounds.into();
        self
    }

    pub fn anchor_center(mut self) -> Self {
        self.anchor_offset = self.bounds * 0.5;
        self
    }

    pub fn h_align(mut self, h_align: HorizontalAlign) -> Self {
        self.h_align = h_align;
        self
    }

    pub fn v_align(mut self, v_align: VerticalAlign) -> Self {
        self.v_align = v_align;
        self
    }

    pub fn align(mut self, h_align: HorizontalAlign, v_align: VerticalAlign) -> Self {
        self.h_align = h_align;
        self.v_align = v_align;
        self
    }

    pub fn align_center(mut self) -> Self {
        self.h_align = HorizontalAlign::Center;
        self.v_align = VerticalAlign::Center;
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
    fn draw(self, canvas: &mut Canvas) {
        canvas.draw_text(self)
    }
}

impl<'a> AsDrawable for &'a Font {
    type Drawable = Text<'a>;

    fn as_drawable(self) -> Self::Drawable {
        Text::new(self)
    }
}

#[derive(Clone, Debug)]
pub struct Section<'a> {
    pub content: &'a str,
    pub font: Option<&'a Font>,
    pub font_size: Option<f32>,
    pub color: Option<Color>,
}

impl<'a> Section<'a> {
    pub fn new(content: &'a str) -> Self {
        Self {
            content,
            font: None,
            font_size: None,
            color: None,
        }
    }

    pub fn font(mut self, font: &'a Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = Some(font_size);
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
