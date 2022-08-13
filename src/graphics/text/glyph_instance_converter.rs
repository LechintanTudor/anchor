use crate::graphics::{Color, GlyphInstance};
use glam::{Affine2, Vec2, Vec4};
use std::hash::{Hash, Hasher};

#[derive(Clone, PartialEq)]
pub(crate) struct RawGlyphInstanceData {
    pub color: Color,
    pub affine: Affine2,
    pub pivot: Vec2,
}

impl Hash for RawGlyphInstanceData {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        69.hash(state);
    }
}

pub(crate) type RawGlyphInstance<'a> = glyph_brush::GlyphVertex<'a, RawGlyphInstanceData>;

pub(crate) fn into_glyph_instance(instance: RawGlyphInstance) -> GlyphInstance {
    let RawGlyphInstance { pixel_coords, bounds, .. } = instance;

    let min_x_outside_pixels = (bounds.min.x - pixel_coords.min.x).max(0.0);
    let min_y_outside_pixels = (bounds.min.y - pixel_coords.min.y).max(0.0);
    let max_x_outside_pixels = (pixel_coords.max.x - bounds.max.x).max(0.0);
    let max_y_outside_pixels = (pixel_coords.max.y - bounds.max.y).max(0.0);

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
    let RawGlyphInstance { pixel_coords, tex_coords, extra, .. } = instance;
    let &RawGlyphInstanceData { color, affine, pivot: _ } = extra;

    let bounds_edges =
        Vec4::new(pixel_coords.min.y, pixel_coords.min.x, pixel_coords.max.y, pixel_coords.max.x);

    let tex_coords_edges =
        Vec4::new(tex_coords.min.y, tex_coords.min.x, tex_coords.max.y, tex_coords.max.x);

    GlyphInstance {
        bounds_edges,
        tex_coords_edges,
        linear_color: color.to_linear_vec4(),
        scale_rotation_col_0: affine.matrix2.col(0),
        scale_rotation_col_1: affine.matrix2.col(1),
        translation: affine.translation,
        ..Default::default()
    }
}

fn into_clipped_glyph_instance(
    instance: RawGlyphInstance,
    min_x_outside_pixels: f32,
    min_y_outside_pixels: f32,
    max_x_outside_pixels: f32,
    max_y_outside_pixels: f32,
) -> GlyphInstance {
    let RawGlyphInstance { pixel_coords, tex_coords, bounds: _, extra } = instance;
    let &RawGlyphInstanceData { color, affine, pivot: _ } = extra;

    let bounds_edges = Vec4::new(
        pixel_coords.min.y + min_y_outside_pixels,
        pixel_coords.min.x + min_x_outside_pixels,
        pixel_coords.max.y - max_y_outside_pixels,
        pixel_coords.max.x - max_x_outside_pixels,
    );

    let tex_coords_edges = {
        fn scale_between(min: f32, value: f32, max: f32) -> f32 {
            (value - min) / (max - min)
        }

        fn from_to_scaled(from: f32, to: f32, scale: f32) -> f32 {
            from + (to - from) * scale
        }

        let min_y_scale = scale_between(pixel_coords.min.y, bounds_edges[0], pixel_coords.max.y);
        let min_x_scale = scale_between(pixel_coords.min.x, bounds_edges[1], pixel_coords.max.x);
        let max_y_scale = scale_between(pixel_coords.min.y, bounds_edges[2], pixel_coords.max.y);
        let max_x_scale = scale_between(pixel_coords.min.x, bounds_edges[3], pixel_coords.max.x);

        Vec4::new(
            from_to_scaled(tex_coords.min.y, tex_coords.max.y, min_y_scale),
            from_to_scaled(tex_coords.min.x, tex_coords.max.x, min_x_scale),
            from_to_scaled(tex_coords.min.y, tex_coords.max.y, max_y_scale),
            from_to_scaled(tex_coords.min.x, tex_coords.max.x, max_x_scale),
        )
    };

    GlyphInstance {
        bounds_edges,
        tex_coords_edges,
        linear_color: color.to_linear_vec4(),
        scale_rotation_col_0: affine.matrix2.col(0),
        scale_rotation_col_1: affine.matrix2.col(1),
        translation: affine.translation,
        ..Default::default()
    }
}
