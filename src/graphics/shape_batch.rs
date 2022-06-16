use crate::core::Context;
use crate::graphics::{Camera, Drawable, Shape, ShapeVertex, Transform};
use wgpu::util::DeviceExt;

#[derive(Default)]
pub struct ShapeBatch {
    vertexes: Vec<ShapeVertex>,
    indexes: Vec<u32>,
    needs_sync: bool,
    data: Option<ShapeBatchData>,
}

struct ShapeBatchData {
    vertexes: wgpu::Buffer,
    vertexes_capacity: usize,
    indexes: wgpu::Buffer,
    indexes_capacity: usize,
    camera: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl ShapeBatch {
    #[inline]
    pub fn new() -> ShapeBatch {
        Default::default()
    }

    pub fn begin(&mut self) -> ShapeDrawer {
        self.vertexes.clear();
        self.indexes.clear();

        ShapeDrawer { shape_batch: self }
    }

    #[inline]
    pub fn resume(&mut self) -> ShapeDrawer {
        ShapeDrawer { shape_batch: self }
    }
}

impl Drawable for ShapeBatch {
    fn prepare(&mut self, ctx: &mut Context, camera: &Camera) {
        if self.vertexes.is_empty() {
            return;
        }

        let device = &mut ctx.graphics.device;
        let queue = &mut ctx.graphics.queue;

        let create_vertex_buffer = |vertexes: &[ShapeVertex]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("shape_batch_vertex_buffer"),
                contents: bytemuck::cast_slice(vertexes),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
        };

        let create_index_buffer = |indexes: &[u32]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("shape_batch_index_buffer"),
                contents: bytemuck::cast_slice(indexes),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            })
        };

        match self.data.as_mut() {
            Some(data) => {
                if self.needs_sync {
                    // Vertex buffer
                    if self.vertexes.len() <= data.vertexes_capacity {
                        queue.write_buffer(&data.vertexes, 0, bytemuck::cast_slice(&self.vertexes))
                    } else {
                        data.vertexes = create_vertex_buffer(&self.vertexes);
                        data.vertexes_capacity = self.vertexes.len();
                    }

                    // Index buffer
                    if self.indexes.len() <= data.indexes_capacity {
                        queue.write_buffer(&data.indexes, 0, bytemuck::cast_slice(&self.indexes));
                    } else {
                        data.indexes = create_index_buffer(&self.indexes);
                        data.indexes_capacity = self.indexes.len();
                    }

                    self.needs_sync = false;
                }

                let ortho_matrix = camera.to_ortho_matrix();
                queue.write_buffer(&data.camera, 0, bytemuck::bytes_of(&ortho_matrix));
            }
            None => {
                let camera = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::bytes_of(&camera.to_ortho_matrix()),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &ctx.graphics.shape_pipeline.camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera.as_entire_binding(),
                    }],
                });

                self.data = Some(ShapeBatchData {
                    vertexes: create_vertex_buffer(&self.vertexes),
                    vertexes_capacity: self.vertexes.len(),
                    indexes: create_index_buffer(&self.indexes),
                    indexes_capacity: self.indexes.len(),
                    camera,
                    bind_group,
                });
            }
        }
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {
        let data = match self.data.as_mut() {
            Some(data) if !self.vertexes.is_empty() => data,
            _ => return,
        };

        let vertexes_slice_len: wgpu::BufferAddress =
            (self.vertexes.len() * std::mem::size_of::<ShapeVertex>()) as wgpu::BufferAddress;

        let indexes_slice_len: wgpu::BufferAddress =
            (self.indexes.len() * std::mem::size_of::<u32>()) as wgpu::BufferAddress;

        pass.set_pipeline(&ctx.graphics.shape_pipeline.pipeline);
        pass.set_bind_group(0, &data.bind_group, &[]);
        pass.set_vertex_buffer(0, data.vertexes.slice(..vertexes_slice_len));
        pass.set_index_buffer(data.indexes.slice(..indexes_slice_len), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.indexes.len() as u32, 0, 0..1);
    }
}

#[must_use]
pub struct ShapeDrawer<'a> {
    shape_batch: &'a mut ShapeBatch,
}

impl<'a> ShapeDrawer<'a> {
    pub fn draw<S>(&mut self, shape: &S, transform: &Transform)
    where
        S: Shape,
    {
        let base_index =
            u32::try_from(self.shape_batch.vertexes.len()).expect("ShapeBatch index overflow");
        self.shape_batch.indexes.extend(shape.indexes().map(|index| base_index + index));

        let transform = transform.to_affine2();
        self.shape_batch.vertexes.extend(shape.vertexes().map(|mut vertex| {
            vertex.position = transform.transform_point2(vertex.position);
            vertex
        }));

        self.shape_batch.needs_sync = true;
    }

    #[inline]
    pub fn finish(self) -> &'a mut ShapeBatch {
        self.shape_batch
    }
}
