use crate::core::Context;
use crate::graphics::Color;

pub trait Drawable {
    fn prepare(&mut self, ctx: &mut Context);

    fn draw<'a>(&'a mut self, ctx: &'a Context, render_pass: &mut wgpu::RenderPass<'a>);
}

pub struct FrameBuilder<'a> {
    clear_color: Color,
    drawables: Vec<&'a mut dyn Drawable>,
}

impl Default for FrameBuilder<'_> {
    fn default() -> Self {
        Self { clear_color: Color::BLACK, drawables: Vec::new() }
    }
}

impl<'a> FrameBuilder<'a> {
    pub fn draw(&mut self, drawable: &'a mut dyn Drawable) -> &mut Self {
        self.drawables.push(drawable);
        self
    }

    pub fn build(&mut self) -> Frame<'a> {
        Frame { clear_color: self.clear_color, drawables: std::mem::take(&mut self.drawables) }
    }
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
    pub fn builder(clear_color: Color) -> FrameBuilder<'a> {
        FrameBuilder { clear_color, drawables: Vec::new() }
    }

    pub(crate) fn split(self) -> (Color, Vec<&'a mut dyn Drawable>) {
        (self.clear_color, self.drawables)
    }
}
