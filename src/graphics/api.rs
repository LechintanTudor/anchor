use crate::graphics::{
    Color, Drawable, Font, Image, Projection, ProjectionBuilder, Shape, SpriteBounds, SpriteSheet,
    Texture,
};
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

#[inline]
pub unsafe fn create_shape_unsafe(ctx: &Context, vertexes: &[Vec2], indexes: &[u32]) -> Shape {
    Shape::new(&ctx.graphics.device, vertexes, indexes)
}

pub fn create_rectangle_shape(ctx: &Context, size: Vec2) -> Shape {
    let half_size = size / 2.0;
    let vertexes = [
        -half_size,
        Vec2::new(-half_size.x, half_size.y),
        half_size,
        Vec2::new(half_size.x, -half_size.y),
    ];
    let indexes = [0, 1, 3, 3, 1, 2];

    unsafe { Shape::new(&ctx.graphics.device, &vertexes, &indexes) }
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

pub fn set_default_projection_builder<P>(ctx: &mut Context, projection_builder: P)
where
    P: ProjectionBuilder,
{
    let default_projection = projection_builder.build_projection(window_size(ctx));
    ctx.graphics.default_projection_builder = Box::new(projection_builder);
    ctx.graphics.default_projection = default_projection;
}

#[inline]
pub fn default_projection(ctx: &Context) -> Projection {
    ctx.graphics.default_projection
}

pub fn display(ctx: &mut Context, clear_color: Color, drawables: &mut [&mut dyn Drawable]) {
    let surface_texture = match ctx.graphics.surface_texture.take() {
        Some(surface_texture) => surface_texture,
        None => return,
    };

    for drawable in drawables.iter_mut() {
        drawable.prepare(ctx);
    }

    let mut encoder = ctx.graphics.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("display_command_buffer"),
    });

    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
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

        for drawable in drawables.iter_mut() {
            drawable.draw(ctx, &mut pass);
        }
    }

    ctx.graphics.queue.submit(Some(encoder.finish()));
    ctx.graphics.surface_texture = Some(surface_texture);
}
