use crate::core::Context;
use crate::graphics::{Drawable, Sprite, SpriteInstance, SpriteSheet, Transform};
use glam::{Vec2, Vec4};
use wgpu::util::DeviceExt;

struct SpriteBatchData {
    instances: wgpu::Buffer,
    instances_capacity: usize,
    camera: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

pub struct SpriteBatch {
    sprite_sheet: SpriteSheet,
    instances: Vec<SpriteInstance>,
    data: Option<SpriteBatchData>,
    needs_sync: bool,
}

impl SpriteBatch {
    pub fn new(sprite_sheet: SpriteSheet) -> Self {
        Self { sprite_sheet, instances: Default::default(), data: None, needs_sync: false }
    }

    #[inline]
    pub fn begin(&mut self) -> SpriteDrawer2 {
        self.instances.clear();
        SpriteDrawer2 { batch: self }
    }

    #[inline]
    pub fn resume(&mut self) -> SpriteDrawer2 {
        SpriteDrawer2 { batch: self }
    }
}

impl Drawable for SpriteBatch {
    fn prepare(&mut self, ctx: &mut Context) {
        if self.instances.is_empty() {
            return;
        }

        let device = &ctx.graphics.device;
        let queue = &ctx.graphics.queue;
        let ortho_matrix = ctx.graphics.window_ortho_matrix();

        let create_instance_buffer = |instances: &[SpriteInstance]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("sprite_batch_instance_buffer"),
                contents: bytemuck::cast_slice(instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
        };

        match self.data.as_mut() {
            Some(data) => {
                if self.needs_sync {
                    if self.instances.len() <= data.instances_capacity {
                        queue.write_buffer(
                            &data.instances,
                            0,
                            bytemuck::cast_slice(&self.instances),
                        );
                    } else {
                        data.instances = create_instance_buffer(&self.instances);
                        data.instances_capacity = self.instances.len();
                    }

                    self.needs_sync = false;
                }

                queue.write_buffer(&data.camera, 0, bytemuck::bytes_of(&ortho_matrix));
            }
            None => {
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
                    instances: create_instance_buffer(&self.instances),
                    instances_capacity: self.instances.len(),
                    camera,
                    bind_group,
                });
            }
        }
    }

    fn draw<'a>(&'a mut self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {
        let data = match self.data.as_mut() {
            Some(data) if !self.instances.is_empty() => data,
            _ => return,
        };

        let instances_size =
            (std::mem::size_of::<SpriteInstance>() * self.instances.len()) as wgpu::BufferAddress;

        pass.set_pipeline(&ctx.graphics.sprite_pipeline.pipeline);
        pass.set_bind_group(0, &data.bind_group, &[]);
        pass.set_vertex_buffer(0, data.instances.slice(..instances_size));
        pass.draw(0..6, 0..(self.instances.len() as u32));
    }
}

pub struct SpriteDrawer2<'a> {
    batch: &'a mut SpriteBatch,
}

impl<'a> SpriteDrawer2<'a> {
    pub fn draw(&mut self, sprite: &Sprite, transform: &Transform) {
        let sprite_sheet_size = Vec2::new(
            self.batch.sprite_sheet.width() as f32,
            self.batch.sprite_sheet.height() as f32,
        );

        let sprite_bounds =
            self.batch.sprite_sheet.get_bounds(sprite.index).expect("Sprite index out of range");

        let size = sprite
            .size
            .unwrap_or_else(|| Vec2::new(sprite_bounds.width as f32, sprite_bounds.height as f32));

        let anchor = Vec2::new(sprite.anchor.x, -sprite.anchor.y);

        let (scale_rotation_col_0, scale_rotation_col_1, translation) = {
            let affine_transform = transform.to_affine2();

            (
                affine_transform.matrix2.col(0),
                affine_transform.matrix2.col(1),
                affine_transform.translation,
            )
        };

        let absolute_tex_coords_edges = {
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

            Vec4::new(top, left, bottom, right)
        };

        let linear_color = sprite.color.to_linear_vec4();

        let instance = SpriteInstance {
            sprite_sheet_size,
            size,
            anchor,
            scale_rotation_col_0,
            scale_rotation_col_1,
            translation,
            absolute_tex_coords_edges,
            linear_color,
        };

        self.batch.instances.push(instance);
        self.batch.needs_sync = true;
    }

    #[inline]
    pub fn finish(self) -> &'a mut SpriteBatch {
        self.batch
    }
}
