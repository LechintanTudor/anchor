use winit::window::Window;

pub struct Context {
    pub(crate) window: Window,
    pub(crate) should_exit: bool,
}

pub(crate) mod api {
    use super::*;

    #[inline]
    pub fn request_exit(ctx: &mut Context) {
        ctx.should_exit = true;
    }
}
