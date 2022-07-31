use crate::core::Context;

pub trait Drawable {
    fn prepare(&mut self, ctx: &mut Context);

    fn draw<'a>(&'a mut self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>);
}
