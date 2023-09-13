mod texture;

pub use self::texture::*;
use crate::graphics::{vertex_attr_array, CameraManager, WgpuContext};
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use std::mem;
use std::ops::Range;

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, Pod, Zeroable)]
pub struct TextureVertex {
    pub position: Vec2,
    pub uv_coords: Vec2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct TextureInstance {
    pub linear_color: Vec4,
}

impl Default for TextureInstance {
    fn default() -> Self {
        Self { linear_color: Vec4::ONE }
    }
}

#[derive(Clone, Debug)]
pub struct TextureBatch {
    pub texture: (),
    pub instances: Range<u32>,
}

#[derive(Debug)]
struct TextureRendererData {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
}

#[derive(Debug)]
pub struct TextureRenderer {
    wgpu: WgpuContext,
    pipeline: wgpu::RenderPipeline,
    vertexes: Vec<TextureVertex>,
    indexes: Vec<u32>,
    instances: Vec<TextureInstance>,
    data: Option<TextureRendererData>,
}

impl TextureRenderer {
    pub fn new(
        wgpu: WgpuContext,
        camera_manager: &CameraManager,
        texture_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let device = wgpu.device();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("texture_pipeline_layout"),
            bind_group_layouts: &[camera_manager.projection_bind_group_layout()],
            push_constant_ranges: &[],
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("texture_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("texture_shader.wgsl").into()),
        });

        let pipeline = Self::create_pipeline(
            device,
            &pipeline_layout,
            &shader_module,
            texture_format,
            sample_count,
        );

        Self {
            wgpu,
            pipeline,
            vertexes: Vec::new(),
            indexes: Vec::new(),
            instances: Vec::new(),
            data: None,
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
            label: Some("texture_pipeline"),
            layout: Some(pipeline_layout),
            vertex: wgpu::VertexState {
                module: shader_module,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<TextureVertex>() as _,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &vertex_attr_array!(TextureVertex {
                            0 => position: Float32x2,
                            1 => uv_coords: Float32x2,
                        }),
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<TextureInstance>() as _,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &vertex_attr_array!(TextureInstance {
                            2 => linear_color: Float32x4,
                        }),
                    },
                ],
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
        self.vertexes.clear();
        self.indexes.clear();
        self.instances.clear();
    }

    pub fn add(&mut self) {
        todo!()
    }

    pub fn end(&mut self) {
        todo!()
    }

    pub fn instance_count(&self) -> u32 {
        self.instances.len() as _
    }

    pub fn prepare_pipeline<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        let Some(data) = self.data.as_ref() else {
            return;
        };

        pass.set_vertex_buffer(0, data.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, data.instance_buffer.slice(..));
        pass.set_index_buffer(data.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
    }
}
