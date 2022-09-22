use crate::core::{GameErrorKind, GameResult};
use glyph_brush::ab_glyph::{self, CodepointIdIter, FontVec, GlyphId, GlyphImage, Outline};
use std::path::Path;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Font(Arc<FontVec>);

impl Font {
    #[inline]
    pub fn new(data: Vec<u8>) -> GameResult<Self> {
        let font_vec = ab_glyph::FontVec::try_from_vec(data)
            .map_err(|e| GameErrorKind::FontError(e).into_error())?;

        Ok(Self(Arc::new(font_vec)))
    }

    pub fn load_from_file<P>(path: P) -> GameResult<Font>
    where
        P: AsRef<Path>,
    {
        fn inner(path: &Path) -> GameResult<Font> {
            let data = std::fs::read(path)
                .map_err(|e| GameErrorKind::IoError(e).into_error_with_path(path))?;

            let font_vec = ab_glyph::FontVec::try_from_vec(data)
                .map_err(|e| GameErrorKind::FontError(e).into_error_with_path(path))?;

            Ok(Self(Arc::new(font_vec)))
        }

        inner(path.as_ref())
    }

    #[inline]
    pub(crate) fn id(&self) -> usize {
        Arc::as_ptr(&self.0) as usize
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
