use crate::graphics::Canvas;

pub trait Drawable {
    fn draw(self, canvas: &mut Canvas);
}

pub trait AsDrawable {
    type Drawable;

    fn as_drawable(self) -> Self::Drawable;
}

macro_rules! impl_drawable_methods {
    ($Ty:ty) => {
        impl $Ty {
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

            pub fn pixel_anchor<A>(mut self, pixel_anchor: A) -> Self
            where
                A: Into<Vec2>,
            {
                self.anchor_offset = pixel_anchor.into();
                self
            }

            pub fn color(mut self, color: Color) -> Self {
                self.color = color;
                self
            }
        }
    };
}

pub(crate) use impl_drawable_methods;
