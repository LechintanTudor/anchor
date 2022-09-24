use crate::core::Context;
use crate::graphics::Projection;

/// Object or batch of objects that can be drawn on the screen.
pub trait Drawable {
    /// Uploads the required data to the GPU to prepare for rendering.
    fn prepare(&mut self, ctx: &Context, projection: Projection);

    /// Issues the draw commands to draw to the screen.
    fn draw<'a>(&'a self, ctx: &'a Context, pass: &mut wgpu::RenderPass<'a>);
}
