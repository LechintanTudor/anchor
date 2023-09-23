use crate::graphics::{SharedBindGroupLayouts, WgpuContext};
use glam::Mat4;
use std::ops::Index;
use wgpu::util::DeviceExt;

#[derive(Debug)]
struct ProjectionBindGroup {
    buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

#[derive(Debug)]
pub struct ProjectionBindGroupAllocator {
    bind_group_layouts: SharedBindGroupLayouts,
    bind_groups: Vec<ProjectionBindGroup>,
    used_bind_groups: usize,
}

impl ProjectionBindGroupAllocator {
    pub fn new(bind_group_layouts: SharedBindGroupLayouts) -> Self {
        Self {
            bind_group_layouts,
            bind_groups: Vec::new(),
            used_bind_groups: 0,
        }
    }

    pub fn clear(&mut self) {
        self.used_bind_groups = 0;
    }

    pub fn alloc(&mut self, wgpu: &WgpuContext, projection: &Mat4) -> usize {
        if self.used_bind_groups < self.bind_groups.len() {
            wgpu.queue().write_buffer(
                &self.bind_groups[self.used_bind_groups].buffer,
                0,
                bytemuck::bytes_of(projection),
            );
        } else {
            let buffer = wgpu
                .device()
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("projection_buffer"),
                    contents: bytemuck::bytes_of(projection),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

            let bind_group = wgpu.device().create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("projection_bind_group"),
                layout: self.bind_group_layouts.projection(),
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            });

            self.bind_groups
                .push(ProjectionBindGroup { buffer, bind_group });
        }

        let index = self.used_bind_groups;
        self.used_bind_groups += 1;
        index
    }
}

impl Index<usize> for ProjectionBindGroupAllocator {
    type Output = wgpu::BindGroup;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.used_bind_groups);
        &self.bind_groups[index].bind_group
    }
}
