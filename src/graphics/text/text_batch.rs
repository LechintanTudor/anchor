use crate::graphics::positioned_text::{PositionedText, PositionedTextSection};
use crate::graphics::{
    self, BatchStatus, Drawable, FilterMode, Font, GlyphInstance, GlyphTexture, GlyphTextureBounds,
    Projection, RawGlyphInstanceData, Text, Transform,
};
use crate::platform::Context;
use glam::Vec2;
use glyph_brush::{BrushAction, BrushError, FontId, GlyphBrushBuilder};
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
    fonts: FxHashMap<usize, FontId>,
    filter_mode: FilterMode,
    brush: GlyphBrush,
    positioned_texts: Vec<PositionedText>,
    texture: Option<GlyphTexture>,
    bind_group: Option<wgpu::BindGroup>,
    data: Option<TextBatchData>,
    status: BatchStatus,
}

impl TextBatch {
    pub fn new(filter_mode: FilterMode) -> Self {
        let brush_builder = GlyphBrushBuilder::using_fonts(vec![])
            .initial_cache_size((INITIAL_DRAW_CACHE_SIZE, INITIAL_DRAW_CACHE_SIZE))
            .draw_cache_position_tolerance(1.0)
            .cache_glyph_positioning(false);

        Self {
            fonts: FxHashMap::default(),
            filter_mode,
            brush: brush_builder.build(),
            positioned_texts: Vec::new(),
            texture: None,
            bind_group: None,
            data: None,
            status: BatchStatus::Empty,
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.positioned_texts.clear();
        self.status = BatchStatus::Empty;
    }

    pub fn add(&mut self, text: &Text, transform: &Transform) {
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

            Vec2::new(
                -(h_align_anchor + text.anchor.x) * text.bounds.x,
                -(v_align_anchor + text.anchor.y) * text.bounds.y,
            )
        };

        let affine = transform.to_affine2();
        let sections = text
            .sections
            .iter()
            .map(|section| PositionedTextSection {
                content: section.content.clone(),
                font_id: self.get_or_insert_font(&section.font),
                font_size: section.font_size,
                affine,
                color: section.color,
            })
            .collect();

        let positioned_text = PositionedText { position, bounds: text.bounds, layout, sections };
        self.positioned_texts.push(positioned_text);
        self.status = BatchStatus::NonEmpty;
    }

    fn get_or_insert_font(&mut self, font: &Font) -> FontId {
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
    fn prepare(&mut self, ctx: &Context, projection: Projection) {
        if self.status != BatchStatus::NonEmpty {
            return;
        }

        for positioned_text in self.positioned_texts.iter() {
            self.brush.queue(positioned_text.to_glyph_brush_section());
        }

        let projection_matrix = projection.to_ortho_mat4();

        loop {
            let mut needs_reprocessing = false;
            let device = &ctx.graphics.device;
            let queue = &ctx.graphics.queue;

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
                                mag_filter: self.filter_mode.into(),
                                min_filter: self.filter_mode.into(),
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
        self.status = BatchStatus::Ready;
    }

    fn draw<'a>(&'a self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {
        let (bind_group, data) = match (self.bind_group.as_ref(), self.data.as_ref()) {
            (Some(bind_group), Some(data)) if self.status == BatchStatus::Ready => {
                (bind_group, data)
            }
            _ => return,
        };

        let instances_size =
            (std::mem::size_of::<GlyphInstance>() * data.instances_len) as wgpu::BufferAddress;

        pass.set_pipeline(&ctx.graphics.text_pipeline.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.set_vertex_buffer(0, data.instances.slice(..instances_size));
        pass.draw(0..6, 0..(data.instances_len as u32));
    }
}
