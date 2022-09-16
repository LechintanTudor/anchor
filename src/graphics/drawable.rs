use crate::graphics::Projection;
use crate::platform::Context;

pub trait Drawable {
    fn prepare(&mut self, ctx: &Context, projection: Projection);

    fn draw<'a>(&'a self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>);
}
