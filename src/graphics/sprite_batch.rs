use wgpu::util::DeviceExt;

use crate::core::Context;
use crate::graphics::{Camera, Drawable, Sprite, SpriteSheet, SpriteVertex, Transform, Vec2};

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

    pub fn resume(&mut self) -> SpriteDrawer {
        SpriteDrawer { sprite_batch: self }
    }
}

impl Drawable for SpriteBatch {
    fn prepare(&mut self, ctx: &mut Context, camera: &Camera) {
        if self.vertexes.is_empty() {
            return;
        }

        let device = &ctx.graphics.device;
        let queue = &ctx.graphics.queue;

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

                    self.needs_sync = false;
                }

                // Camera buffer
                let ortho_matrix = camera.to_ortho_matrix();
                queue.write_buffer(&data.camera, 0, bytemuck::bytes_of(&ortho_matrix));
            }
            None => {
                // Camera
                let ortho_matrix = camera.to_ortho_matrix();
                let camera = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("sprite_batch_camera_buffer"),
                    contents: bytemuck::bytes_of(&ortho_matrix),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("sprite_batch_bind_group"),
                    layout: &ctx.graphics.sprite_pipeline.bind_group_layout,
                    entries: todo!(),
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
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, render_pass: &mut wgpu::RenderPass<'a>) {
        todo!()
    }
}

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
            let size = sprite.size.unwrap_or_else(|| {
                Vec2::new(sprite_bounds.width as f32, sprite_bounds.height as f32)
            });
            let offset = sprite.anchor * size;

            let transform_point = |anchor| {
                transform.transform_point2(transform.translation + size * anchor + offset) - offset
            };

            [
                transform_point(Sprite::ANCHOR_TOP_LEFT),
                transform_point(Sprite::ANCHOR_BOTTOM_LEFT),
                transform_point(Sprite::ANCHOR_BOTTOM_RIGHT),
                transform_point(Sprite::ANCHOR_TOP_RIGHT),
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
                base_index + 2,
                base_index + 2,
                base_index + 1,
                base_index + 3,
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

    pub fn finish(self) -> &'a mut SpriteBatch {
        self.sprite_batch
    }
}
