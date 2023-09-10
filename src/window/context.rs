use crate::game::GameResult;
use crate::window::WindowConfig;
use glam::UVec2;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

#[derive(Clone, Debug)]
pub(crate) struct WindowUpdate {
    pub window_size: UVec2,
}

pub(crate) struct WindowContext {
    pub window: Window,
    pub window_size: UVec2,
    pub next_update: Option<WindowUpdate>,
}

impl WindowContext {
    pub fn new(event_loop: &EventLoop<()>, config: WindowConfig) -> GameResult<Self> {
        let window = WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(PhysicalSize::<u32>::from(config.size))
            .with_resizable(config.resizable)
            .build(event_loop)?;

        window.set_cursor_visible(config.cursor_visible);

        let window_size = window.inner_size();
        let window_size = UVec2::new(window_size.width.max(1), window_size.height.max(1));

        Ok(Self { window, window_size, next_update: None })
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            self.next_update = Some(WindowUpdate { window_size: UVec2::new(width, height) })
        }
    }
}
