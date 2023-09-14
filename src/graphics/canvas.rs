use crate::graphics::shape::{Shape, ShapeBatch, ShapeInstance};
use crate::graphics::sprite::SpriteBatch;
use crate::graphics::{Camera, Color, Drawable, GraphicsContext};
use glam::Mat4;
use std::ops::Range;

enum CanvasCommand {
    UpdateCamera,
    DrawShapes(ShapeBatch),
    DrawSprites(SpriteBatch),
}

pub struct Canvas<'a> {
    graphics: &'a mut GraphicsContext,
    surface_texture: wgpu::SurfaceTexture,
    clear_color: Color,
    commands: Vec<CanvasCommand>,
    projections: Vec<Mat4>,
}

impl<'a> Canvas<'a> {
    pub fn new<G>(graphics: &'a mut G) -> Self
    where
        G: AsMut<GraphicsContext>,
    {
        let graphics = graphics.as_mut();
        graphics.camera_manager.clear();
        graphics.shape_renderer.begin();

        let surface_texture = graphics.get_surface_texture().unwrap();
        let surface_size = graphics.surface_size().as_vec2();
        let projection = Camera::from_size(surface_size).ortho_matrix();

        Self {
            graphics,
            surface_texture,
            clear_color: Color::BLACK,
            commands: vec![CanvasCommand::UpdateCamera],
            projections: vec![projection],
        }
    }

    pub fn set_clear_color(&mut self, clear_color: Color) {
        self.clear_color = clear_color;
    }

    pub fn set_camera<P>(&mut self, projection: P)
    where
        P: Into<Mat4>,
    {
        let projection = projection.into();

        if matches!(self.commands.last_mut(), Some(CanvasCommand::UpdateCamera)) {
            *self.projections.last_mut().unwrap() = projection;
        } else {
            self.commands.push(CanvasCommand::UpdateCamera);
            self.projections.push(projection);
        }
    }

    pub fn draw<D>(&mut self, drawable: D)
    where
        D: Drawable,
    {
        drawable.draw(self);
    }

    pub fn draw_shape(&mut self, shape: &Shape, shape_instance: ShapeInstance) {
        match self.commands.last_mut() {
            Some(CanvasCommand::DrawShapes(batch)) if &batch.shape == shape => {
                batch.instances.end += 1;
            }
            _ => {
                self.commands.push(CanvasCommand::DrawShapes(ShapeBatch {
                    shape: shape.clone(),
                    instances: Range {
                        start: self.graphics.shape_renderer.instance_count(),
                        end: self.graphics.shape_renderer.instance_count() + 1,
                    },
                }))
            }
        }

        self.graphics.shape_renderer.add(shape_instance);
    }

    pub fn present(self) {
        self.graphics.shape_renderer.end();

        for projection in self.projections.iter() {
            self.graphics.camera_manager.alloc_bind_group(projection);
        }

        let mut projection_bind_group_index = 0;

        let mut next_projection_bind_group = || {
            let bind_group = &self.graphics.camera_manager[projection_bind_group_index];
            projection_bind_group_index += 1;
            bind_group
        };

        let mut encoder =
            self.graphics.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("command_encoder"),
            });

        let surface_view = self.surface_texture.texture.create_view(&Default::default());

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            let mut last_draw_command = &CanvasCommand::UpdateCamera;

            for command in self.commands.iter() {
                match command {
                    CanvasCommand::UpdateCamera => {
                        pass.set_bind_group(0, next_projection_bind_group(), &[]);
                    }
                    CanvasCommand::DrawShapes(batch) => {
                        if !matches!(last_draw_command, CanvasCommand::DrawShapes(_)) {
                            self.graphics.shape_renderer.prepare_pipeline(&mut pass);
                            last_draw_command = command;
                        }

                        self.graphics.shape_renderer.draw(&mut pass, batch);
                    }
                    CanvasCommand::DrawSprites(batch) => {
                        todo!();
                    }
                }
            }
        }

        self.graphics.wgpu.queue().submit(Some(encoder.finish()));
        self.surface_texture.present();
    }
}
