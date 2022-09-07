use crate::graphics::Projection;
use crate::platform::Context;

pub trait Drawable {
    fn prepare(&mut self, ctx: &Context, projection: Projection);

    fn draw<'a>(
        &'a mut self,
        ctx: &'a Context,
        projection: Projection,
        pass: &mut wgpu::RenderPass<'a>,
    );
}
