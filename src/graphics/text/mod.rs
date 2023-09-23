mod font;
mod glyph_data;
mod glyph_texture;
mod text;
mod text_instance;

pub use self::font::*;
pub use self::glyph_data::*;
pub use self::glyph_texture::*;
pub use self::text::*;
pub use self::text_instance::*;

use crate::graphics::{vertex_attr_array, SharedBindGroupLayouts, WgpuContext};
use glam::Vec2;
use glyph_brush::{BrushAction, BrushError, FontId, GlyphBrush, GlyphBrushBuilder};
use rustc_hash::FxHashMap;
use std::mem;
use std::ops::Range;
use wgpu::util::DeviceExt;

const INITIAL_GLYPH_CACHE_SIZE: (u32, u32) = (128, 128);

#[derive(Debug)]
pub struct TextRenderer {
    // Brush
    fonts: FxHashMap<usize, FontId>,
    glyph_brush: GlyphBrush<TextInstance, GlyphData, Font>,
    text_index: u32,

    // Texture
    bind_group_layouts: SharedBindGroupLayouts,
    glyph_texture: GlyphTexture,

    // Pipeline
    pipeline: wgpu::RenderPipeline,
    instance_buffer: Option<wgpu::Buffer>,
    instance_ranges: Vec<Range<u32>>,
}

impl TextRenderer {
    pub fn new(
        wgpu: &WgpuContext,
        bind_group_layouts: SharedBindGroupLayouts,
        format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let glyph_brush = GlyphBrushBuilder::using_fonts(vec![])
            .initial_cache_size(INITIAL_GLYPH_CACHE_SIZE)
            .cache_redraws(false)
            .build();

        let glyph_texture = GlyphTexture::new(wgpu, &bind_group_layouts, INITIAL_GLYPH_CACHE_SIZE);

        let device = wgpu.device();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text_pipeline_layout"),
            bind_group_layouts: &[
                bind_group_layouts.projection(),
                bind_group_layouts.texture(),
                bind_group_layouts.sampler(),
            ],
            push_constant_ranges: &[],
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/text.wgsl").into()),
        });

        let pipeline = Self::create_pipeline(
            device,
            &pipeline_layout,
            &shader_module,
            format,
            sample_count,
        );

        Self {
            fonts: Default::default(),
            glyph_brush,
            text_index: 0,
            bind_group_layouts,
            glyph_texture,
            pipeline,
            instance_buffer: None,
            instance_ranges: Vec::new(),
        }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        pipeline_layout: &wgpu::PipelineLayout,
        shader_module: &wgpu::ShaderModule,
        texture_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text_pipeline"),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<TextInstance>() as _,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &vertex_attr_array!(TextInstance {
                        0 => size: Float32x2,
                        1 => scale_rotation_x_axis: Float32x2,
                        2 => scale_rotation_y_axis: Float32x2,
                        3 => translation: Float32x2,
                        4 => anchor_offset: Float32x2,
                        5 => uv_edges: Float32x4,
                        6 => linear_color: Float32x4,
                    }),
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: sample_count,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: texture_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        })
    }

    pub fn begin(&mut self) {
        self.text_index = 0;
        self.instance_ranges.clear();
    }

    pub fn add(&mut self, text: Text) -> u32 {
        let (layout, anchor_offset) = {
            let (h_align, h_align_scale) = match text.h_align {
                HorizontalAlign::Left => (glyph_brush::HorizontalAlign::Left, 0.0),
                HorizontalAlign::Center => (glyph_brush::HorizontalAlign::Center, 0.5),
                HorizontalAlign::Right => (glyph_brush::HorizontalAlign::Right, 1.0),
            };

            let (v_align, v_align_scale) = match text.v_align {
                VerticalAlign::Top => (glyph_brush::VerticalAlign::Top, 0.0),
                VerticalAlign::Center => (glyph_brush::VerticalAlign::Center, 0.5),
                VerticalAlign::Bottom => (glyph_brush::VerticalAlign::Bottom, 1.0),
            };

            let layout = glyph_brush::Layout::Wrap {
                line_breaker: glyph_brush::BuiltInLineBreaker::UnicodeLineBreaker,
                h_align,
                v_align,
            };

            let anchor_offset =
                -Vec2::new(h_align_scale, v_align_scale) * text.bounds + text.anchor_offset;

            (layout, anchor_offset)
        };

        let text_index = self.text_index;
        let affine2 = text.transform.to_affine2();

        let glyph_brush_texts = text
            .sections
            .iter()
            .map(|section| {
                glyph_brush::Text {
                    text: section.content,
                    scale: section.font_size.unwrap_or(text.font_size).into(),
                    font_id: self.get_or_insert_font(section.font.unwrap_or(text.font)),
                    extra: GlyphData {
                        text_index,
                        affine2,
                        anchor_offset,
                        linear_color: section.color.unwrap_or(text.color).to_linear_vec4(),
                    },
                }
            })
            .collect::<Vec<_>>();

        self.text_index += 1;

        let glyph_brush_section = glyph_brush::Section {
            screen_position: (0.0, 0.0),
            bounds: text.bounds.into(),
            layout,
            text: glyph_brush_texts,
        };

        self.glyph_brush.queue(glyph_brush_section);
        text_index
    }

    pub fn end(&mut self, wgpu: &WgpuContext) {
        let instances = loop {
            match self.glyph_brush.process_queued(
                |bounds, data| {
                    self.glyph_texture.write(
                        wgpu.queue(),
                        bounds.min[0],
                        bounds.min[1],
                        bounds.width(),
                        bounds.height(),
                        data,
                    );
                },
                convert_to_text_instance,
            ) {
                Ok(BrushAction::Draw(instances)) => break instances,
                Ok(BrushAction::ReDraw) => unimplemented!(),
                Err(BrushError::TextureTooSmall { suggested }) => {
                    self.glyph_texture =
                        GlyphTexture::new(wgpu, &self.bind_group_layouts, suggested);
                }
            };
        };

        if instances.is_empty() {
            return;
        }

        let instances_size =
            (instances.len() * mem::size_of::<TextInstance>()) as wgpu::BufferAddress;

        match self.instance_buffer.as_ref() {
            Some(instance_buffer) if instances_size <= instance_buffer.size() => {
                wgpu.queue()
                    .write_buffer(instance_buffer, 0, bytemuck::cast_slice(&instances));
            }
            _ => {
                self.instance_buffer = Some(wgpu.device().create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("text_instance_buffer"),
                        contents: bytemuck::cast_slice(&instances),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    },
                ));
            }
        }

        let text_index_iter = instances
            .iter()
            .map(|instance| instance.text_index)
            .chain(Some(u32::MAX));

        let mut last_text_index = 0;
        let mut range_start = 0;
        let mut range_len = 0;

        text_index_iter.for_each(|text_index| {
            if text_index == last_text_index {
                range_len += 1;
            } else {
                let range_end = range_start + range_len;
                self.instance_ranges.push(range_start..range_end);

                last_text_index = text_index;
                range_start = range_end;
                range_len = 1;
            }
        });
    }

    pub fn prepare_pipeline<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        let Some(instance_buffer) = self.instance_buffer.as_ref() else {
            return;
        };

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, instance_buffer.slice(..));
    }

    pub fn draw<'a>(
        &'a self,
        pass: &mut wgpu::RenderPass<'a>,
        sampler_bind_group: &'a wgpu::BindGroup,
        texts: Range<u32>,
    ) {
        let start_instance = self.instance_ranges[texts.start as usize].start;
        let end_instance = self.instance_ranges[(texts.end - 1) as usize].end;

        pass.set_bind_group(1, self.glyph_texture.bind_group(), &[]);
        pass.set_bind_group(2, sampler_bind_group, &[]);
        pass.draw(0..4, start_instance..end_instance);
    }

    fn get_or_insert_font(&mut self, font: &Font) -> FontId {
        *self
            .fonts
            .entry(font.id())
            .or_insert_with(|| self.glyph_brush.add_font(font.clone()))
    }
}
