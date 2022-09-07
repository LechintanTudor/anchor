use crate::graphics::{Drawable, Projection};

pub struct Layer<'a> {
    pub projection: Projection,
    pub drawable: &'a mut dyn Drawable,
}

impl<'a> Layer<'a> {
    #[inline]
    pub fn new(projection: Projection, drawable: &'a mut dyn Drawable) -> Self {
        Self { projection, drawable }
    }
}
