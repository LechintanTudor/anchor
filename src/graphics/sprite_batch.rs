use crate::core::Context;
use crate::graphics::{Drawable, Sprite, SpriteSheet, SpriteVertex, Transform};
use bytemuck::{Pod, Zeroable};
use glam::{const_vec2, Vec2, Vec4};
use wgpu::util::DeviceExt;

struct SpriteBatchData {
    vertexes: wgpu::Buffer,
    vertexes_capacity: usize,
    indexes: wgpu::Buffer,
    indexes_capacity: usize,
    camera: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

pub struct SpriteBatch {
    sprite_sheet: SpriteSheet,
    vertexes: Vec<SpriteVertex>,
    indexes: Vec<u32>,
    data: Option<SpriteBatchData>,
    needs_sync: bool,
}

impl SpriteBatch {
    pub fn new(sprite_sheet: SpriteSheet) -> Self {
        Self {
            sprite_sheet,
            vertexes: Vec::new(),
            indexes: Vec::new(),
            data: None,
            needs_sync: false,
        }
    }

    pub fn begin(&mut self) -> SpriteDrawer {
        self.vertexes.clear();
        self.indexes.clear();

        SpriteDrawer { sprite_batch: self }
    }

    #[inline]
    pub fn resume(&mut self) -> SpriteDrawer {
        SpriteDrawer { sprite_batch: self }
    }
}

impl Drawable for SpriteBatch {
    fn prepare(&mut self, ctx: &mut Context) {
        if self.vertexes.is_empty() {
            return;
        }

        let device = &ctx.graphics.device;
        let queue = &ctx.graphics.queue;
        let ortho_matrix = ctx.graphics.window_ortho_matrix();

        let create_vertex_buffer = |vertexes: &[SpriteVertex]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("sprite_batch_vertex_buffer"),
                contents: bytemuck::cast_slice(vertexes),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
        };

        let create_index_buffer = |indexes: &[u32]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("sprite_batch_index_buffer"),
                contents: bytemuck::cast_slice(indexes),
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            })
        };

        match self.data.as_mut() {
            Some(data) => {
                if self.needs_sync {
                    // Vertex buffer
                    if self.vertexes.len() <= data.vertexes_capacity {
                        queue.write_buffer(&data.vertexes, 0, bytemuck::cast_slice(&self.vertexes));
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
                }

                // Camera buffer
                queue.write_buffer(&data.camera, 0, bytemuck::bytes_of(&ortho_matrix));
            }
            None => {
                // Camera
                let camera = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("sprite_batch_camera_buffer"),
                    contents: bytemuck::bytes_of(&ortho_matrix),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

                let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Nearest,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                });

                // Bind group
                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("sprite_batch_bind_group"),
                    layout: &ctx.graphics.sprite_pipeline.bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry { binding: 0, resource: camera.as_entire_binding() },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(
                                self.sprite_sheet.texture().view(),
                            ),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                });

                self.data = Some(SpriteBatchData {
                    vertexes: create_vertex_buffer(&self.vertexes),
                    vertexes_capacity: self.vertexes.len(),
                    indexes: create_index_buffer(&self.indexes),
                    indexes_capacity: self.indexes.len(),
                    camera,
                    bind_group,
                })
            }
        }

        self.needs_sync = false;
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {
        let data = match self.data.as_mut() {
            Some(data) if !self.vertexes.is_empty() => data,
            _ => return,
        };

        let vertexes_slice_len =
            (std::mem::size_of::<SpriteVertex>() * self.vertexes.len()) as wgpu::BufferAddress;

        let indexes_slice_len =
            (std::mem::size_of::<u32>() * self.indexes.len()) as wgpu::BufferAddress;

        pass.set_pipeline(&ctx.graphics.sprite_pipeline.pipeline);
        pass.set_bind_group(0, &data.bind_group, &[]);
        pass.set_vertex_buffer(0, data.vertexes.slice(..vertexes_slice_len));
        pass.set_index_buffer(data.indexes.slice(..indexes_slice_len), wgpu::IndexFormat::Uint32);
        pass.draw_indexed(0..self.indexes.len() as u32, 0, 0..1);
    }
}

#[must_use]
pub struct SpriteDrawer<'a> {
    sprite_batch: &'a mut SpriteBatch,
}

impl<'a> SpriteDrawer<'a> {
    pub fn draw(&mut self, sprite: &Sprite, transform: &Transform) {
        let sprite_bounds =
            self.sprite_batch.sprite_sheet.get_bounds(sprite.index).expect("Invalid sprite index");

        // Compute vertex positions
        let [top_left, bottom_left, bottom_right, top_right] = {
            let transform = transform.to_affine2();
            let anchor = Vec2::new(sprite.anchor.x, -sprite.anchor.y);
            let size = sprite.size.unwrap_or_else(|| {
                Vec2::new(sprite_bounds.width as f32, sprite_bounds.height as f32)
            });

            let transform_point = |corner| transform.transform_point2(size * (corner - anchor));

            [
                transform_point(const_vec2!([-0.5, -0.5])),
                transform_point(const_vec2!([-0.5, 0.5])),
                transform_point(const_vec2!([0.5, 0.5])),
                transform_point(const_vec2!([0.5, -0.5])),
            ]
        };

        // Compute vertex tex coords
        let [tex_top_left, tex_bottom_left, tex_bottom_right, tex_top_right] = {
            let texure_size = Vec2::new(
                self.sprite_batch.sprite_sheet.width() as f32,
                self.sprite_batch.sprite_sheet.height() as f32,
            );

            let (left, right) = {
                let left = sprite_bounds.x as f32;
                let right = (sprite_bounds.x + sprite_bounds.width) as f32;

                if sprite.flip_x {
                    (right, left)
                } else {
                    (left, right)
                }
            };

            let (top, bottom) = {
                let top = sprite_bounds.y as f32;
                let bottom = (sprite_bounds.y + sprite_bounds.height) as f32;

                if sprite.flip_y {
                    (bottom, top)
                } else {
                    (top, bottom)
                }
            };

            [
                Vec2::new(left, top) / texure_size,
                Vec2::new(left, bottom) / texure_size,
                Vec2::new(right, bottom) / texure_size,
                Vec2::new(right, top) / texure_size,
            ]
        };

        // Add indexes
        {
            let base_index = u32::try_from(self.sprite_batch.vertexes.len())
                .expect("Sprite batch index overflow");

            self.sprite_batch.indexes.extend([
                base_index,
                base_index + 1,
                base_index + 3,
                base_index + 3,
                base_index + 1,
                base_index + 2,
            ]);
        }

        // Add vertexes
        {
            let linear_color = sprite.color.to_linear_vec4();

            self.sprite_batch.vertexes.extend([
                SpriteVertex::new(top_left, tex_top_left, linear_color),
                SpriteVertex::new(bottom_left, tex_bottom_left, linear_color),
                SpriteVertex::new(bottom_right, tex_bottom_right, linear_color),
                SpriteVertex::new(top_right, tex_top_right, linear_color),
            ]);
        }

        self.sprite_batch.needs_sync = true;
    }

    #[inline]
    pub fn finish(self) -> &'a mut SpriteBatch {
        self.sprite_batch
    }
}
