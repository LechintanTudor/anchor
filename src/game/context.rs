use crate::game::{Config, GameResult};
use crate::graphics::{GraphicsContext, WgpuContext};
use crate::time::TimeContext;
use winit::event_loop::EventLoopWindowTarget;

#[derive(Debug)]
pub struct Context {
    pub time: TimeContext,
    pub graphics: GraphicsContext,
}

impl Context {
    pub fn new(event_loop: &EventLoopWindowTarget<()>, config: &Config) -> GameResult<Self> {
        Ok(Self {
            time: TimeContext::new(config),
            graphics: GraphicsContext::new(event_loop, config)?,
        })
    }
}

impl AsRef<TimeContext> for Context {
    fn as_ref(&self) -> &TimeContext {
        &self.time
    }
}

impl AsMut<TimeContext> for Context {
    fn as_mut(&mut self) -> &mut TimeContext {
        &mut self.time
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

impl AsRef<WgpuContext> for Context {
    fn as_ref(&self) -> &WgpuContext {
        &self.graphics.wgpu
    }
}
