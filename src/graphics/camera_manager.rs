use crate::graphics::WgpuContext;
use glam::Mat4;
use std::ops::Index;
use wgpu::util::DeviceExt;

#[derive(Debug)]
struct ProjectionBindGroup {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

#[derive(Debug)]
pub struct CameraManager {
    wgpu: WgpuContext,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<ProjectionBindGroup>,
    used_bind_groups: usize,
}

impl CameraManager {
    pub fn new(wgpu: WgpuContext) -> Self {
        let projection_bind_group_layout =
            wgpu.device().create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("projection_bind_group_layout"),
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

        Self {
            wgpu,
            bind_group_layout: projection_bind_group_layout,
            bind_groups: Vec::new(),
            used_bind_groups: 0,
        }
    }

    pub fn projection_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn clear(&mut self) {
        self.used_bind_groups = 0;
    }

    pub fn alloc_bind_group(&mut self, projection: &Mat4) -> usize {
        if self.used_bind_groups < self.bind_groups.len() {
            self.wgpu.queue().write_buffer(
                &self.bind_groups[self.used_bind_groups].buffer,
                0,
                bytemuck::bytes_of(projection),
            );
        } else {
            let buffer = self.wgpu.device().create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("projection_buffer"),
                contents: bytemuck::bytes_of(projection),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

            let bind_group = self.wgpu.device().create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("projection_bind_group"),
                layout: &self.bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

            self.bind_groups.push(ProjectionBindGroup { buffer, bind_group });
        }

        let index = self.used_bind_groups;
        self.used_bind_groups += 1;
        index
    }
}

impl Index<usize> for CameraManager {
    type Output = wgpu::BindGroup;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.used_bind_groups);
        &self.bind_groups[index].bind_group
    }
}
