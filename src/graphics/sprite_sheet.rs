use crate::graphics::Texture;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct SpriteBounds {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl SpriteBounds {
    pub const fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self { x, y, width, height }
    }
}

pub struct SpriteSheetBuilder {
    texture: Texture,
    sprite_bounds: Vec<SpriteBounds>,
    sprite_indexes: HashMap<String, usize>,
}

impl SpriteSheetBuilder {
    fn new(texture: Texture) -> Self {
        Self { texture, sprite_bounds: Default::default(), sprite_indexes: Default::default() }
    }

    pub fn add_sprite(&mut self, sprite_name: String, sprite_bounds: SpriteBounds) -> &mut Self {
        match self.sprite_indexes.entry(sprite_name) {
            Entry::Occupied(entry) => {
                let prev_sprite_index = *entry.get();
                self.sprite_bounds[prev_sprite_index] = sprite_bounds;
            }
            Entry::Vacant(entry) => {
                let sprite_index = self.sprite_bounds.len();
                entry.insert(sprite_index);
                self.sprite_bounds.push(sprite_bounds);
            }
        }

        self
    }

    pub fn build(&mut self) -> SpriteSheet {
        let texture = self.texture.clone();

        if !self.sprite_indexes.is_empty() {
            SpriteSheet {
                texture,
                data: Arc::new(SpriteSheetData {
                    sprite_bounds: std::mem::take(&mut self.sprite_bounds),
                    sprite_indexes: std::mem::take(&mut self.sprite_indexes),
                }),
            }
        } else {
            let width = texture.width();
            let height = texture.height();

            SpriteSheet {
                texture,
                data: Arc::new(SpriteSheetData {
                    sprite_bounds: vec![SpriteBounds::new(0, 0, width, height)],
                    sprite_indexes: Default::default(),
                }),
            }
        }
    }
}

struct SpriteSheetData {
    sprite_bounds: Vec<SpriteBounds>,
    sprite_indexes: HashMap<String, usize>,
}

#[derive(Clone)]
pub struct SpriteSheet {
    texture: Texture,
    data: Arc<SpriteSheetData>,
}

impl SpriteSheet {
    #[inline]
    pub fn builder(texture: Texture) -> SpriteSheetBuilder {
        SpriteSheetBuilder::new(texture)
    }

    #[inline]
    pub fn get_index(&self, sprite_name: &str) -> Option<usize> {
        self.data.sprite_indexes.get(sprite_name).copied()
    }

    #[inline]
    pub fn get_bounds(&self, sprite_index: usize) -> Option<&SpriteBounds> {
        self.data.sprite_bounds.get(sprite_index)
    }

    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}
