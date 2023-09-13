use crate::game::{Config, GameResult};
use crate::graphics::GraphicsContext;
use winit::event_loop::EventLoopWindowTarget;

#[derive(Debug)]
pub struct Context {
    pub graphics: GraphicsContext,
}

impl Context {
    pub fn new(event_loop: &EventLoopWindowTarget<()>, config: &Config) -> GameResult<Self> {
        Ok(Self { graphics: GraphicsContext::new(event_loop, config)? })
    }
}

impl AsRef<GraphicsContext> for Context {
    fn as_ref(&self) -> &GraphicsContext {
        &self.graphics
    }
}

impl AsMut<GraphicsContext> for Context {
    fn as_mut(&mut self) -> &mut GraphicsContext {
        &mut self.graphics
    }
}
