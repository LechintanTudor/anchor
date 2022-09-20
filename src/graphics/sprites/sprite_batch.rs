use crate::graphics::{
    BatchStatus, Drawable, FilterMode, Projection, Sprite, SpriteInstance, SpriteSheet, Transform,
};
use crate::platform::Context;
use glam::{Vec2, Vec4};
use wgpu::util::DeviceExt;

struct SpriteBatchData {
    instances: wgpu::Buffer,
    instances_capacity: usize,
    projection: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

pub struct SpriteBatch {
    sprite_sheet: SpriteSheet,
    filter_mode: FilterMode,
    instances: Vec<SpriteInstance>,
    data: Option<SpriteBatchData>,
    status: BatchStatus,
}

impl SpriteBatch {
    pub fn new(sprite_sheet: SpriteSheet, filter_mode: FilterMode) -> Self {
        Self {
            sprite_sheet,
            filter_mode,
            instances: Vec::new(),
            data: None,
            status: BatchStatus::Empty,
        }
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.status = BatchStatus::Empty;
    }

    pub fn add(&mut self, sprite: &Sprite, transform: &Transform) {
        let sprite_sheet_size =
            Vec2::new(self.sprite_sheet.width() as f32, self.sprite_sheet.height() as f32);

        let sprite_bounds =
            self.sprite_sheet.get_bounds(sprite.index).expect("Sprite index out of range");

        let size = sprite
            .size
            .unwrap_or_else(|| Vec2::new(sprite_bounds.w as f32, sprite_bounds.h as f32));

        let affine = transform.to_affine2();

        let pixel_tex_coords_edges = {
            let (left, right) = {
                let left = sprite_bounds.x as f32;
                let right = (sprite_bounds.x + sprite_bounds.w) as f32;

                if sprite.flip_x {
                    (right, left)
                } else {
                    (left, right)
                }
            };

            let (top, bottom) = {
                let top = sprite_bounds.y as f32;
                let bottom = (sprite_bounds.y + sprite_bounds.h) as f32;

                if sprite.flip_y {
                    (bottom, top)
                } else {
                    (top, bottom)
                }
            };

            Vec4::new(top, left, bottom, right)
        };

        let instance = SpriteInstance {
            sprite_sheet_size,
            size,
            anchor: sprite.anchor,
            scale_rotation_col_0: affine.matrix2.col(0),
            scale_rotation_col_1: affine.matrix2.col(1),
            translation: affine.translation,
            pixel_tex_coords_edges,
            linear_color: sprite.color.to_linear_vec4(),
        };

        self.instances.push(instance);
        self.status = BatchStatus::NonEmpty;
    }

    #[inline]
    pub fn sprite_sheet(&self) -> &SpriteSheet {
        &self.sprite_sheet
    }
}

impl Drawable for SpriteBatch {
    fn prepare(&mut self, ctx: &Context, projection: Projection) {
        if self.status != BatchStatus::NonEmpty {
            return;
        }

        let projection_matrix = projection.to_ortho_mat4();

        let device = &ctx.graphics.device;
        let queue = &ctx.graphics.queue;

        let create_instance_buffer = |instances: &[SpriteInstance]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("sprite_batch_instance_buffer"),
                contents: bytemuck::cast_slice(instances),
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            })
        };

        match self.data.as_mut() {
            Some(data) => {
                if self.instances.len() <= data.instances_capacity {
                    queue.write_buffer(&data.instances, 0, bytemuck::cast_slice(&self.instances));
                } else {
                    data.instances = create_instance_buffer(&self.instances);
                    data.instances_capacity = self.instances.len();
                }

                queue.write_buffer(&data.projection, 0, bytemuck::bytes_of(&projection_matrix));
            }
            None => {
                let projection = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("sprite_batch_projection_buffer"),
                    contents: bytemuck::bytes_of(&projection_matrix),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

                let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: self.filter_mode.into(),
                    min_filter: self.filter_mode.into(),
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    ..Default::default()
                });

                let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("sprite_batch_bind_group"),
                    layout: &ctx.graphics.sprite_pipeline.bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: projection.as_entire_binding(),
                        },
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
                    projection,
                    bind_group,
                });
            }
        }

        self.status = BatchStatus::NonEmpty;
    }

    fn draw<'a>(&'a self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>) {
        let data = match self.data.as_ref() {
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
