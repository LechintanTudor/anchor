use crate::game::{Config, GamePhase, GameResult, ShouldExit};
use crate::graphics::GraphicsContext;
use crate::input::InputContext;
use crate::time::TimeContext;
use crate::window::WindowContext;

use winit::event_loop::EventLoop;

/// Groups together functionallity from all modules of the crate.
pub struct Context {
    pub(crate) should_exit: ShouldExit,
    pub(crate) game_phase: GamePhase,
    pub(crate) time: TimeContext,
    pub(crate) window: WindowContext,
    pub(crate) input: InputContext,
    pub(crate) graphics: GraphicsContext,
}

impl Context {
    pub(crate) fn new(event_loop: &EventLoop<()>, config: Config) -> GameResult<Self> {
        let time = TimeContext::new(config.time);
        let window = WindowContext::new(event_loop, config.window)?;
        let graphics = GraphicsContext::new(&window.window, config.graphics);

        Ok(Self {
            should_exit: ShouldExit::No,
            game_phase: GamePhase::Input,
            time,
            window,
            input: Default::default(),
            graphics,
        })
    }

    pub(crate) fn take_should_exit(&mut self) -> ShouldExit {
        let should_exit = self.should_exit;
        self.should_exit = ShouldExit::No;
        should_exit
    }
}
