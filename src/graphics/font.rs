use ab_glyph::{CodepointIdIter, FontVec, GlyphId, GlyphImage, Outline};
use glyph_brush::ab_glyph;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Font(Arc<FontVec>);

impl Font {
    #[inline]
    pub(crate) fn new(font_vec: FontVec) -> Self {
        Self(Arc::new(font_vec))
    }

    #[inline]
    pub(crate) fn id(&self) -> usize {
        self.0.deref() as *const FontVec as usize
    }
}

impl ab_glyph::Font for Font {
    #[inline]
    fn units_per_em(&self) -> Option<f32> {
        self.0.units_per_em()
    }

    #[inline]
    fn ascent_unscaled(&self) -> f32 {
        self.0.ascent_unscaled()
    }

    #[inline]
    fn descent_unscaled(&self) -> f32 {
        self.0.descent_unscaled()
    }

    #[inline]
    fn line_gap_unscaled(&self) -> f32 {
        self.0.line_gap_unscaled()
    }

    #[inline]
    fn glyph_id(&self, c: char) -> GlyphId {
        self.0.glyph_id(c)
    }

    #[inline]
    fn h_advance_unscaled(&self, id: GlyphId) -> f32 {
        self.0.h_advance_unscaled(id)
    }

    #[inline]
    fn h_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
        self.0.h_side_bearing_unscaled(id)
    }

    #[inline]
    fn v_advance_unscaled(&self, id: GlyphId) -> f32 {
        self.0.v_advance_unscaled(id)
    }

    #[inline]
    fn v_side_bearing_unscaled(&self, id: GlyphId) -> f32 {
        self.0.v_side_bearing_unscaled(id)
    }

    #[inline]
    fn kern_unscaled(&self, first: GlyphId, second: GlyphId) -> f32 {
        self.0.kern_unscaled(first, second)
    }

    #[inline]
    fn outline(&self, glyph: GlyphId) -> Option<Outline> {
        self.0.outline(glyph)
    }

    #[inline]
    fn glyph_count(&self) -> usize {
        self.0.glyph_count()
    }

    #[inline]
    fn codepoint_ids(&self) -> CodepointIdIter<'_> {
        self.0.codepoint_ids()
    }

    #[inline]
    fn glyph_raster_image(&self, id: GlyphId, size: u16) -> Option<GlyphImage> {
        self.0.glyph_raster_image(id, size)
    }
}
