use crate::core::{Config, GameErrorKind, GamePhase, GameResult};
use crate::graphics::GraphicsContext;
use crate::input::InputContext;
use crate::time::TimeContext;
use winit::dpi::Size as WindowSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct Context {
    pub(crate) should_exit: bool,
    pub(crate) game_phase: GamePhase,
    pub(crate) time: TimeContext,
    pub(crate) window: Window,
    pub(crate) input: InputContext,
    pub(crate) graphics: GraphicsContext,
}

impl Context {
    pub fn new(event_loop: &EventLoop<()>, config: Config) -> GameResult<Self> {
        let time = TimeContext::new(config.time);

        let window = WindowBuilder::new()
            .with_title(config.window.title)
            .with_inner_size(WindowSize::Physical(config.window.size.into()))
            .with_resizable(config.window.resizable)
            .build(event_loop)
            .map_err(|error| GameErrorKind::OsError(error).into_error())?;

        window.set_cursor_visible(config.window.cursor_visible);

        let graphics = GraphicsContext::new(&window, config.graphics);

        Ok(Self {
            should_exit: false,
            game_phase: GamePhase::Input,
            time,
            window,
            input: Default::default(),
            graphics,
        })
    }

    pub(crate) fn take_should_exit(&mut self) -> bool {
        let should_exit = self.should_exit;
        self.should_exit = false;
        should_exit
    }
}
