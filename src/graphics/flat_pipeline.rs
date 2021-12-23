use std::ops::Deref;
use wgpu::*;

pub struct FlatPipeline {
    pipeline: RenderPipeline,
}

impl FlatPipeline {
    pub fn new(device: &Device, format: TextureFormat) -> anyhow::Result<Self> {
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("flat"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(&ShaderModuleDescriptor {
            label: Some("flat"),
            source: ShaderSource::Wgsl(include_str!("shaders/flat.wgsl").into()),
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("flat"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[ColorTargetState {
                    format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: ColorWrites::ALL,
                }],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                polygon_mode: PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Ok(Self { pipeline })
    }
}

impl Deref for FlatPipeline {
    type Target = RenderPipeline;

    fn deref(&self) -> &Self::Target {
        &self.pipeline
    }
}
