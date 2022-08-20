use crate::graphics::GraphicsContext;
use crate::input::InputContext;
use crate::platform::{Config, GameError, GameErrorKind, GameResult, Timer};
use winit::dpi::Size as WindowSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub use winit;
pub use winit::window::Window;

pub struct Context {
    pub(crate) should_exit: bool,
    pub(crate) vsync: bool,
    pub(crate) timer: Timer,
    pub(crate) window: Window,
    pub(crate) input: InputContext,
    pub(crate) graphics: GraphicsContext,
}

impl Context {
    pub fn new(event_loop: &EventLoop<()>, config: Config) -> GameResult<Self> {
        let timer =
            Timer::new(config.target_frames_per_second, config.target_fixed_updates_per_second);

        let window = WindowBuilder::new()
            .with_title(config.window_title)
            .with_inner_size(WindowSize::Physical(config.window_size.into()))
            .build(&event_loop)
            .map_err(|error| GameError::new(GameErrorKind::OsError(error)))?;

        window.set_cursor_visible(config.cursor_visible);

        let graphics = GraphicsContext::new(&window, config.vsync);

        Ok(Self {
            should_exit: false,
            vsync: config.vsync,
            timer,
            window,
            input: InputContext::default(),
            graphics,
        })
    }

    pub(crate) fn take_should_exit(&mut self) -> bool {
        let should_exit = self.should_exit;
        self.should_exit = false;
        should_exit
    }
}
