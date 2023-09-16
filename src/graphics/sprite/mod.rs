mod sprite;
mod texture;

pub use self::sprite::*;
pub use self::texture::*;
use crate::graphics::{vertex_attr_array, WgpuContext};
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use std::mem;
use std::ops::Range;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SpriteInstance {
    pub size: Vec2,
    pub _padding: Vec2,
    pub scale_rotation_x_axis: Vec2,
    pub scale_rotation_y_axis: Vec2,
    pub translation: Vec2,
    pub anchor_offset: Vec2,
    pub uv_edges: Vec4, // top, left, bottom, right
    pub linear_color: Vec4,
}

impl Default for SpriteInstance {
    fn default() -> Self {
        Self {
            size: Vec2::ZERO,
            _padding: Vec2::ZERO,
            scale_rotation_x_axis: Vec2::new(1.0, 0.0),
            scale_rotation_y_axis: Vec2::new(0.0, 1.0),
            translation: Vec2::ZERO,
            anchor_offset: Vec2::ZERO,
            uv_edges: Vec4::new(0.0, 0.0, 1.0, 1.0),
            linear_color: Vec4::ONE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpriteBatch {
    pub texture: Texture,
    pub smooth: bool,
    pub instances: Range<u32>,
}

#[derive(Debug)]
pub struct SpriteRenderer {
    wgpu: WgpuContext,
    pipeline: wgpu::RenderPipeline,
    nearest_sampler_bind_group: wgpu::BindGroup,
    linear_sampler_bind_group: wgpu::BindGroup,
    instances: Vec<SpriteInstance>,
    instance_buffer: Option<wgpu::Buffer>,
}

impl SpriteRenderer {
    pub fn new(
        wgpu: WgpuContext,
        projection_bind_group_layout: &wgpu::BindGroupLayout,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let device = wgpu.device();

        let sampler_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("sampler_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }],
            });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sprite_pipeline_layout"),
            bind_group_layouts: &[
                projection_bind_group_layout,
                &sampler_bind_group_layout,
                texture_bind_group_layout,
            ],
            push_constant_ranges: &[],
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("sprite_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("sprite_shader.wgsl").into()),
        });

        let pipeline =
            Self::create_pipeline(device, &pipeline_layout, &shader_module, format, sample_count);

        let nearest_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            min_filter: wgpu::FilterMode::Nearest,
            mag_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let nearest_sampler_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("nearest_sampler_bind_group"),
            layout: &sampler_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&nearest_sampler),
            }],
        });

        let linear_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let linear_sampler_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("linear_sampler_bind_group"),
            layout: &sampler_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Sampler(&linear_sampler),
            }],
        });

        Self {
            wgpu,
            pipeline,
            nearest_sampler_bind_group,
            linear_sampler_bind_group,
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

    pub fn end(&mut self) {
        if self.instances.is_empty() {
            return;
        }

        let instances_size =
            (self.instances.len() * mem::size_of::<SpriteInstance>()) as wgpu::BufferAddress;

        match self.instance_buffer.as_ref() {
            Some(instance_buffer) if instances_size <= instance_buffer.size() => {
                self.wgpu.queue().write_buffer(
                    instance_buffer,
                    0,
                    bytemuck::cast_slice(&self.instances),
                );
            }
            _ => {
                self.instance_buffer = Some(self.wgpu.device().create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("sprite_instance_buffer"),
                        contents: bytemuck::cast_slice(&self.instances),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    },
                ));
            }
        }
    }

    pub fn next_batch(&self, texture: Texture, smooth: bool) -> SpriteBatch {
        let instance_count = self.instances.len() as u32;
        SpriteBatch { texture, smooth, instances: instance_count..(instance_count + 1) }
    }

    pub fn prepare_pipeline<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        let Some(instance_buffer) = self.instance_buffer.as_ref() else {
            return;
        };

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, instance_buffer.slice(..));
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, batch: &'a SpriteBatch) {
        let sampler_bind_group = if batch.smooth {
            &self.linear_sampler_bind_group
        } else {
            &self.nearest_sampler_bind_group
        };

        pass.set_bind_group(1, sampler_bind_group, &[]);
        pass.set_bind_group(2, batch.texture.bind_group(), &[]);
        pass.draw(0..4, batch.instances.clone());
    }
}
