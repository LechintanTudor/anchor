use crate::graphics::{Color, GlyphInstance};
use glam::{Vec2, Vec4};

pub(crate) type RawGlyphInstance<'a> = glyph_brush::GlyphVertex<'a, Color>;

pub(crate) fn into_glyph_instance(instance: RawGlyphInstance) -> GlyphInstance {
    let text_bounds = instance.bounds;
    let glyph_bounds = instance.pixel_coords;

    let min_x_outside_pixels = (text_bounds.min.x - glyph_bounds.min.x).max(0.0);
    let min_y_outside_pixels = (text_bounds.min.y - glyph_bounds.min.y).max(0.0);
    let max_x_outside_pixels = (glyph_bounds.max.x - text_bounds.max.x).max(0.0);
    let max_y_outside_pixels = (glyph_bounds.max.y - text_bounds.max.y).max(0.0);

    let glyph_fully_visible =
        (min_x_outside_pixels, min_y_outside_pixels, max_x_outside_pixels, max_y_outside_pixels)
            == (0.0, 0.0, 0.0, 0.0);

    if glyph_fully_visible {
        into_unclipped_glyph_instance(instance)
    } else {
        into_clipped_glyph_instance(
            instance,
            min_x_outside_pixels,
            min_y_outside_pixels,
            max_x_outside_pixels,
            max_y_outside_pixels,
        )
    }
}

fn into_unclipped_glyph_instance(instance: RawGlyphInstance) -> GlyphInstance {
    let RawGlyphInstance { pixel_coords: glyph_bounds, tex_coords, extra: color, .. } = instance;

    let size = Vec2::new(glyph_bounds.width(), glyph_bounds.height());

    let translation =
        Vec2::new(glyph_bounds.min.x + glyph_bounds.max.x, glyph_bounds.min.y + glyph_bounds.max.y)
            / 2.0;

    let tex_coords_edges =
        Vec4::new(tex_coords.min.y, tex_coords.min.x, tex_coords.max.y, tex_coords.max.x);

    let linear_color = color.to_linear_vec4();

    GlyphInstance { size, translation, tex_coords_edges, linear_color }
}

fn into_clipped_glyph_instance(
    instance: RawGlyphInstance,
    min_x_outside_pixels: f32,
    min_y_outside_pixels: f32,
    max_x_outside_pixels: f32,
    max_y_outside_pixels: f32,
) -> GlyphInstance {
    use glyph_brush::ab_glyph::{point, Rect};

    let RawGlyphInstance { pixel_coords: glyph_bounds, tex_coords, extra: color, .. } = instance;

    let size = Vec2::new(
        glyph_bounds.width() - min_x_outside_pixels - max_x_outside_pixels,
        glyph_bounds.height() - min_y_outside_pixels - max_y_outside_pixels,
    );

    let bounds = Rect {
        min: point(
            glyph_bounds.min.x + min_x_outside_pixels,
            glyph_bounds.min.y + min_y_outside_pixels,
        ),
        max: point(
            glyph_bounds.max.x - max_x_outside_pixels,
            glyph_bounds.max.y - max_y_outside_pixels,
        ),
    };

    let translation = Vec2::new(bounds.min.x + bounds.max.x, bounds.min.y + bounds.max.y) / 2.0;

    let tex_coords_edges = {
        fn scale_between(min: f32, value: f32, max: f32) -> f32 {
            (value - min) / (max - min)
        }

        fn from_to_scaled(from: f32, to: f32, scale: f32) -> f32 {
            from + (to - from) * scale
        }

        let min_x_scale = scale_between(glyph_bounds.min.x, bounds.min.x, glyph_bounds.max.x);
        let min_y_scale = scale_between(glyph_bounds.min.y, bounds.min.y, glyph_bounds.max.y);
        let max_x_scale = scale_between(glyph_bounds.min.x, bounds.max.x, glyph_bounds.max.x);
        let max_y_scale = scale_between(glyph_bounds.min.y, bounds.max.y, glyph_bounds.max.y);

        Vec4::new(
            from_to_scaled(tex_coords.min.y, tex_coords.max.y, min_y_scale),
            from_to_scaled(tex_coords.min.x, tex_coords.max.x, min_x_scale),
            from_to_scaled(tex_coords.min.y, tex_coords.max.y, max_y_scale),
            from_to_scaled(tex_coords.min.x, tex_coords.max.x, max_x_scale),
        )
    };

    let linear_color = color.to_linear_vec4();

    GlyphInstance { size, translation, tex_coords_edges, linear_color }
}
