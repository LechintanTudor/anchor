use crate::graphics::shape::{Shape, ShapeBatch, ShapeInstance};
use crate::graphics::sprite::{SpriteBatch, SpriteInstance, Texture};
use crate::graphics::text::Text;
use crate::graphics::{Bounds, Color, Drawable, GraphicsContext, WgpuContext};
use glam::Mat4;
use std::ops::Range;

#[derive(Clone, Debug)]
enum CanvasCommand {
    UpdateProjection,
    UpdateViewport(Bounds),
    DrawShapes(ShapeBatch),
    DrawSprites(SpriteBatch),
    DrawText(Range<u32>),
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
        graphics.projection_bind_group_allocator.clear();
        graphics.shape_renderer.begin();
        graphics.sprite_renderer.begin();
        graphics.text_renderer.begin();

        let surface_texture = graphics.surface_texture.take().unwrap();
        let projection = graphics.default_camera().ortho_matrix();

        Self {
            graphics,
            surface_texture,
            clear_color: Color::BLACK,
            commands: vec![CanvasCommand::UpdateProjection],
            projections: vec![projection],
        }
    }

    pub fn set_clear_color(&mut self, clear_color: Color) {
        self.clear_color = clear_color;
    }

    pub fn set_projection<P>(&mut self, projection: P)
    where
        P: Into<Mat4>,
    {
        let projection = projection.into();

        match self.commands.last() {
            Some(CanvasCommand::UpdateProjection) => {
                *self.projections.last_mut().unwrap() = projection;
            }
            _ => {
                self.commands.push(CanvasCommand::UpdateProjection);
                self.projections.push(projection);
            }
        }
    }

    pub fn set_viewport<V>(&mut self, viewport: V)
    where
        V: Into<Bounds>,
    {
        let viewport = viewport.into();

        match self.commands.last_mut() {
            Some(CanvasCommand::UpdateViewport(old_viewport)) => {
                *old_viewport = viewport;
            }
            _ => {
                self.commands.push(CanvasCommand::UpdateViewport(viewport));
            }
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
                self.commands.push(CanvasCommand::DrawShapes(
                    self.graphics.shape_renderer.next_batch(shape.clone()),
                ));
            }
        }

        self.graphics.shape_renderer.add(shape_instance);
    }

    pub fn draw_sprite(
        &mut self,
        texture: &Texture,
        smooth: bool,
        sprite_instance: SpriteInstance,
    ) {
        match self.commands.last_mut() {
            Some(CanvasCommand::DrawSprites(batch))
                if &batch.texture == texture && batch.smooth == smooth =>
            {
                batch.instances.end += 1;
            }
            _ => {
                self.commands.push(CanvasCommand::DrawSprites(
                    self.graphics
                        .sprite_renderer
                        .next_batch(texture.clone(), smooth),
                ));
            }
        }

        self.graphics.sprite_renderer.add(sprite_instance);
    }

    pub fn draw_text(&mut self, text: Text) {
        let text_index = self.graphics.text_renderer.add(text);

        match self.commands.last_mut() {
            Some(CanvasCommand::DrawText(text_range)) => text_range.end += 1,
            _ => {
                self.commands
                    .push(CanvasCommand::DrawText(text_index..(text_index + 1)))
            }
        }
    }

    pub fn present(self) {
        self.graphics.shape_renderer.end(&self.graphics.wgpu);
        self.graphics.sprite_renderer.end(&self.graphics.wgpu);
        self.graphics.text_renderer.end(&self.graphics.wgpu);

        for projection in self.projections.iter() {
            self.graphics
                .projection_bind_group_allocator
                .alloc(&self.graphics.wgpu, projection);
        }

        let mut projection_bind_group_index = 0;

        let mut next_projection_bind_group = || {
            let bind_group =
                &self.graphics.projection_bind_group_allocator[projection_bind_group_index];
            projection_bind_group_index += 1;
            bind_group
        };

        let mut encoder =
            self.graphics
                .device()
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("command_encoder"),
                });

        let surface_view = self
            .surface_texture
            .texture
            .create_view(&Default::default());

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color.into()),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            let mut last_draw_command = &CanvasCommand::UpdateProjection;

            for command in self.commands.iter() {
                match command {
                    CanvasCommand::UpdateProjection => {
                        pass.set_bind_group(0, next_projection_bind_group(), &[]);
                    }
                    CanvasCommand::UpdateViewport(viewport) => {
                        pass.set_viewport(viewport.x, viewport.y, viewport.w, viewport.h, 0.0, 1.0);
                    }
                    CanvasCommand::DrawShapes(batch) => {
                        if !matches!(last_draw_command, CanvasCommand::DrawShapes(_)) {
                            self.graphics.shape_renderer.prepare_pipeline(&mut pass);
                            last_draw_command = command;
                        }

                        self.graphics.shape_renderer.draw(&mut pass, batch);
                    }
                    CanvasCommand::DrawSprites(batch) => {
                        if !matches!(last_draw_command, CanvasCommand::DrawSprites(_)) {
                            self.graphics.sprite_renderer.prepare_pipeline(&mut pass);
                            last_draw_command = command;
                        }

                        let sampler_bind_group = if batch.smooth {
                            &self.graphics.linear_sampler_bind_group
                        } else {
                            &self.graphics.nearest_sampler_bind_group
                        };

                        self.graphics.sprite_renderer.draw(
                            &mut pass,
                            batch.texture.bind_group(),
                            sampler_bind_group,
                            batch.instances.clone(),
                        );
                    }
                    CanvasCommand::DrawText(text_range) => {
                        if !matches!(last_draw_command, CanvasCommand::DrawText(_)) {
                            self.graphics.text_renderer.prepare_pipeline(&mut pass);
                            last_draw_command = command;
                        }

                        self.graphics.text_renderer.draw(
                            &mut pass,
                            &self.graphics.linear_sampler_bind_group,
                            text_range.clone(),
                        );
                    }
                }
            }
        }

        self.graphics.wgpu.queue().submit(Some(encoder.finish()));
        self.surface_texture.present();
    }
}

impl AsRef<GraphicsContext> for Canvas<'_> {
    fn as_ref(&self) -> &GraphicsContext {
        self.graphics
    }
}

impl AsRef<WgpuContext> for Canvas<'_> {
    fn as_ref(&self) -> &WgpuContext {
        &self.graphics.wgpu
    }
}
