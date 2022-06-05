use crate::core::Context;
use crate::graphics::{Drawable, Shape, ShapeVertex, Transform};
use glam::Vec2;
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
        let shape_vertexes = {
            let prev_vertex_count = self.vertexes.len();
            shape.write(&mut self.vertexes, &mut self.indexes);
            &mut self.vertexes[prev_vertex_count..]
        };

        if transform.rotation == 0.0 {
            for vertex in shape_vertexes {
                vertex.position =
                    (vertex.position - transform.offset) * transform.scale + transform.position;
            }
        } else {
            let sin = transform.rotation.sin();
            let cos = transform.rotation.cos();

            for vertex in shape_vertexes {
                let [unrotated_x, unrotated_y] =
                    ((vertex.position - transform.offset) * transform.scale + transform.position)
                        .to_array();

                vertex.position = Vec2::new(
                    unrotated_x * cos - unrotated_y * sin,
                    unrotated_x * sin + unrotated_y * cos,
                );
            }
        };

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
            (self.vertexes.len() * std::mem::size_of::<ShapeVertex>()).try_into().unwrap();

        let index_buffer_slice_len: wgpu::BufferAddress =
            (self.indexes.len() * std::mem::size_of::<u32>()).try_into().unwrap();

        render_pass.set_pipeline(&ctx.graphics.shape_pipeline.pipeline);
        render_pass.set_vertex_buffer(0, wgpu_data.vertex_buffer.slice(..vertex_buffer_slice_len));
        render_pass.set_index_buffer(
            wgpu_data.index_buffer.slice(..index_buffer_slice_len),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..self.indexes.len() as u32, 0, 0..1);
    }
}
