use serde::Deserialize;

use crate::core::{Context, GameResult};
use crate::graphics::Texture;
use std::collections::HashMap;
use std::path::PathBuf;
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

impl From<(u32, u32, u32, u32)> for SpriteBounds {
    #[inline]
    fn from((x, y, w, h): (u32, u32, u32, u32)) -> Self {
        Self::new(x, y, w, h)
    }
}

impl From<[u32; 4]> for SpriteBounds {
    #[inline]
    fn from([x, y, w, h]: [u32; 4]) -> Self {
        Self::new(x, y, w, h)
    }
}

#[derive(Deserialize)]
struct PathSpriteSheetBuilder {
    texture: PathBuf,
    sprites: HashMap<String, (u32, u32, u32, u32)>,
}

#[derive(Debug, Deserialize)]
#[serde(from = "PathSpriteSheetBuilder")]
pub struct SpriteSheetBuilder {
    texture: SpriteSheetBuilderTexture,
    sprites: HashMap<String, SpriteBounds>,
}

impl From<PathSpriteSheetBuilder> for SpriteSheetBuilder {
    fn from(mut builder: PathSpriteSheetBuilder) -> Self {
        Self {
            texture: SpriteSheetBuilderTexture::Path(builder.texture),
            sprites: builder
                .sprites
                .drain()
                .map(|(name, bounds)| (name, SpriteBounds::from(bounds)))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
enum SpriteSheetBuilderTexture {
    Texture(Texture),
    Path(PathBuf),
}

impl SpriteSheetBuilder {
    pub fn from_texture(texture: Texture) -> Self {
        Self { texture: SpriteSheetBuilderTexture::Texture(texture), sprites: Default::default() }
    }

    pub fn from_texture_path<P>(texture_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            texture: SpriteSheetBuilderTexture::Path(texture_path.into()),
            sprites: Default::default(),
        }
    }

    pub fn add_sprite<S>(&mut self, name: S, bounds: SpriteBounds) -> &mut Self
    where
        S: Into<String>,
    {
        self.sprites.insert(name.into(), bounds);
        self
    }

    pub fn build(&mut self, ctx: &Context) -> GameResult<SpriteSheet> {
        let texture = match &self.texture {
            SpriteSheetBuilderTexture::Texture(texture) => texture.clone(),
            SpriteSheetBuilderTexture::Path(path) => Texture::load_from_file(ctx, path)?,
        };

        let mut bounds = vec![SpriteBounds::new(0, 0, texture.width(), texture.height())];
        let mut indexes = HashMap::<String, usize>::new();

        for (sprite_name, sprite_bounds) in self.sprites.drain() {
            indexes.insert(sprite_name, bounds.len());
            bounds.push(sprite_bounds);
        }

        Ok(SpriteSheet { texture, data: Arc::new(SpriteSheetData { bounds, indexes }) })
    }
}

#[derive(Debug)]
struct SpriteSheetData {
    bounds: Vec<SpriteBounds>,
    indexes: HashMap<String, usize>,
}

#[derive(Clone, Debug)]
pub struct SpriteSheet {
    texture: Texture,
    data: Arc<SpriteSheetData>,
}

impl SpriteSheet {
    #[inline]
    pub fn get_index(&self, sprite_name: &str) -> Option<usize> {
        self.data.indexes.get(sprite_name).copied()
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
