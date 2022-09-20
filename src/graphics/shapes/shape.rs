use std::fmt;
use std::sync::Arc;
use wgpu::util::DeviceExt;

use crate::graphics::ShapeVertex;

struct ShapeData {
    vertexes: wgpu::Buffer,
    vertex_count: usize,
    indexes: wgpu::Buffer,
    index_count: usize,
}

#[derive(Clone)]
pub struct Shape(Arc<ShapeData>);

impl fmt::Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Shape")
            .field("vertex_count", &self.0.vertex_count)
            .field("index_count", &self.0.index_count)
            .finish_non_exhaustive()
    }
}

impl Shape {
    pub(crate) unsafe fn new(
        device: &wgpu::Device,
        vertexes: &[ShapeVertex],
        indexes: &[u16],
    ) -> Self {
        Self(Arc::new(ShapeData {
            vertexes: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("shape_vertex_buffer"),
                contents: bytemuck::cast_slice(vertexes),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            vertex_count: vertexes.len(),
            indexes: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("shape_index_buffer"),
                contents: bytemuck::cast_slice(indexes),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: indexes.len(),
        }))
    }

    #[inline]
    pub fn vertexes(&self) -> wgpu::BufferSlice {
        self.0.vertexes.slice(..)
    }

    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.0.vertex_count
    }

    #[inline]
    pub fn indexes(&self) -> wgpu::BufferSlice {
        self.0.indexes.slice(..)
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.0.index_count
    }
}
