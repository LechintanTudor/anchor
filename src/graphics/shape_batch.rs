use crate::core::Context;
use crate::graphics::{Drawable, Shape, ShapeVertex, Transform};
use glam::Affine2;
use wgpu::util::DeviceExt;

#[derive(Default)]
pub struct ShapeBatch {
    vertexes: Vec<ShapeVertex>,
    indexes: Vec<u32>,
    needs_sync: bool,
    wgpu_data: Option<ShapeBatchWgpuData>,
}

struct ShapeBatchWgpuData {
    vertex_buffer: wgpu::Buffer,
    vertex_buffer_capacity: usize,
    index_buffer: wgpu::Buffer,
    index_buffer_capacity: usize,
}

impl ShapeBatch {
    pub fn clear(&mut self) {
        self.vertexes.clear();
        self.indexes.clear();
    }

    pub fn draw<S>(&mut self, shape: &S, transform: &Transform)
    where
        S: Shape,
    {
        let affine_tranform = Affine2::from_scale_angle_translation(
            transform.scale,
            transform.rotation,
            transform.position,
        );
        let base_index = u32::try_from(self.vertexes.len()).expect("ShapeBatch index overflow");

        let vertexes = shape.vertexes().map(|mut vertex| {
            vertex.position = affine_tranform.transform_point2(vertex.position - transform.offset);
            vertex
        });
        self.vertexes.extend(vertexes);

        let indexes = shape.indexes().map(|index| base_index + index);
        self.indexes.extend(indexes);

        self.needs_sync = true;
    }
}

impl Drawable for ShapeBatch {
    fn prepare(&mut self, ctx: &mut Context) {
        if !self.needs_sync {
            return;
        }

        let device = &mut ctx.graphics.device;
        let queue = &mut ctx.graphics.queue;

        match self.wgpu_data.as_mut() {
            Some(wgpu_data) => {
                if self.vertexes.len() <= wgpu_data.vertex_buffer_capacity {
                    queue.write_buffer(
                        &wgpu_data.vertex_buffer,
                        0,
                        bytemuck::cast_slice(self.vertexes.as_slice()),
                    )
                } else {
                    wgpu_data.vertex_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytemuck::cast_slice(self.vertexes.as_slice()),
                            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                        });
                    wgpu_data.vertex_buffer_capacity = self.vertexes.len();
                }

                if self.indexes.len() <= wgpu_data.index_buffer_capacity {
                    queue.write_buffer(
                        &wgpu_data.index_buffer,
                        0,
                        bytemuck::cast_slice(self.indexes.as_slice()),
                    );
                } else {
                    wgpu_data.index_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: None,
                            contents: bytemuck::cast_slice(self.indexes.as_slice()),
                            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                        });
                    wgpu_data.index_buffer_capacity = self.indexes.len();
                }
            }
            None => {
                self.wgpu_data = Some(ShapeBatchWgpuData {
                    vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(self.vertexes.as_slice()),
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    }),
                    vertex_buffer_capacity: self.vertexes.len(),
                    index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: None,
                        contents: bytemuck::cast_slice(self.indexes.as_slice()),
                        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                    }),
                    index_buffer_capacity: self.indexes.len(),
                });
            }
        }

        self.needs_sync = false;
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, render_pass: &mut wgpu::RenderPass<'a>) {
        let wgpu_data = match self.wgpu_data.as_mut() {
            Some(wgpu_data) if !self.vertexes.is_empty() => wgpu_data,
            _ => return,
        };

        let vertex_buffer_slice_len: wgpu::BufferAddress =
            (self.vertexes.len() * std::mem::size_of::<ShapeVertex>()) as wgpu::BufferAddress;

        let index_buffer_slice_len: wgpu::BufferAddress =
            (self.indexes.len() * std::mem::size_of::<u32>()) as wgpu::BufferAddress;

        render_pass.set_pipeline(&ctx.graphics.shape_pipeline.pipeline);
        render_pass.set_vertex_buffer(0, wgpu_data.vertex_buffer.slice(..vertex_buffer_slice_len));
        render_pass.set_index_buffer(
            wgpu_data.index_buffer.slice(..index_buffer_slice_len),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..self.indexes.len() as u32, 0, 0..1);
    }
}
