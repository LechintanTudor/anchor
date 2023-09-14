use crate::graphics::sprite::Texture;
use crate::graphics::{impl_drawable_methods, Bounds, Canvas, Color, Drawable, Transform};
use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Sprite<'a> {
    pub texture: &'a Texture,
    pub uv_bounds: Bounds,
    pub flip_x: bool,
    pub flip_y: bool,
    pub transform: Transform,
    pub anchor_offset: Vec2,
    pub color: Color,
}

impl_drawable_methods!(Sprite<'_>);

impl Sprite<'_> {
    pub fn uv_bounds(mut self, uv_bounds: Bounds) -> Self {
        self.uv_bounds = uv_bounds;
        self
    }

    pub fn pixel_uv_bounds(mut self, pixel_uv_bounds: Bounds) -> Self {
        let texture_size = self.texture.size().as_vec2();

        self.uv_bounds = Bounds::new(
            pixel_uv_bounds.x / texture_size.x,
            pixel_uv_bounds.y / texture_size.y,
            pixel_uv_bounds.w / texture_size.x,
            pixel_uv_bounds.h / texture_size.y,
        );

        self
    }

    pub fn flip_x(mut self, flip_x: bool) -> Self {
        self.flip_x = flip_x;
        self
    }

    pub fn flip_y(mut self, flip_y: bool) -> Self {
        self.flip_y = flip_y;
        self
    }
}

impl Drawable for Sprite<'_> {
    fn draw(self, canvas: &mut Canvas) {
        todo!()
    }
}

impl<'a> From<&'a Texture> for Sprite<'a> {
    fn from(texture: &'a Texture) -> Self {
        Self {
            texture,
            uv_bounds: Bounds::new(0.0, 0.0, 1.0, 1.0),
            flip_x: false,
            flip_y: false,
            transform: Transform::IDENTITY,
            anchor_offset: texture.size().as_vec2() / 2.0,
            color: Color::WHITE,
        }
    }
}
