use crate::graphics::Texture;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct SpriteBounds {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl SpriteBounds {
    #[inline]
    pub const fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Self { x, y, w, h }
    }
}

#[derive(Debug)]
pub struct SpriteSheetBuilder {
    texture: Texture,
    sprites: HashMap<String, Vec<SpriteBounds>>,
}

impl SpriteSheetBuilder {
    #[inline]
    fn new(texture: Texture) -> Self {
        Self { texture, sprites: Default::default() }
    }

    #[inline]
    pub fn add_sprite(&mut self, name: String, bounds: SpriteBounds) -> &mut Self {
        self.sprites.insert(name, vec![bounds]);
        self
    }

    #[inline]
    pub fn add_sprites(&mut self, name: String, bounds: Vec<SpriteBounds>) -> &mut Self {
        self.sprites.insert(name, bounds);
        self
    }

    pub fn build(&mut self) -> SpriteSheet {
        let data = {
            let mut bounds =
                vec![SpriteBounds::new(0, 0, self.texture.width(), self.texture.height())];
            let mut ranges = HashMap::<String, Range<usize>>::new();

            for (sprite_name, sprite_bounds) in self.sprites.drain() {
                let range = bounds.len()..(bounds.len() + sprite_bounds.len());
                bounds.extend(sprite_bounds.iter().copied());
                ranges.insert(sprite_name, range);
            }

            SpriteSheetData { bounds, ranges }
        };

        SpriteSheet { texture: self.texture.clone(), data: Arc::new(data) }
    }
}

#[derive(Debug)]
struct SpriteSheetData {
    bounds: Vec<SpriteBounds>,
    ranges: HashMap<String, Range<usize>>,
}

#[derive(Clone, Debug)]
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
        self.data.ranges.get(sprite_name).map(|range| range.start)
    }

    #[inline]
    pub fn get_range(&self, sprite_name: &str) -> Option<Range<usize>> {
        self.data.ranges.get(sprite_name).cloned()
    }

    #[inline]
    pub fn get_bounds(&self, index: usize) -> Option<&SpriteBounds> {
        self.data.bounds.get(index)
    }

    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.texture.height()
    }
}
