use crate::game::Context;
use crate::graphics::{Color, Layer, Projection};
use glam::Vec2;

/// Clears the screen with the provided color, then draws the layers.
pub fn draw(ctx: &mut Context, clear_color: Color, layers: &mut [Layer]) {
    let surface_texture = match ctx.graphics.surface_texture.take() {
        Some(surface_texture) => surface_texture,
        None => return,
    };

    for layer in layers.iter_mut() {
        layer.drawable.prepare(ctx, layer.projection);
    }

    let mut encoder = ctx.graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("display_command_buffer"),
    });

    let mut framebuffer = ctx.graphics.framebuffer.take();

    {
        let mut pass = match framebuffer.as_mut() {
            Some(framebuffer) => encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("display_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &framebuffer.view,
                    resolve_target: Some(&surface_texture.view),
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color.into()),
                        store: false,
                    },
                })],
                depth_stencil_attachment: None,
            }),
            None => encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("display_render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color.into()),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            }),
        };

        for layer in layers.iter_mut() {
            let viewport = layer.projection.viewport;
            pass.set_viewport(viewport.x, viewport.y, viewport.w, viewport.h, 0.0, 1.0);

            layer.drawable.draw(ctx, &mut pass);
        }
    }

    ctx.graphics.queue.submit(Some(encoder.finish()));

    ctx.graphics.surface_texture = Some(surface_texture);
    ctx.graphics.framebuffer = framebuffer;
}

/// Toggles vsync. Enabling vsync caps the framerate to the display's refresh rate.
#[inline]
pub fn set_vsync(ctx: &mut Context, vsync: bool) {
    ctx.graphics.next_config.vsync = vsync;
}

/// Toggles multisampling. Enabling multisampling smooths out rough edges but decreases performance.
#[inline]
pub fn set_multisample(ctx: &mut Context, multisample: bool) {
    ctx.graphics.next_config.multisample = multisample;
}

/// Returns the size of the drawing surface.
#[inline]
pub fn surface_size(ctx: &Context) -> Vec2 {
    Vec2::new(ctx.graphics.surface_config.width as f32, ctx.graphics.surface_config.height as f32)
}

#[inline]
pub fn fill_projection(ctx: &Context) -> Projection {
    Projection::fill(surface_size(ctx))
}

#[inline]
pub fn fit_projection(ctx: &Context, camera_size: Vec2) -> Projection {
    Projection::fit(camera_size, surface_size(ctx))
}

/// Returns whether vsync is enabled.
#[inline]
pub fn vsync(ctx: &Context) -> bool {
    ctx.graphics.config.vsync
}

/// Returns whether multisampling is enabled.
#[inline]
pub fn multisample(ctx: &Context) -> bool {
    ctx.graphics.config.multisample
}
