use crate::input::Keyboard;

pub use winit;
pub use winit::window::Window;

pub struct Context {
    pub(crate) window: Window,
    pub(crate) should_exit: bool,
    pub(crate) keyboard: Keyboard,
}

impl Context {
    pub fn new(window: Window) -> Self {
        Self { window, should_exit: false, keyboard: Default::default() }
    }
}
