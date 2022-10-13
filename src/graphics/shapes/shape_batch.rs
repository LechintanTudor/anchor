use crate::game::Context;
use crate::graphics::{BatchStatus, Color, Drawable, Projection, Shape, ShapeInstance, Transform};
use glam::Vec2;
use std::mem;
use wgpu::util::DeviceExt;

struct ShapeBatchData {
    instances: wgpu::Buffer,
    projection: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

/// Draws instances of a shape.
pub struct ShapeBatch {
    shape: Shape,
    instances: Vec<ShapeInstance>,
    data: Option<ShapeBatchData>,
    status: BatchStatus,
}

impl ShapeBatch {
    /// Creates a shape batch with the given `shape`.
    pub fn new(shape: Shape) -> ShapeBatch {
        Self { shape, instances: Vec::new(), data: None, status: BatchStatus::Empty }
    }

    /// Clears the shape batch.
    pub fn clear(&mut self) {
        self.instances.clear();
        self.status = BatchStatus::Empty;
    }

    /// Adds a shape with the given params and transform.
    pub fn add(&mut self, shape_params: &ShapeParams, transform: &Transform) {
        let affine = transform.to_affine2();

        self.instances.push(ShapeInstance {
            scale_rotation_col_0: affine.matrix2.col(0),
            scale_rotation_col_1: affine.matrix2.col(1),
            translation: affine.translation,
            pixel_anchor: shape_params.pixel_anchor,
            linear_color: shape_params.color.to_linear_vec4(),
        });

        self.status = BatchStatus::NonEmpty;
    }

    /// Sets the shape to use when drawing.
    #[inline]
    pub fn set_shape(&mut self, shape: Shape) {
        self.shape = shape;
    }

    /// Return the shape drawn by the shape batch.
    #[inline]
    pub fn shape(&self) -> &Shape {
        &self.shape
    }
}

impl Drawable for ShapeBatch {
    fn prepare(&mut self, ctx: &Context, projection: Projection) {
        if self.status != BatchStatus::NonEmpty {
            return;
        }

        let projection_matrix = projection.to_ortho_mat4();

        let device = &ctx.graphics.device;
        let queue = &ctx.graphics.queue;

        let create_instance_buffer = |instances: &[ShapeInstance]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("shape_batch_instance_buffer"),
                contents: bytemuck::cast_slice(instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
        };

        match self.data.as_mut() {
            Some(data) => {
                let instances_cap =
                    (data.instances.size() as usize) / std::mem::size_of::<ShapeInstance>();

                if self.instances.len() <= instances_cap {
                    queue.write_buffer(&data.instances, 0, bytemuck::cast_slice(&self.instances))
                } else {
                    data.instances = create_instance_buffer(&self.instances);
                }

                queue.write_buffer(&data.projection, 0, bytemuck::bytes_of(&projection_matrix));
            }
            None => {
                let projection = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("shape_batch_projection_buffer"),
                    contents: bytemuck::bytes_of(&projection_matrix),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("shape_batch_bind_group"),
                    layout: &ctx.graphics.shape_pipeline.bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: projection.as_entire_binding(),
                    }],
                });

                self.data = Some(ShapeBatchData {
                    instances: create_instance_buffer(&self.instances),
                    projection,
                    bind_group,
                });
            }
        }

        self.status = BatchStatus::Ready;
    }

    fn draw<'a>(&'a self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {
        let data = match self.data.as_ref() {
            Some(data) if self.status == BatchStatus::Ready => data,
            _ => return,
        };

        let instance_slice_len =
            (self.instances.len() * mem::size_of::<ShapeInstance>()) as wgpu::BufferAddress;

        pass.set_pipeline(&ctx.graphics.shape_pipeline.pipeline);
        pass.set_bind_group(0, &data.bind_group, &[]);
        pass.set_vertex_buffer(0, self.shape.vertexes());
        pass.set_index_buffer(self.shape.indexes(), wgpu::IndexFormat::Uint16);
        pass.set_vertex_buffer(1, data.instances.slice(..instance_slice_len));
        pass.draw_indexed(0..self.shape.index_count() as u32, 0, 0..self.instances.len() as u32);
    }
}

/// Params used to alter shape drawing.
#[derive(Clone, Copy, Debug)]
pub struct ShapeParams {
    /// Anchor offset in pixels.
    pub pixel_anchor: Vec2,
    /// Color used to tint the shape.
    pub color: Color,
}

impl ShapeParams {
    /// Creates shape params with the given `color`.
    #[inline]
    pub fn from_color(color: Color) -> Self {
        Self { color, ..Default::default() }
    }
}

impl Default for ShapeParams {
    #[inline]
    fn default() -> Self {
        Self { pixel_anchor: Vec2::ZERO, color: Color::WHITE }
    }
}
