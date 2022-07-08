use crate::core::Context;
use crate::graphics;
use crate::graphics::{Drawable, Projection, Shape, ShapeVertex, Transform};
use wgpu::util::DeviceExt;

#[derive(Default)]
pub struct ShapeBatch {
    vertexes: Vec<ShapeVertex>,
    indexes: Vec<u32>,
    projection: Projection,
    data: Option<ShapeBatchData>,
    needs_sync: bool,
}

struct ShapeBatchData {
    vertexes: wgpu::Buffer,
    vertexes_capacity: usize,
    indexes: wgpu::Buffer,
    indexes_capacity: usize,
    projection: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl ShapeBatch {
    #[inline]
    pub fn new() -> ShapeBatch {
        Default::default()
    }

    pub fn set_projection<P>(&mut self, projection: P)
    where
        P: Into<Projection>,
    {
        self.projection = projection.into();
    }

    pub fn begin(&mut self) -> ShapeDrawer {
        self.vertexes.clear();
        self.indexes.clear();

        ShapeDrawer { batch: self }
    }

    #[inline]
    pub fn resume(&mut self) -> ShapeDrawer {
        ShapeDrawer { batch: self }
    }
}

impl Drawable for ShapeBatch {
    fn prepare(&mut self, ctx: &mut Context) {
        if self.vertexes.is_empty() {
            return;
        }

        let device = &ctx.graphics.device;
        let queue = &ctx.graphics.queue;

        let projection = self.projection.to_mat4(graphics::window_size(ctx));

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
                    if self.vertexes.len() <= data.vertexes_capacity {
                        queue.write_buffer(&data.vertexes, 0, bytemuck::cast_slice(&self.vertexes))
                    } else {
                        data.vertexes = create_vertex_buffer(&self.vertexes);
                        data.vertexes_capacity = self.vertexes.len();
                    }

                    if self.indexes.len() <= data.indexes_capacity {
                        queue.write_buffer(&data.indexes, 0, bytemuck::cast_slice(&self.indexes));
                    } else {
                        data.indexes = create_index_buffer(&self.indexes);
                        data.indexes_capacity = self.indexes.len();
                    }

                    self.needs_sync = false;
                }

                queue.write_buffer(&data.projection, 0, bytemuck::bytes_of(&projection));
            }
            None => {
                let projection = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("shape_batch_projection_buffer"),
                    contents: bytemuck::bytes_of(&projection),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("shape_batch_bind_group"),
                    layout: &ctx.graphics.shape_pipeline.camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: projection.as_entire_binding(),
                    }],
                });

                self.data = Some(ShapeBatchData {
                    vertexes: create_vertex_buffer(&self.vertexes),
                    vertexes_capacity: self.vertexes.len(),
                    indexes: create_index_buffer(&self.indexes),
                    indexes_capacity: self.indexes.len(),
                    projection,
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

pub struct ShapeDrawer<'a> {
    batch: &'a mut ShapeBatch,
}

impl<'a> ShapeDrawer<'a> {
    pub fn draw<S>(&mut self, shape: &S, transform: &Transform)
    where
        S: Shape,
    {
        let base_index =
            u32::try_from(self.batch.vertexes.len()).expect("ShapeBatch index overflow");
        self.batch.indexes.extend(shape.indexes().map(|index| base_index + index));

        let transform = transform.to_affine2();
        self.batch.vertexes.extend(shape.vertexes().map(|mut vertex| {
            vertex.position = transform.transform_point2(vertex.position);
            vertex
        }));

        self.batch.needs_sync = true;
    }

    #[inline]
    pub fn finish(self) -> &'a mut ShapeBatch {
        self.batch
    }
}
