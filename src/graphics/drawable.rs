use crate::graphics::Canvas;

pub trait Drawable {
    fn draw(self, canvas: &mut Canvas);
}
