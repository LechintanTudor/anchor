use crate::graphics::{Drawable, Projection};

/// Layer to draw on the screen.
pub struct Layer<'a> {
    /// Projection used to transform world coords into device coords.
    pub projection: Projection,
    /// Content to draw.
    pub drawable: &'a mut dyn Drawable,
}

impl<'a> Layer<'a> {
    /// Creates a [Layer] from the given [Projection] and [Drawable].
    #[inline]
    pub fn new(projection: Projection, drawable: &'a mut dyn Drawable) -> Self {
        Self { projection, drawable }
    }
}
