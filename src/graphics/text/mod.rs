mod font;
mod glyph_data;
mod glyph_texture;
mod text;

pub use self::font::*;
pub use self::glyph_data::*;
pub use self::glyph_texture::*;
pub use self::text::*;

use crate::graphics::sprite::SpriteInstance;
use crate::graphics::{TextureBindGroupLayout, WgpuContext};
use glyph_brush::{BrushAction, BrushError, FontId, GlyphBrush, GlyphBrushBuilder};
use rustc_hash::FxHashMap;

const INITIAL_GLYPH_CACHE_SIZE: (u32, u32) = (128, 128);

pub struct TextCache {
    wgpu: WgpuContext,
    texture_bind_group_layout: TextureBindGroupLayout,
    fonts: FxHashMap<usize, FontId>,
    glyph_brush: GlyphBrush<SpriteInstance, GlyphData, Font>,
    glyph_texture: GlyphTexture,
}

impl TextCache {
    pub fn new(wgpu: WgpuContext, texture_bind_group_layout: TextureBindGroupLayout) -> Self {
        let glyph_brush = GlyphBrushBuilder::using_fonts(vec![])
            .initial_cache_size(INITIAL_GLYPH_CACHE_SIZE)
            .cache_redraws(false)
            .build();

        let glyph_texture =
            GlyphTexture::new(&wgpu, &texture_bind_group_layout, INITIAL_GLYPH_CACHE_SIZE);

        Self {
            wgpu,
            texture_bind_group_layout,
            fonts: Default::default(),
            glyph_brush,
            glyph_texture,
        }
    }

    pub fn add(&mut self, text: Text) {
        let section = self.convert_to_section(text);
        self.glyph_brush.queue(section);
    }

    pub fn end(&mut self) -> Vec<SpriteInstance> {
        loop {
            match self.glyph_brush.process_queued(
                |bounds, data| {
                    let x = bounds.min[0];
                    let y = bounds.min[1];
                    let w = x + bounds.max[0];
                    let h = y + bounds.max[1];
                    self.glyph_texture.copy(self.wgpu.queue(), x, y, w, h, data);
                },
                convert_to_sprite,
            ) {
                Ok(BrushAction::Draw(instances)) => return instances,
                Ok(BrushAction::ReDraw) => unimplemented!(),
                Err(BrushError::TextureTooSmall { suggested }) => {
                    self.glyph_texture =
                        GlyphTexture::new(&self.wgpu, &self.texture_bind_group_layout, suggested);
                }
            };
        }
    }

    fn convert_to_section<'a>(&mut self, text: Text<'a>) -> glyph_brush::Section<'a, GlyphData> {
        let sections = text
            .sections
            .iter()
            .map(|section| {
                glyph_brush::Text {
                    text: section.content,
                    scale: section.size.unwrap_or(text.size).into(),
                    font_id: self.get_or_insert_font(section.font.unwrap_or(text.font)),
                    extra: GlyphData {
                        affine2: text.transform.to_affine2(),
                        linear_color: text.color.to_linear_vec4(),
                    },
                }
            })
            .collect::<Vec<_>>();

        glyph_brush::Section {
            screen_position: (0.0, 0.0),
            bounds: text.bounds.into(),
            layout: Default::default(),
            text: sections,
        }
    }

    fn get_or_insert_font(&mut self, font: &Font) -> FontId {
        *self
            .fonts
            .entry(font.id())
            .or_insert_with(|| self.glyph_brush.add_font(font.clone()))
    }
}
