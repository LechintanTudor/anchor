use crate::core::Context;
use crate::graphics::{Camera, Drawable, Sprite, SpriteSheet, SpriteVertex, Transform};

struct SpriteBatchData {
    vertexes: wgpu::Buffer,
    vertexes_capacity: usize,
    indexes: wgpu::Buffer,
    indexes_capacity: usize,
}

pub struct SpriteBatch {
    sprite_sheet: SpriteSheet,
    vertexes: Vec<SpriteVertex>,
    indexes: Vec<u32>,
    data: Option<SpriteBatchData>,
    needs_sync: bool,
}

impl SpriteBatch {
    pub fn new(sprite_sheet: SpriteSheet) -> Self {
        Self {
            sprite_sheet,
            vertexes: Vec::new(),
            indexes: Vec::new(),
            data: None,
            needs_sync: false,
        }
    }

    pub fn begin(&mut self) -> SpriteDrawer {
        self.vertexes.clear();
        self.indexes.clear();

        SpriteDrawer { sprite_batch: self }
    }

    pub fn resume(&mut self) -> SpriteDrawer {
        SpriteDrawer { sprite_batch: self }
    }
}

impl Drawable for SpriteBatch {
    fn prepare(&mut self, ctx: &mut Context, camera: &Camera) {
        todo!()
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, render_pass: &mut wgpu::RenderPass<'a>) {
        todo!()
    }
}

pub struct SpriteDrawer<'a> {
    sprite_batch: &'a mut SpriteBatch,
}

impl<'a> SpriteDrawer<'a> {
    pub fn draw(&mut self, sprite: &Sprite, transform: Transform) {
        todo!()
    }

    pub fn finish(self) -> &'a mut SpriteBatch {
        self.sprite_batch
    }
}
