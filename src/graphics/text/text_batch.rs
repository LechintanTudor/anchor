use crate::graphics::{self, Color, Drawable, Font, GlyphInstance, Projection, Text};
use crate::platform::Context;
use glam::{Vec2, Vec4};
use glyph_brush::{BrushAction, BrushError, FontId as FontIndex, GlyphBrushBuilder};
use rustc_hash::FxHashMap;
use wgpu::util::DeviceExt;

const INITIAL_DRAW_CACHE_SIZE: u32 = 256;

type Bounds = glyph_brush::Rectangle<u32>;
type GlyphBrush = glyph_brush::GlyphBrush<GlyphInstance, Color, Font>;
type RawGlyphInstance<'a> = glyph_brush::GlyphVertex<'a, Color>;

struct TextBatchData {
    instances: wgpu::Buffer,
    instances_cap: usize,
    instances_len: usize,
    projection: wgpu::Buffer,
    sampler: wgpu::Sampler,
}

struct TextTextureData {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
}

impl TextTextureData {
    fn new(width: u32, height: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        assert!(width != 0 && height != 0);

        let data = vec![0_u8; (width * height) as usize];

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("text_batch_texture"),
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            },
            &data,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, texture_view }
    }

    fn write(&mut self, bounds: Bounds, data: &[u8], queue: &wgpu::Queue) {
        let (offset_x, offset_y, width, height) =
            (bounds.min[0], bounds.min[1], bounds.width(), bounds.height());

        assert!(data.len() == (width * height) as usize);

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: (offset_y * width + offset_x) as u64,
                bytes_per_row: std::num::NonZeroU32::new(width),
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }
}

pub struct TextBatch {
    fonts: FxHashMap<usize, FontIndex>,
    brush: GlyphBrush,
    texture: Option<TextTextureData>,
    data: Option<TextBatchData>,
    bind_group: Option<wgpu::BindGroup>,
}

impl Default for TextBatch {
    fn default() -> Self {
        let brush_builder = GlyphBrushBuilder::using_fonts(vec![])
            .initial_cache_size((INITIAL_DRAW_CACHE_SIZE, INITIAL_DRAW_CACHE_SIZE));

        Self {
            fonts: Default::default(),
            brush: brush_builder.build(),
            texture: Default::default(),
            data: Default::default(),
            bind_group: Default::default(),
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
        let projection = Projection::default().to_mat4(graphics::window_size(ctx));

        loop {
            let mut needs_reprocessing = false;

            let update_texture = |bounds: Bounds, data: &[u8]| {
                let texture = self.texture.get_or_insert_with(|| {
                    // Reset bind group when texture changes
                    self.bind_group = None;

                    TextTextureData::new(
                        INITIAL_DRAW_CACHE_SIZE,
                        INITIAL_DRAW_CACHE_SIZE,
                        &device,
                        &queue,
                    )
                });

                texture.write(bounds, data, &queue);
            };

            match self.brush.process_queued(update_texture, into_glyph_instance) {
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
                                bytemuck::bytes_of(&projection),
                            );
                        }
                        None => {
                            let projection =
                                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                    label: Some("text_batch_projection_buffer"),
                                    contents: bytemuck::bytes_of(&projection),
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
                        queue.write_buffer(&data.projection, 0, bytemuck::bytes_of(&projection));
                    }
                }
                Err(BrushError::TextureTooSmall { suggested: (width, height) }) => {
                    self.brush.resize_texture(width, height);
                    self.texture = Some(TextTextureData::new(width, height, device, queue));
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

        pass.set_pipeline(&ctx.graphics.text_pipeline.pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.set_vertex_buffer(0, data.instances.slice(..instances_size));
        pass.draw(0..6, 0..(data.instances_len as u32));
    }
}

fn into_glyph_instance(instance: RawGlyphInstance) -> GlyphInstance {
    use glyph_brush::ab_glyph::{point, Rect};

    let RawGlyphInstance {
        pixel_coords: glyph_bounds,
        tex_coords,
        extra: color,
        bounds: text_bounds,
    } = instance;

    let min_x_outside_pixels = (text_bounds.min.x - glyph_bounds.min.x).max(0.0);
    let min_y_outside_pixels = (text_bounds.min.y - glyph_bounds.min.y).max(0.0);
    let max_x_outside_pixels = (glyph_bounds.max.x - text_bounds.max.x).max(0.0);
    let max_y_outside_pixels = (glyph_bounds.max.y - text_bounds.max.y).max(0.0);

    let size = Vec2::new(
        glyph_bounds.width() - min_x_outside_pixels - max_x_outside_pixels,
        glyph_bounds.height() - min_y_outside_pixels - max_y_outside_pixels,
    );

    let bounds = Rect {
        min: point(
            glyph_bounds.min.x - min_x_outside_pixels,
            glyph_bounds.min.y - min_y_outside_pixels,
        ),
        max: point(
            glyph_bounds.max.x - max_x_outside_pixels,
            glyph_bounds.max.y - max_y_outside_pixels,
        ),
    };

    let translation = Vec2::new(bounds.min.x, bounds.min.y) + size / 2.0;

    let tex_coords_edges = {
        #[inline]
        fn scale_between(min: f32, value: f32, max: f32) -> f32 {
            (value - min) / (max - min)
        }

        let min_x_scale = scale_between(glyph_bounds.min.x, bounds.min.x, glyph_bounds.max.x);
        let min_y_scale = scale_between(glyph_bounds.min.y, bounds.min.y, glyph_bounds.max.y);
        let max_x_scale = scale_between(glyph_bounds.min.x, bounds.max.x, glyph_bounds.max.x);
        let max_y_scale = scale_between(glyph_bounds.min.y, bounds.max.y, glyph_bounds.max.y);

        #[inline]
        fn from_to_scaled(from: f32, to: f32, scale: f32) -> f32 {
            from + (to - from) * scale
        }

        Vec4::new(
            from_to_scaled(tex_coords.min.y, tex_coords.max.y, min_y_scale),
            from_to_scaled(tex_coords.min.x, tex_coords.max.x, min_x_scale),
            from_to_scaled(tex_coords.min.y, tex_coords.max.y, max_y_scale),
            from_to_scaled(tex_coords.min.x, tex_coords.max.x, max_x_scale),
        )
    };

    let linear_color = color.to_linear_vec4();

    GlyphInstance { size, translation, tex_coords_edges, linear_color }
}

pub struct TextDrawer<'a> {
    batch: &'a mut TextBatch,
}

impl<'a> TextDrawer<'a> {
    pub fn draw(&mut self, text: &Text, position: Vec2) {
        use crate::graphics::{HorizontalAlign, VerticalAlign};

        let position = {
            let (h_align, v_align) = text.aligns();

            let x_anchor = match h_align {
                HorizontalAlign::Center => 0.0,
                HorizontalAlign::Left => 0.5,
                HorizontalAlign::Right => -0.5,
            };

            let y_anchor = match v_align {
                VerticalAlign::Center => 0.0,
                VerticalAlign::Top => 0.5,
                VerticalAlign::Bottom => -0.5,
            };

            (position.x - x_anchor * text.bounds.x, position.y - y_anchor * text.bounds.y)
        };

        let section = glyph_brush::Section {
            screen_position: position,
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

    #[inline]
    pub fn finish(self) -> &'a mut TextBatch {
        self.batch
    }
}
