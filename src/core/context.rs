pub use winit;
pub use winit::window::Window;

use crate::graphics::GraphicsContext;
use crate::input::InputContext;

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
}
