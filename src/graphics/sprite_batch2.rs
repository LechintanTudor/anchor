use crate::core::Context;
use crate::graphics::{Drawable, Sprite, SpriteInstance, SpriteSheet, Transform};
use glam::{Vec2, Vec4};

pub struct SpriteBatchData2 {
    instances: wgpu::Buffer,
    camera: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    sprite_sheet_bind_group: wgpu::BindGroup,
}

pub struct SpriteBatch2 {
    sprite_sheet: SpriteSheet,
    instances: Vec<SpriteInstance>,
    data: Option<SpriteBatchData2>,
    needs_sync: bool,
}

impl SpriteBatch2 {
    pub fn new(sprite_sheet: SpriteSheet) -> Self {
        Self { sprite_sheet, instances: Default::default(), data: None, needs_sync: false }
    }

    #[inline]
    pub fn begin(&mut self) -> SpriteDrawer2 {
        self.instances.clear();
        SpriteDrawer2 { batch: self }
    }

    #[inline]
    pub fn resume(&mut self) -> SpriteDrawer2 {
        SpriteDrawer2 { batch: self }
    }
}

impl Drawable for SpriteBatch2 {
    fn prepare(&mut self, ctx: &mut Context) {
        todo!()
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {
        todo!()
    }
}

pub struct SpriteDrawer2<'a> {
    batch: &'a mut SpriteBatch2,
}

impl<'a> SpriteDrawer2<'a> {
    pub fn draw(&mut self, sprite: &Sprite, transform: &Transform) {
        let sprite_sheet_size = Vec2::new(
            self.batch.sprite_sheet.width() as f32,
            self.batch.sprite_sheet.height() as f32,
        );

        let sprite_bounds =
            self.batch.sprite_sheet.get_bounds(sprite.index).expect("Sprite index out of range");

        let size = sprite
            .size
            .unwrap_or_else(|| Vec2::new(sprite_bounds.width as f32, sprite_bounds.height as f32));

        let anchor = Vec2::new(sprite.anchor.x, -sprite.anchor.y);

        let (scale_rotation_col_0, scale_rotation_col_1, translation) = {
            let affine_transform = transform.to_affine2();

            (
                affine_transform.matrix2.col(0),
                affine_transform.matrix2.col(1),
                affine_transform.translation,
            )
        };

        let absolute_tex_coords_edges = {
            let (left, right) = {
                let left = sprite_bounds.x as f32;
                let right = (sprite_bounds.x + sprite_bounds.width) as f32;

                if sprite.flip_x {
                    (right, left)
                } else {
                    (left, right)
                }
            };

            let (top, bottom) = {
                let top = sprite_bounds.y as f32;
                let bottom = (sprite_bounds.y + sprite_bounds.height) as f32;

                if sprite.flip_y {
                    (bottom, top)
                } else {
                    (top, bottom)
                }
            };

            Vec4::new(top, left, bottom, right)
        };

        let linear_color = sprite.color.to_linear_vec4();

        let instance = SpriteInstance {
            sprite_sheet_size,
            size,
            anchor,
            scale_rotation_col_0,
            scale_rotation_col_1,
            translation,
            absolute_tex_coords_edges,
            linear_color,
        };

        self.batch.instances.push(instance);
        self.batch.needs_sync = true;
    }

    #[inline]
    pub fn finish(self) -> &'a mut SpriteBatch2 {
        self.batch
    }
}
