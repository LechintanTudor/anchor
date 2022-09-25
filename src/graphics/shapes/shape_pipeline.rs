use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use std::mem;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub(crate) struct ShapeInstance {
    pub scale_rotation_col_0: Vec2,
    pub scale_rotation_col_1: Vec2,
    pub translation: Vec2,
    pub pixel_anchor: Vec2,
    pub linear_color: Vec4,
}

/// Vertex used for building a drawable shape.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ShapeVertex {
    /// Vertex position.
    pub position: Vec2,
    _padding: Vec2,
    /// Vertex color expressed as a linear RGBA vector.
    pub linear_color: Vec4,
}

impl ShapeVertex {
    /// Creates a shape vertex.
    #[inline]
    pub fn new(position: Vec2, linear_color: Vec4) -> Self {
        Self { position, _padding: Vec2::ZERO, linear_color }
    }
}

pub(crate) struct ShapePipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pipeline_layout: wgpu::PipelineLayout,
    shader_module: wgpu::ShaderModule,
}

impl ShapePipeline {
    pub fn new(
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        sample_count: u32,
    ) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shape_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("shape_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shape_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shape.wgsl").into()),
        });

        let pipeline = Self::create_pipeline(
            device,
            &pipeline_layout,
            &shader_module,
            texture_format,
            sample_count,
        );

        Self { pipeline, bind_group_layout, pipeline_layout, shader_module }
    }

    pub fn recreate_pipeline(
        &mut self,
        device: &wgpu::Device,
        texture_format: wgpu::TextureFormat,
        sample_count: u32,
    ) {
        self.pipeline = Self::create_pipeline(
            device,
            &self.pipeline_layout,
            &self.shader_module,
            texture_format,
            sample_count,
        );
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
                module: shader_module,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<ShapeVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[
                            // position
                            wgpu::VertexAttribute {
                                shader_location: 0,
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 0,
                            },
                            // linear_color
                            wgpu::VertexAttribute {
                                shader_location: 1,
                                format: wgpu::VertexFormat::Float32x4,
                                offset: 16,
                            },
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: mem::size_of::<ShapeInstance>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![
                            2 => Float32x2, // scale_rotation_col_0
                            3 => Float32x2, // scale_rotation_col_1
                            4 => Float32x2, // translation
                            5 => Float32x2, // pixel_anchor
                            6 => Float32x4, // linear_color
                        ],
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
            multisample: wgpu::MultisampleState { count: sample_count, ..Default::default() },
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
}
