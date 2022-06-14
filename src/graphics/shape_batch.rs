use crate::core::Context;
use crate::graphics::{Camera, Drawable, Shape, ShapeVertex, Transform};
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
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
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
        let affine_transform = transform.to_affine2();
        let base_index = u32::try_from(self.vertexes.len()).expect("ShapeBatch index overflow");

        let vertexes = shape.vertexes().map(|mut vertex| {
            vertex.position = affine_transform.transform_point2(vertex.position);
            vertex
        });
        self.vertexes.extend(vertexes);

        let indexes = shape.indexes().map(|index| base_index + index);
        self.indexes.extend(indexes);

        self.needs_sync = true;
    }
}

impl Drawable for ShapeBatch {
    fn prepare(&mut self, ctx: &mut Context, camera: &Camera) {
        if self.vertexes.is_empty() {
            return;
        }

        let device = &mut ctx.graphics.device;
        let queue = &mut ctx.graphics.queue;

        match self.wgpu_data.as_mut() {
            Some(wgpu_data) => {
                if self.needs_sync {
                    // Vertex buffer
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

                    // Index buffer
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

                    self.needs_sync = true;
                }

                let ortho_matrix = camera.to_ortho_matrix();
                queue.write_buffer(&wgpu_data.camera_buffer, 0, bytemuck::bytes_of(&ortho_matrix));
            }
            None => {
                // Vertex buffer
                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(self.vertexes.as_slice()),
                    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                });
                let vertex_buffer_capacity = self.vertexes.len();

                // Index buffer
                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(self.indexes.as_slice()),
                    usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                });
                let index_buffer_capacity = self.indexes.len();

                // Camera uniform
                let ortho_matrix = camera.to_ortho_matrix();
                let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::bytes_of(&ortho_matrix),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });
                let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &ctx.graphics.shape_pipeline.camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    }],
                });

                self.wgpu_data = Some(ShapeBatchWgpuData {
                    vertex_buffer,
                    vertex_buffer_capacity,
                    index_buffer,
                    index_buffer_capacity,
                    camera_buffer,
                    camera_bind_group,
                });
            }
        }
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
        render_pass.set_bind_group(0, &wgpu_data.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, wgpu_data.vertex_buffer.slice(..vertex_buffer_slice_len));
        render_pass.set_index_buffer(
            wgpu_data.index_buffer.slice(..index_buffer_slice_len),
            wgpu::IndexFormat::Uint32,
        );
        render_pass.draw_indexed(0..self.indexes.len() as u32, 0, 0..1);
    }
}
