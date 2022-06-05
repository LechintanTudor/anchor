use crate::core::Context;
use crate::graphics::Color;

pub trait Drawable {
    fn prepare(&mut self, ctx: &mut Context);

    fn draw<'a>(&'a mut self, ctx: &'a Context, render_pass: &mut wgpu::RenderPass<'a>);
}

pub struct Frame<'a> {
    clear_color: Color,
    drawables: Vec<&'a mut dyn Drawable>,
}

impl Default for Frame<'_> {
    fn default() -> Self {
        Self { clear_color: Color::BLACK, drawables: Vec::new() }
    }
}

impl<'a> Frame<'a> {
    pub fn new(clear_color: Color) -> Self {
        Self { clear_color, drawables: Vec::new() }
    }

    pub fn draw(mut self, drawable: &'a mut dyn Drawable) -> Self {
        self.drawables.push(drawable);
        self
    }

    pub(crate) fn split(self) -> (Color, Vec<&'a mut dyn Drawable>) {
        (self.clear_color, self.drawables)
    }
}
