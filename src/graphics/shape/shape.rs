use crate::graphics::WgpuContext;
use bytemuck::{Pod, Zeroable};
use glam::{Vec2, Vec4};
use std::sync::Arc;
use std::{fmt, mem};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ShapeVertex {
    pub position: Vec2,
    pub _padding: [f32; 2],
    pub linear_color: Vec4,
}

impl Default for ShapeVertex {
    fn default() -> Self {
        Self { position: Vec2::ZERO, _padding: [0.0, 0.0], linear_color: Vec4::ONE }
    }
}

impl fmt::Debug for ShapeVertex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ShapeVertex")
            .field("position", &self.position)
            .field("linear_color", &self.linear_color)
            .finish()
    }
}

#[derive(Debug)]
struct ShapeData {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

#[derive(Clone, Debug)]
pub struct Shape(Arc<ShapeData>);

impl Shape {
    pub fn new<W>(wgpu: &W, vertexes: &[ShapeVertex], indexes: &[u16]) -> Self
    where
        W: AsRef<WgpuContext>,
    {
        let device = wgpu.as_ref().device();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shape_vertex_buffer"),
            contents: bytemuck::cast_slice(vertexes),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("shape_index_buffer"),
            contents: bytemuck::cast_slice(indexes),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self(Arc::new(ShapeData { vertex_buffer, index_buffer }))
    }

    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.0.vertex_buffer
    }

    pub fn index_buffer(&self) -> &wgpu::Buffer {
        &self.0.index_buffer
    }

    pub fn index_count(&self) -> u32 {
        (self.0.index_buffer.size() / (mem::size_of::<u16>() as wgpu::BufferAddress)) as u32
    }
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for Shape {
    // Empty
}
