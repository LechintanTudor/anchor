use crate::game::{GameErrorKind, GameResult};
use crate::window::WindowConfig;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[derive(Clone, Debug)]
pub(crate) struct WindowUpdate {
    pub width: u32,
    pub height: u32,
}

pub(crate) struct WindowContext {
    pub window: Window,
    pub next_update: Option<WindowUpdate>,
}

impl WindowContext {
    pub fn new(event_loop: &EventLoop<()>, config: WindowConfig) -> GameResult<Self> {
        let window = WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(PhysicalSize::new(config.size.0, config.size.1))
            .with_resizable(config.resizable)
            .build(event_loop)
            .map_err(|error| GameErrorKind::OsError(error).into_error())?;

        window.set_cursor_visible(config.cursor_visible);

        Ok(Self { window, next_update: None })
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            self.next_update = Some(WindowUpdate { width, height })
        }
    }
}
