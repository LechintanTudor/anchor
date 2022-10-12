use crate::game::Context;
use crate::graphics::{ImageAtlas, PixelBounds, Texture};
use std::sync::Arc;

#[derive(Debug)]
pub struct SpriteSheetBuilder {
    texture: Texture,
    bounds: Vec<PixelBounds>,
}

impl SpriteSheetBuilder {
    pub fn new(texture: Texture) -> Self {
        let texture_bounds = PixelBounds::new(0, 0, texture.width(), texture.height());
        Self { texture, bounds: vec![texture_bounds] }
    }

    #[inline]
    pub fn add_sprite(&mut self, bounds: PixelBounds) -> usize {
        self.bounds.push(bounds);
        self.bounds.len()
    }

    /// Builds the sprite sheet.
    pub fn build(&self) -> SpriteSheet {
        SpriteSheet { texture: self.texture.clone(), bounds: self.bounds.as_slice().into() }
    }
}

/// Maps sprite names to sprite indexes and bounds. Cheap to clone.
#[derive(Clone, Debug)]
pub struct SpriteSheet {
    texture: Texture,
    bounds: Arc<[PixelBounds]>,
}

impl SpriteSheet {
    pub fn from_image_atlas(ctx: &Context, atlas: ImageAtlas) -> Self {
        Self { texture: Texture::from_image(ctx, atlas.image()), bounds: atlas.bounds }
    }

    #[inline]
    pub fn builder(texture: Texture) -> SpriteSheetBuilder {
        SpriteSheetBuilder::new(texture)
    }

    /// Returns the bounds mapped to sprite index.
    #[inline]
    pub fn get_bounds(&self, index: usize) -> Option<&PixelBounds> {
        self.bounds.get(index)
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
