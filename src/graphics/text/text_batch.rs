use crate::graphics::{
    self, Drawable, Font, GlyphInstance, GlyphTexture, GlyphTextureBounds, Projection,
    RawGlyphInstanceData, Text, Transform,
};
use crate::platform::Context;
use glyph_brush::{BrushAction, BrushError, FontId as FontIndex, GlyphBrushBuilder};
use rustc_hash::FxHashMap;
use wgpu::util::DeviceExt;

const INITIAL_DRAW_CACHE_SIZE: u32 = 256;

type GlyphBrush = glyph_brush::GlyphBrush<GlyphInstance, RawGlyphInstanceData, Font>;

struct TextBatchData {
    instances: wgpu::Buffer,
    instances_cap: usize,
    instances_len: usize,
    projection: wgpu::Buffer,
    sampler: wgpu::Sampler,
}

pub struct TextBatch {
    fonts: FxHashMap<usize, FontIndex>,
    brush: GlyphBrush,
    projection: Option<Projection>,
    texture: Option<GlyphTexture>,
    data: Option<TextBatchData>,
    bind_group: Option<wgpu::BindGroup>,
}

impl Default for TextBatch {
    fn default() -> Self {
        let brush_builder = GlyphBrushBuilder::using_fonts(vec![])
            .initial_cache_size((INITIAL_DRAW_CACHE_SIZE, INITIAL_DRAW_CACHE_SIZE))
            .draw_cache_position_tolerance(1.0)
            .cache_glyph_positioning(false);

        Self {
            fonts: Default::default(),
            brush: brush_builder.build(),
            projection: None,
            texture: None,
            data: None,
            bind_group: None,
        }
    }
}

impl TextBatch {
    #[inline]
    pub fn set_projection(&mut self, projection: Option<Projection>) {
        self.projection = projection.into();
    }

    #[inline]
    pub fn begin(&mut self) -> TextDrawer {
        TextDrawer { batch: self }
    }

    fn get_or_insert_font(&mut self, font: &Font) -> FontIndex {
        *self.fonts.entry(font.id()).or_insert_with(|| self.brush.add_font(font.clone()))
    }

    /// Recreates the bind group if necessary.
    fn recreate_bind_group(&mut self, ctx: &Context) {
        if self.bind_group.is_some() {
            return;
        }

        let (texture, data) = match (self.texture.as_mut(), self.data.as_mut()) {
            (Some(texture), Some(data)) => (texture, data),
            _ => return,
        };

        self.bind_group = Some(ctx.graphics.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text_batch_bind_group"),
            layout: &ctx.graphics.text_pipeline.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: data.projection.as_entire_binding() },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&texture.texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&data.sampler),
                },
            ],
        }));
    }
}

impl Drawable for TextBatch {
    fn prepare(&mut self, ctx: &mut Context) {
        let device = &ctx.graphics.device;
        let queue = &ctx.graphics.queue;

        let projection_matrix =
            self.projection.unwrap_or(ctx.graphics.default_projection).to_mat4();

        loop {
            let mut needs_reprocessing = false;

            let update_texture = |bounds: GlyphTextureBounds, data: &[u8]| {
                let texture = self.texture.get_or_insert_with(|| {
                    // Reset bind group when texture changes
                    self.bind_group = None;

                    GlyphTexture::new(
                        INITIAL_DRAW_CACHE_SIZE,
                        INITIAL_DRAW_CACHE_SIZE,
                        device,
                        queue,
                    )
                });

                texture.write(bounds, data, queue);
            };

            match self.brush.process_queued(update_texture, graphics::into_glyph_instance) {
                Ok(BrushAction::Draw(instances)) => {
                    if instances.is_empty() {
                        if let Some(data) = self.data.as_mut() {
                            data.instances_len = 0;
                        }

                        return;
                    }

                    let create_instance_buffer = |instances: &[GlyphInstance]| -> wgpu::Buffer {
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("text_batch_instance_buffer"),
                            contents: bytemuck::cast_slice(instances),
                            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                        })
                    };

                    match self.data.as_mut() {
                        Some(data) => {
                            if instances.len() <= data.instances_cap {
                                queue.write_buffer(
                                    &data.instances,
                                    0,
                                    bytemuck::cast_slice(&instances),
                                );
                            } else {
                                data.instances = create_instance_buffer(&instances);
                                data.instances_cap = instances.len();
                            }

                            data.instances_len = instances.len();

                            queue.write_buffer(
                                &data.projection,
                                0,
                                bytemuck::bytes_of(&projection_matrix),
                            );
                        }
                        None => {
                            let projection =
                                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: Some("text_batch_projection_buffer"),
                                    contents: bytemuck::bytes_of(&projection_matrix),
                                    usage: wgpu::BufferUsages::UNIFORM
                                        | wgpu::BufferUsages::COPY_DST,
                                });

                            let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                                address_mode_u: wgpu::AddressMode::ClampToEdge,
                                address_mode_v: wgpu::AddressMode::ClampToEdge,
                                address_mode_w: wgpu::AddressMode::ClampToEdge,
                                mag_filter: wgpu::FilterMode::Linear,
                                min_filter: wgpu::FilterMode::Linear,
                                ..Default::default()
                            });

                            self.data = Some(TextBatchData {
                                instances: create_instance_buffer(&instances),
                                instances_cap: instances.len(),
                                instances_len: instances.len(),
                                projection,
                                sampler,
                            });
                        }
                    }
                }
                Ok(BrushAction::ReDraw) => {
                    if let Some(data) = self.data.as_mut() {
                        queue.write_buffer(
                            &data.projection,
                            0,
                            bytemuck::bytes_of(&projection_matrix),
                        );
                    }
                }
                Err(BrushError::TextureTooSmall { suggested: (width, height) }) => {
                    self.brush.resize_texture(width, height);
                    self.texture = Some(GlyphTexture::new(width, height, device, queue));
                    needs_reprocessing = true;
                }
            }

            if !needs_reprocessing {
                break;
            }
        }

        self.recreate_bind_group(ctx);
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {
        let (bind_group, data) = match (self.bind_group.as_mut(), self.data.as_mut()) {
            (Some(bind_group), Some(data)) if data.instances_len != 0 => (bind_group, data),
            _ => return,
        };

        let instances_size =
            (std::mem::size_of::<GlyphInstance>() * data.instances_len) as wgpu::BufferAddress;

        let viewport = self.projection.unwrap_or(ctx.graphics.default_projection).viewport;

        pass.set_pipeline(&ctx.graphics.text_pipeline.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.set_vertex_buffer(0, data.instances.slice(..instances_size));
        pass.set_viewport(viewport.x, viewport.y, viewport.w, viewport.h, 0.0, 1.0);
        pass.draw(0..6, 0..(data.instances_len as u32));
    }
}

pub struct TextDrawer<'a> {
    batch: &'a mut TextBatch,
}

impl<'a> TextDrawer<'a> {
    pub fn draw(&mut self, text: &Text, tranform: &Transform) {
        use glyph_brush::{HorizontalAlign, Layout, VerticalAlign};

        let layout = text.layout();

        let position = {
            let (h_align, v_align) = match layout {
                Layout::SingleLine { h_align, v_align, .. } => (h_align, v_align),
                Layout::Wrap { h_align, v_align, .. } => (h_align, v_align),
            };

            let h_align_anchor = match h_align {
                HorizontalAlign::Center => 0.0,
                HorizontalAlign::Left => 0.5,
                HorizontalAlign::Right => -0.5,
            };

            let v_align_anchor = match v_align {
                VerticalAlign::Center => 0.0,
                VerticalAlign::Top => 0.5,
                VerticalAlign::Bottom => -0.5,
            };

            (
                -(h_align_anchor + text.anchor.x) * text.bounds.x,
                -(v_align_anchor + text.anchor.y) * text.bounds.y,
            )
        };

        let section = glyph_brush::Section {
            screen_position: position,
            bounds: text.bounds.into(),
            layout,
            text: text
                .sections
                .iter()
                .map(|section| glyph_brush::Text {
                    text: &section.content,
                    scale: section.font_size.into(),
                    font_id: self.batch.get_or_insert_font(&section.font),
                    extra: RawGlyphInstanceData {
                        affine: tranform.to_affine2(),
                        color: section.color,
                    },
                })
                .collect(),
        };

        self.batch.brush.queue(section);
    }

    #[inline]
    pub fn finish(self) -> &'a mut TextBatch {
        self.batch
    }
}
