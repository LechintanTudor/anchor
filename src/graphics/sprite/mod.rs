mod sprite;
mod sprite_instance;
mod texture;

pub use self::sprite::*;
pub use self::sprite_instance::*;
pub use self::texture::*;

use crate::graphics::{vertex_attr_array, SharedBindGroupLayouts, WgpuContext};
use std::mem;
use std::ops::Range;
use wgpu::util::DeviceExt;

#[derive(Clone, Debug)]
pub struct SpriteBatch {
    pub texture: Texture,
    pub smooth: bool,
    pub instances: Range<u32>,
}

#[derive(Debug)]
pub struct SpriteRenderer {
    pipeline: wgpu::RenderPipeline,
    instances: Vec<SpriteInstance>,
    instance_buffer: Option<wgpu::Buffer>,
}

impl SpriteRenderer {
    pub fn new(
        wgpu: &WgpuContext,
        bind_group_layouts: &SharedBindGroupLayouts,
        format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let device = wgpu.device();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sprite_pipeline_layout"),
            bind_group_layouts: &[
                bind_group_layouts.projection(),
                bind_group_layouts.texture(),
                bind_group_layouts.sampler(),
            ],
            push_constant_ranges: &[],
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sprite_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/sprite.wgsl").into()),
        });

        let pipeline = Self::create_pipeline(
            device,
            &pipeline_layout,
            &shader_module,
            format,
            sample_count,
        );

        Self {
            pipeline,
            instances: Vec::new(),
            instance_buffer: None,
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
            label: Some("sprite_pipeline"),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: mem::size_of::<SpriteInstance>() as _,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &vertex_attr_array!(SpriteInstance {
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
        self.instances.clear();
    }

    pub fn add(&mut self, instance: SpriteInstance) {
        self.instances.push(instance);
    }

    pub fn end(&mut self, wgpu: &WgpuContext) {
        if self.instances.is_empty() {
            return;
        }

        let instances_size =
            (self.instances.len() * mem::size_of::<SpriteInstance>()) as wgpu::BufferAddress;

        match self.instance_buffer.as_ref() {
            Some(instance_buffer) if instances_size <= instance_buffer.size() => {
                wgpu.queue().write_buffer(
                    instance_buffer,
                    0,
                    bytemuck::cast_slice(&self.instances),
                );
            }
            _ => {
                self.instance_buffer = Some(wgpu.device().create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("sprite_instance_buffer"),
                        contents: bytemuck::cast_slice(&self.instances),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    },
                ));
            }
        }
    }

    pub fn instance_count(&self) -> u32 {
        self.instances.len() as _
    }

    pub fn next_batch(&self, texture: Texture, smooth: bool) -> SpriteBatch {
        let instance_count = self.instances.len() as u32;

        SpriteBatch {
            texture,
            smooth,
            instances: instance_count..(instance_count + 1),
        }
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
        texture_bind_group: &'a wgpu::BindGroup,
        sampler_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    ) {
        pass.set_bind_group(1, texture_bind_group, &[]);
        pass.set_bind_group(2, sampler_bind_group, &[]);
        pass.draw(0..4, instances);
    }
}
