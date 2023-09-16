mod drawable_shape;
mod shape;

pub use self::drawable_shape::*;
pub use self::shape::*;

use crate::graphics::{vertex_attr_array, WgpuContext};
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use std::mem;
use std::ops::Range;
use wgpu::util::DeviceExt;

#[repr(C)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ShapeInstance {
    pub scale_rotation_x_axis: Vec2,
    pub scale_rotation_y_axis: Vec2,
    pub translation: Vec2,
    pub anchor_offset: Vec2,
    pub linear_color: Vec4,
}

impl Default for ShapeInstance {
    fn default() -> Self {
        Self {
            scale_rotation_x_axis: Vec2::new(1.0, 0.0),
            scale_rotation_y_axis: Vec2::new(0.0, 1.0),
            translation: Vec2::ZERO,
            anchor_offset: Vec2::ZERO,
            linear_color: Vec4::ONE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShapeBatch {
    pub shape: Shape,
    pub instances: Range<u32>,
}

#[derive(Debug)]
pub struct ShapeRenderer {
    wgpu: WgpuContext,
    pipeline: wgpu::RenderPipeline,
    instances: Vec<ShapeInstance>,
    instance_buffer: Option<wgpu::Buffer>,
}

impl ShapeRenderer {
    pub fn new(
        wgpu: WgpuContext,
        projection_bind_group_layout: &wgpu::BindGroupLayout,
        texture_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let device = wgpu.device();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("shape_pipeline_layout"),
            bind_group_layouts: &[projection_bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shape_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shape_shader.wgsl").into()),
        });

        let pipeline = Self::create_pipeline(
            device,
            &pipeline_layout,
            &shader_module,
            texture_format,
            sample_count,
        );

        Self { wgpu, pipeline, instances: Vec::new(), instance_buffer: None }
    }

    fn create_pipeline(
        device: &wgpu::Device,
        pipeline_layout: &wgpu::PipelineLayout,
        shader_module: &wgpu::ShaderModule,
        texture_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> wgpu::RenderPipeline {
        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shape_pipeline"),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<ShapeVertex>() as _,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &vertex_attr_array!(ShapeVertex {
                            0 => position: Float32x2,
                            1 => linear_color: Float32x4,
                        }),
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<ShapeInstance>() as _,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &vertex_attr_array!(ShapeInstance {
                            2 => scale_rotation_x_axis: Float32x2,
                            3 => scale_rotation_y_axis: Float32x2,
                            4 => translation: Float32x2,
                            5 => anchor_offset: Float32x2,
                            6 => linear_color: Float32x4,
                        }),
                    },
                ],
                module: shader_module,
                entry_point: "vs_main",
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
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

    pub fn add(&mut self, instance: ShapeInstance) {
        self.instances.push(instance);
    }

    pub fn end(&mut self) {
        if self.instances.is_empty() {
            return;
        }

        let instances_size =
            (self.instances.len() * mem::size_of::<ShapeInstance>()) as wgpu::BufferAddress;

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
                        label: Some("shape_instance_buffer"),
                        contents: bytemuck::cast_slice(&self.instances),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    },
                ));
            }
        }
    }

    pub fn next_batch(&self, shape: Shape) -> ShapeBatch {
        let instance_count = self.instances.len() as u32;
        ShapeBatch { shape, instances: instance_count..(instance_count + 1) }
    }

    pub fn prepare_pipeline<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        let Some(instance_buffer) = self.instance_buffer.as_ref() else {
            return;
        };

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(1, instance_buffer.slice(..));
    }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>, batch: &'a ShapeBatch) {
        pass.set_vertex_buffer(0, batch.shape.vertex_buffer().slice(..));
        pass.set_index_buffer(batch.shape.index_buffer().slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..batch.shape.index_count(), 0, batch.instances.clone());
    }
}
