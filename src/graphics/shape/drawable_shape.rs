use crate::graphics::shape::{Shape, ShapeInstance};
use crate::graphics::Color;
use glam::{Affine2, Vec2};

pub struct DrawableShape<'a> {
    pub shape: &'a Shape,
    pub transform: Affine2,
    pub anchor_offset: Vec2,
    pub color: Color,
}

impl DrawableShape<'_> {
    pub fn with_translation<T>(mut self, translation: T) -> Self
    where
        T: Into<Vec2>,
    {
        self.transform.translation = translation.into();
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn as_shape_instance(&self) -> ShapeInstance {
        ShapeInstance {
            scale_rotation_col_0: self.transform.matrix2.x_axis,
            scale_rotation_col_1: self.transform.matrix2.y_axis,
            translation: self.transform.translation,
            anchor_offset: self.anchor_offset,
            linear_color: self.color.as_linear_vec4(),
        }
    }
}

impl<'a> From<&'a Shape> for DrawableShape<'a> {
    fn from(shape: &'a Shape) -> Self {
        Self { shape, transform: Affine2::IDENTITY, anchor_offset: Vec2::ZERO, color: Color::WHITE }
    }
}
