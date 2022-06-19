mod color;
mod context;
mod frame;
mod image;
mod shape;
mod shape_batch;
mod shape_pipeline;
mod shapes;
mod sprite;
mod sprite_batch;
mod sprite_pipeline;
mod sprite_sheet;
mod texture;
mod transform;

use crate::core::Context;
use std::path::Path;

pub use self::color::*;
pub use self::frame::*;
pub use self::image::*;
pub use self::shape::*;
pub use self::shape_batch::*;
pub use self::shape_pipeline::*;
pub use self::shapes::*;
pub use self::sprite::*;
pub use self::sprite_batch::*;
pub use self::sprite_pipeline::*;
pub use self::sprite_sheet::*;
pub use self::texture::*;
pub use self::transform::*;
pub use glam::f32::{Vec2, Vec4};

pub(crate) use self::context::*;

pub fn load_texure<P>(ctx: &Context, path: P) -> Texture
where
    P: AsRef<Path>,
{
    Texture::new(&Image::load(path), &ctx.graphics.device, &ctx.graphics.queue)
}

pub(crate) fn display(ctx: &mut Context, mut frame: Frame) {
    let output = match ctx.graphics.surface.get_current_texture() {
        Ok(output) => output,
        Err(wgpu::SurfaceError::Lost) => {
            ctx.graphics.surface.configure(&ctx.graphics.device, &ctx.graphics.surface_config);
            return;
        }
        _ => return,
    };
    let output_view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

    for drawable in frame.drawables.iter_mut() {
        drawable.prepare(ctx);
    }

    let mut encoder =
        ctx.graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(frame.clear_color.into()),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        for drawable in frame.drawables.iter_mut() {
            drawable.draw(ctx, &mut pass);
        }
    }

    let command_buffer = encoder.finish();
    ctx.graphics.queue.submit(Some(command_buffer));
    output.present();
}
