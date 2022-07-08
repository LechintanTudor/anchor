use crate::core::{Context, FileError, GameError, GameResult};
use crate::graphics::{Font, Frame, Image, Texture};
use glam::Vec2;
use image::ImageError;
use std::path::Path;

#[inline]
pub fn window_size(ctx: &Context) -> Vec2 {
    let size = ctx.window.inner_size();
    Vec2::new(size.width as f32, size.height as f32)
}

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

pub fn load_font<P>(ctx: &Context, path: P) -> GameResult<Font>
where
    P: AsRef<Path>,
{
    fn inner(_ctx: &Context, path: &Path) -> GameResult<Font> {
        let data = match std::fs::read(path) {
            Ok(data) => data,
            Err(error) => {
                return Err(Box::new(GameError::FileError(FileError::new(
                    path.to_path_buf(),
                    error,
                ))))
            }
        };

        let font_vec = glyph_brush::ab_glyph::FontVec::try_from_vec(data).expect("TODO");
        Ok(Font::new(font_vec))
    }

    inner(ctx, path.as_ref())
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
