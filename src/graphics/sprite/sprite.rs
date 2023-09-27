use crate::graphics::sprite::{SpriteInstance, Texture};
use crate::graphics::{
    impl_drawable_methods, AsDrawable, Bounds, Canvas, Color, Drawable, Transform,
};
use glam::{Vec2, Vec4};

#[derive(Clone, Debug)]
pub struct Sprite<'a> {
    pub texture: &'a Texture,
    pub smooth: bool,
    pub custom_size: Option<Vec2>,
    pub uv_bounds: Bounds,
    pub flip_x: bool,
    pub flip_y: bool,
    pub transform: Transform,
    pub anchor_offset: Vec2,
    pub color: Color,
}

impl_drawable_methods!(Sprite<'_>);

impl<'a> Sprite<'a> {
    pub fn new(texture: &'a Texture) -> Self {
        Self {
            texture,
            smooth: false,
            custom_size: None,
            uv_bounds: Bounds::new(0.0, 0.0, 1.0, 1.0),
            flip_x: false,
            flip_y: false,
            transform: Transform::IDENTITY,
            anchor_offset: Vec2::ZERO,
            color: Color::WHITE,
        }
    }

    pub fn custom_size<S>(mut self, size: S) -> Self
    where
        S: Into<Vec2>,
    {
        self.custom_size = Some(size.into());
        self
    }

    pub fn uv_bounds<B>(mut self, uv_bounds: B) -> Self
    where
        B: Into<Bounds>,
    {
        self.uv_bounds = uv_bounds.into();
        self
    }

    pub fn pixel_uv_bounds<B>(mut self, pixel_uv_bounds: B) -> Self
    where
        B: Into<Bounds>,
    {
        let texture_size = self.texture.size().as_vec2();
        let pixel_uv_bounds = pixel_uv_bounds.into();

        self.uv_bounds = Bounds::new(
            pixel_uv_bounds.x / texture_size.x,
            pixel_uv_bounds.y / texture_size.y,
            pixel_uv_bounds.w / texture_size.x,
            pixel_uv_bounds.h / texture_size.y,
        );

        self
    }

    pub fn anchor_center(mut self) -> Self {
        self.anchor_offset = self.size() * 0.5;
        self
    }

    pub fn relative_anchor<A>(mut self, relative_anchor: A) -> Self
    where
        A: Into<Vec2>,
    {
        self.anchor_offset = self.size() * relative_anchor.into();
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

    fn size(&self) -> Vec2 {
        self.custom_size
            .unwrap_or_else(|| self.texture.size().as_vec2())
    }

    pub fn to_sprite_instance(&self) -> SpriteInstance {
        let affine2 = self.transform.to_affine2();

        let (left, right) = if self.flip_x {
            (self.uv_bounds.x + self.uv_bounds.w, self.uv_bounds.x)
        } else {
            (self.uv_bounds.x, self.uv_bounds.x + self.uv_bounds.w)
        };

        let (top, bottom) = if self.flip_y {
            (self.uv_bounds.y + self.uv_bounds.h, self.uv_bounds.y)
        } else {
            (self.uv_bounds.y, self.uv_bounds.y + self.uv_bounds.h)
        };

        SpriteInstance {
            size: self.size(),
            scale_rotation_x_axis: affine2.matrix2.x_axis,
            scale_rotation_y_axis: affine2.matrix2.y_axis,
            translation: affine2.translation,
            anchor_offset: self.anchor_offset,
            uv_edges: Vec4::new(top, left, bottom, right),
            linear_color: self.color.to_linear_vec4(),
            ..Default::default()
        }
    }
}

impl Drawable for Sprite<'_> {
    fn draw(self, canvas: &mut Canvas) {
        canvas.draw_sprite(self.texture, self.smooth, self.to_sprite_instance());
    }
}

impl<'a> AsDrawable for &'a Texture {
    type Drawable = Sprite<'a>;

    fn as_drawable(self) -> Self::Drawable {
        Sprite::new(self)
    }
}
