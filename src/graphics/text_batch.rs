use crate::core::Context;
use crate::graphics::{Color, Drawable, Font, Text, TextSection, Transform};
use glyph_brush::{BrushAction, BrushError, FontId};
use rustc_hash::FxHashMap;

type TextVertex = crate::graphics::SpriteVertex;
type GlyphBrush = glyph_brush::GlyphBrush<TextVertex, Color, Font>;

pub struct TextBatch {
    fonts: FxHashMap<usize, FontId>,
    brush: GlyphBrush,
    vertexes: Vec<TextVertex>,
    indexes: Vec<u32>,
    texture: Option<TextBatchTexture>,
    data: Option<TextBatchData>,
}

impl TextBatch {
    #[inline]
    pub fn begin(&mut self) -> TextDrawer {
        TextDrawer { brush: &mut self.brush }
    }

    fn insert_font(&mut self, font: &Font) -> FontId {
        todo!()
    }
}

impl Drawable for TextBatch {
    fn prepare(&mut self, ctx: &mut Context) {
        match self.brush.process_queued(|rect, tex_data| (), |vertex| todo!()) {
            Ok(BrushAction::Draw(vertexes)) => {}
            Ok(BrushAction::ReDraw) => {}
            Err(BrushError::TextureTooSmall { suggested: (width, height) }) => {}
        }
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {}
}

pub struct TextDrawer<'a> {
    brush: &'a mut GlyphBrush,
}

impl<'a> TextDrawer<'a> {
    pub fn draw(&mut self, text: &Text, transform: &Transform) {
        let section = glyph_brush::Section {
            screen_position: transform.translation.into(),
            bounds: text.bounds.into(),
            layout: text.layout,
            text: text
                .sections
                .iter()
                .map(|section| glyph_brush::Text {
                    text: &section.value,
                    scale: glyph_brush::ab_glyph::PxScale::from(section.font_size),
                    font_id: glyph_brush::FontId::default(),
                    extra: section.color,
                })
                .collect(),
        };
    }
}

struct TextBatchTexture {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
}

struct TextBatchData {
    vertexes: wgpu::Buffer,
    indexes: wgpu::Buffer,
}
