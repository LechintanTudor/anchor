use crate::core::Context;
use crate::graphics::{Color, Drawable, Font, SpriteInstance, Text, TextInstance};
use glam::Vec2;
use glyph_brush::{BrushAction, BrushError, FontId as FontIndex, GlyphBrushBuilder};
use rustc_hash::FxHashMap;

type GlyphBrush = glyph_brush::GlyphBrush<SpriteInstance, Color, Font>;

pub struct TextBatch {
    fonts: FxHashMap<usize, FontIndex>,
    brush: GlyphBrush,
    instances: Vec<SpriteInstance>,
    texture: Option<TextBatchTexture>,
    data: Option<TextBatchData>,
}

impl Default for TextBatch {
    fn default() -> Self {
        Self {
            fonts: Default::default(),
            brush: GlyphBrushBuilder::using_fonts(vec![]).build(),
            instances: Default::default(),
            texture: Default::default(),
            data: Default::default(),
        }
    }
}

impl TextBatch {
    #[inline]
    pub fn begin(&mut self) -> TextDrawer {
        TextDrawer { batch: self }
    }

    fn get_or_insert_font(&mut self, font: &Font) -> FontIndex {
        *self.fonts.entry(font.id()).or_insert_with(|| self.brush.add_font(font.clone()))
    }
}

impl Drawable for TextBatch {
    fn prepare(&mut self, _ctx: &mut Context) {
        let update_texture = |bounds: glyph_brush::Rectangle<u32>, data: &[u8]| {
            let width = bounds.width();

            for i in bounds.min[1]..bounds.max[1] {
                for j in bounds.min[0]..bounds.max[0] {
                    let c = match data[(i * width + j) as usize] {
                        0 => ' ',
                        1..=127 => '#',
                        128..=255 => '|',
                    };

                    print!("{}", c);
                }

                println!();
            }

            println!();
        };

        let into_instance =
            |_instance: glyph_brush::GlyphVertex<Color>| -> SpriteInstance { todo!() };

        match self.brush.process_queued(update_texture, into_instance) {
            Ok(BrushAction::Draw(_vertexes)) => {}
            Ok(BrushAction::ReDraw) => {}
            Err(BrushError::TextureTooSmall { suggested: (_width, _height) }) => {}
        }
    }

    fn draw<'a>(&'a mut self, _ctx: &'a Context, _pass: &mut wgpu::RenderPass<'a>) {}
}

pub struct TextDrawer<'a> {
    batch: &'a mut TextBatch,
}

impl<'a> TextDrawer<'a> {
    pub fn draw(&mut self, text: &Text, position: Vec2) {
        let section = glyph_brush::Section {
            screen_position: position.into(),
            bounds: text.bounds.into(),
            layout: text.layout,
            text: text
                .sections
                .iter()
                .map(|section| glyph_brush::Text {
                    text: &section.content,
                    scale: glyph_brush::ab_glyph::PxScale::from(section.font_size),
                    font_id: self.batch.get_or_insert_font(&section.font),
                    extra: section.color,
                })
                .collect(),
        };

        self.batch.brush.queue(section);
    }
}

struct TextBatchTexture {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
}

struct TextBatchData {
    instances: wgpu::Buffer,
    instances_cap: usize,
}
