use crate::graphics::{Color, Font, Image, Layer, SpriteBounds, SpriteSheet, Texture};
use crate::platform::{Context, GameErrorKind, GameResult};
use glam::Vec2;
use image::ImageError;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[inline]
pub fn window_size(ctx: &Context) -> Vec2 {
    let size = ctx.window.inner_size();
    Vec2::new(size.width as f32, size.height as f32)
}

pub fn load_image<P>(_ctx: &Context, path: P) -> GameResult<Image>
where
    P: AsRef<Path>,
{
    fn inner(path: &Path) -> GameResult<Image> {
        match image::open(path) {
            Ok(image) => Ok(Image::new(image.into_rgba8())),
            Err(error) => match error {
                ImageError::IoError(error) => {
                    Err(GameErrorKind::IoError(error).into_error().with_source_path(path))
                }
                error => Err(GameErrorKind::ImageError(error).into_error()),
            },
        }
    }

    inner(path.as_ref())
}

pub fn load_texure<P>(ctx: &Context, path: P) -> GameResult<Texture>
where
    P: AsRef<Path>,
{
    let image = load_image(ctx, path)?;
    Ok(Texture::new(&image, &ctx.graphics.device, &ctx.graphics.queue))
}

pub fn load_sprite_sheet<P>(ctx: &Context, path: P) -> GameResult<SpriteSheet>
where
    P: AsRef<Path>,
{
    #[derive(Deserialize)]
    struct SerializedSpriteSheet {
        texture: String,
        sprites: HashMap<String, Vec<(u32, u32, u32, u32)>>,
    }

    fn inner(ctx: &Context, path: &Path) -> GameResult<SpriteSheet> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| GameErrorKind::IoError(e).into_error().with_source_path(path))?;

        let mut serialized_sprite_sheet = ron::from_str::<SerializedSpriteSheet>(&data)
            .map_err(|e| GameErrorKind::RonError(e).into_error().with_source_path(path))?;

        let texture = load_texure(ctx, &serialized_sprite_sheet.texture)
            .map_err(|e| e.with_source_path(path))?;

        let mut sprite_sheet_builder = SpriteSheet::builder(texture);

        for (name, bounds) in serialized_sprite_sheet.sprites.drain() {
            let bounds = bounds
                .iter()
                .map(|&(x, y, width, height)| SpriteBounds::new(x, y, width, height))
                .collect::<Vec<_>>();

            sprite_sheet_builder.add_sprites(name, bounds);
        }

        Ok(sprite_sheet_builder.build())
    }

    inner(ctx, path.as_ref())
}

pub fn load_font<P>(_ctx: &Context, path: P) -> GameResult<Font>
where
    P: AsRef<Path>,
{
    fn inner(path: &Path) -> GameResult<Font> {
        let data = std::fs::read(path)
            .map_err(|e| GameErrorKind::IoError(e).into_error().with_source_path(path))?;

        let font_vec = glyph_brush::ab_glyph::FontVec::try_from_vec(data)
            .map_err(|e| GameErrorKind::FontError(e).into_error().with_source_path(path))?;

        Ok(Font::new(font_vec))
    }

    inner(path.as_ref())
}

pub fn display(ctx: &mut Context, clear_color: Color, layers: &mut [Layer]) {
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

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("display_render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_texture.texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color.into()),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        for layer in layers.iter_mut() {
            let viewport = layer.projection.viewport;
            pass.set_viewport(viewport.x, viewport.y, viewport.w, viewport.h, 0.0, 1.0);

            layer.drawable.draw(ctx, &mut pass);
        }
    }

    ctx.graphics.queue.submit(Some(encoder.finish()));
    ctx.graphics.surface_texture = Some(surface_texture);
}
