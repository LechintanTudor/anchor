use serde::Deserialize;

use crate::core::{Context, GameResult};
use crate::graphics::Texture;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Sprite texture coords in pixels.
#[derive(Clone, Copy, Debug)]
pub struct SpriteBounds {
    /// Offset from the left side of the texture.
    pub x: u32,
    /// Offset from the top of the texture.
    pub y: u32,
    /// Width of the sprite.
    pub w: u32,
    /// Height of the sprite.
    pub h: u32,
}

impl SpriteBounds {
    /// Creates sprite bounds with the given properties.
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

/// Implements the builder pattern for creating sprite sheets.
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
    /// Creates a sprite sheet builder from the given texture.
    pub fn from_texture(texture: Texture) -> Self {
        Self { texture: SpriteSheetBuilderTexture::Texture(texture), sprites: Default::default() }
    }

    /// Creates a sprite sheet builder from the given texture path.
    pub fn from_texture_path<P>(texture_path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            texture: SpriteSheetBuilderTexture::Path(texture_path.into()),
            sprites: Default::default(),
        }
    }

    /// Adds a named sprite to the sprite sheet.
    pub fn add_sprite<S>(&mut self, name: S, bounds: SpriteBounds) -> &mut Self
    where
        S: Into<String>,
    {
        self.sprites.insert(name.into(), bounds);
        self
    }

    /// Builds the sprite sheet.
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

/// Maps sprite names to sprite indexes and bounds. Cheap to clone.
#[derive(Clone, Debug)]
pub struct SpriteSheet {
    texture: Texture,
    data: Arc<SpriteSheetData>,
}

impl SpriteSheet {
    /// Returns the index mapped to a sprite name.
    #[inline]
    pub fn get_index(&self, sprite_name: &str) -> Option<usize> {
        self.data.indexes.get(sprite_name).copied()
    }

    /// Returns the bounds mapped to sprite index.
    #[inline]
    pub fn get_bounds(&self, index: usize) -> Option<&SpriteBounds> {
        self.data.bounds.get(index)
    }

    /// Returns the sprite sheet texture.
    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.texture
    }

    /// Returns the width of the sprite sheet texture.
    #[inline]
    pub fn width(&self) -> u32 {
        self.texture.width()
    }

    /// Returns the height of the sprite sheet texture.
    #[inline]
    pub fn height(&self) -> u32 {
        self.texture.height()
    }
}
