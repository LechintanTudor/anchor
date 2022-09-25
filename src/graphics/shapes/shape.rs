use crate::core::Context;
use crate::graphics::{Color, ShapeVertex};
use glam::Vec2;
use std::fmt;
use std::sync::Arc;
use wgpu::util::DeviceExt;

struct ShapeData {
    vertexes: wgpu::Buffer,
    vertex_count: usize,
    indexes: wgpu::Buffer,
    index_count: usize,
}

/// Handle to shape data stored on the GPU. Cheap to clone.
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
    /// Creates a shape with the given vertexes and indexes.
    /// 
    /// # Safety
    /// The shape must have at least 3 vertexes and the value of each index in the `indexes` array
    /// must be less than the length of the `vertexes` array.
    pub unsafe fn new_unchecked(ctx: &Context, vertexes: &[ShapeVertex], indexes: &[u16]) -> Self {
        Self(Arc::new(ShapeData {
            vertexes: ctx.graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("shape_vertex_buffer"),
                contents: bytemuck::cast_slice(vertexes),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            vertex_count: vertexes.len(),
            indexes: ctx.graphics.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("shape_index_buffer"),
                contents: bytemuck::cast_slice(indexes),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: indexes.len(),
        }))
    }

    /// Creates a triangle with the given vertex positions and color.
    pub fn triangle(ctx: &Context, vertex_positions: [Vec2; 3], color: Color) -> Self {
        let linear_color = color.to_linear_vec4();
        let vertexes = vertex_positions.map(|position| ShapeVertex::new(position, linear_color));

        unsafe { Self::new_unchecked(ctx, &vertexes, &[0, 1, 2]) }
    }

    /// Creates an equilateral triangle with the given side length and color.
    pub fn equilateral_triangle(ctx: &Context, side_length: f32, color: Color) -> Self {
        let height = (f32::sqrt(3.0) / 2.0) * side_length;
        let bottom = height / 3.0;
        let top = bottom - height;
        let half_side_length = side_length / 2.0;

        let vertex_positions = [
            Vec2::new(0.0, top),
            Vec2::new(-half_side_length, bottom),
            Vec2::new(half_side_length, bottom),
        ];

        Self::triangle(ctx, vertex_positions, color)
    }

    /// Creates a quadrilateral with the given vertex positions and color.
    pub fn quadrilateral(ctx: &Context, vertex_positions: [Vec2; 4], color: Color) -> Self {
        let linear_color = color.to_linear_vec4();
        let vertexes = vertex_positions.map(|position| ShapeVertex::new(position, linear_color));

        unsafe { Self::new_unchecked(ctx, &vertexes, &[0, 1, 3, 3, 1, 2]) }
    }

    /// Creates a rectangle with the given dimensions and color.
    pub fn rectangle(ctx: &Context, size: Vec2, color: Color) -> Self {
        let half_size = size * 0.5;

        let vertex_positions = [
            -half_size,
            Vec2::new(-half_size.x, half_size.y),
            half_size,
            Vec2::new(half_size.x, -half_size.y),
        ];

        Self::quadrilateral(ctx, vertex_positions, color)
    }

    /// Creates a square with the given side length and color.
    pub fn square(ctx: &Context, side_length: f32, color: Color) -> Self {
        Self::rectangle(ctx, Vec2::splat(side_length), color)
    }

    /// Returns a buffer slice of the shape vertexes.
    #[inline]
    pub fn vertexes(&self) -> wgpu::BufferSlice {
        self.0.vertexes.slice(..)
    }

    /// Returns the number of vertexes that make up the shape.
    #[inline]
    pub fn vertex_count(&self) -> usize {
        self.0.vertex_count
    }

    /// Returns a buffer slice of the shape indexes.
    #[inline]
    pub fn indexes(&self) -> wgpu::BufferSlice {
        self.0.indexes.slice(..)
    }

    /// Returns the number of indexes that make up the shape.
    #[inline]
    pub fn index_count(&self) -> usize {
        self.0.index_count
    }
}
