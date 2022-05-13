mod color;
mod context;
mod frame;
mod shape;
mod shape_batch;
mod shape_pipeline;
mod transform;

pub use self::color::*;
pub use self::frame::*;
pub use self::shape::*;
pub use self::shape_batch::*;
pub use self::shape_pipeline::*;
pub use self::transform::*;

pub(crate) use self::context::*;

use crate::core::Context;

pub(crate) fn display(ctx: &mut Context, frame: Frame) {
    let output = match ctx.graphics.surface.get_current_texture() {
        Ok(output) => output,
        Err(wgpu::SurfaceError::Lost) => {
            ctx.graphics.surface.configure(&ctx.graphics.device, &ctx.graphics.surface_config);
            return;
        }
        _ => return,
    };
    let output_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    let (clear_color, mut drawables) = frame.split();

    for drawable in drawables.iter_mut() {
        drawable.prepare(ctx);
    }

    let mut encoder =
        ctx.graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color.into()),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        for drawable in drawables.iter_mut() {
            drawable.draw(ctx, &mut render_pass);
        }
    }

    let command_buffer = encoder.finish();
    ctx.graphics.queue.submit(Some(command_buffer));
    output.present();
}
