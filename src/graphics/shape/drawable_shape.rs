use crate::graphics::shape::{Shape, ShapeInstance};
use crate::graphics::{Canvas, Color, Drawable, Transform};
use glam::Vec2;

#[derive(Clone, Debug)]
pub struct DrawableShape<'a> {
    pub shape: &'a Shape,
    pub transform: Transform,
    pub anchor_offset: Vec2,
    pub color: Color,
}

impl DrawableShape<'_> {
    pub fn transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.transform.scale = Vec2::splat(scale);
        self
    }

    pub fn nonuniform_scale<S>(mut self, scale: S) -> Self
    where
        S: Into<Vec2>,
    {
        self.transform.scale = scale.into();
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.transform.rotation = rotation;
        self
    }

    pub fn translation<T>(mut self, translation: T) -> Self
    where
        T: Into<Vec2>,
    {
        self.transform.translation = translation.into();
        self
    }

    pub fn anchor_offset<O>(mut self, offset: O) -> Self
    where
        O: Into<Vec2>,
    {
        self.anchor_offset = offset.into();
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn to_shape_instance(&self) -> ShapeInstance {
        let affine2 = self.transform.to_affine2();

        ShapeInstance {
            scale_rotation_col_0: affine2.matrix2.x_axis,
            scale_rotation_col_1: affine2.matrix2.y_axis,
            translation: affine2.translation,
            anchor_offset: self.anchor_offset,
            linear_color: self.color.to_linear_vec4(),
        }
    }
}

impl Drawable for DrawableShape<'_> {
    fn draw(self, canvas: &mut Canvas) {
        canvas.draw_shape(self.shape, self.to_shape_instance());
    }
}

impl<'a> From<&'a Shape> for DrawableShape<'a> {
    fn from(shape: &'a Shape) -> Self {
        Self {
            shape,
            transform: Transform::IDENTITY,
            anchor_offset: Vec2::ZERO,
            color: Color::WHITE,
        }
    }
}
