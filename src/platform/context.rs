use crate::graphics::GraphicsContext;
use crate::input::InputContext;

pub use winit;
pub use winit::window::Window;

pub struct Context {
    pub(crate) should_exit: bool,
    pub(crate) window: Window,
    pub(crate) input: InputContext,
    pub(crate) graphics: GraphicsContext,
}

impl Context {
    pub fn new(window: Window) -> Self {
        let graphics = GraphicsContext::new(&window);

        Self { should_exit: false, window, input: Default::default(), graphics }
    }

    pub(crate) fn take_should_exit(&mut self) -> bool {
        let should_exit = self.should_exit;
        self.should_exit = false;
        should_exit
    }
}
