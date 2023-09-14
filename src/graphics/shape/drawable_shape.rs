use crate::graphics::shape::{Shape, ShapeInstance};
use crate::graphics::{impl_drawable_methods, Canvas, Color, Drawable, Transform};
use glam::Vec2;

#[derive(Clone, Debug)]
pub struct DrawableShape<'a> {
    pub shape: &'a Shape,
    pub transform: Transform,
    pub anchor_offset: Vec2,
    pub color: Color,
}

impl_drawable_methods!(DrawableShape<'_>);

impl DrawableShape<'_> {
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
