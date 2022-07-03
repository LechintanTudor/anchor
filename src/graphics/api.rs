use crate::core::{Context, FileError, GameError, GameResult};
use crate::graphics::{Frame, Image, Texture};
use image::ImageError;
use std::path::Path;

pub fn load_image<P>(ctx: &Context, path: P) -> GameResult<Image>
where
    P: AsRef<Path>,
{
    let _ = ctx;
    let path = path.as_ref();

    image::open(path).map(|image| Image::new(image.to_rgba8())).map_err(|error| {
        Box::new(match error {
            ImageError::IoError(error) => {
                GameError::FileError(FileError::new(path.to_path_buf(), error))
            }
            _ => GameError::ImageError(error),
        })
    })
}

pub fn load_texure<P>(ctx: &Context, path: P) -> GameResult<Texture>
where
    P: AsRef<Path>,
{
    let image = load_image(ctx, path)?;
    Ok(Texture::new(&image, &ctx.graphics.device, &ctx.graphics.queue))
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
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &output_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(frame.clear_color.into()),
                    store: true,
                },
            })],
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
