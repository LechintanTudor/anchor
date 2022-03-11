use winit::window::Window;

pub struct Context {
    pub(crate) window: Window,
    pub(crate) should_exit: bool,
}

#[inline]
pub fn window(ctx: &Context) -> &Window {
    &ctx.window
}

#[inline]
pub fn request_exit(ctx: &mut Context) {
    ctx.should_exit = true;
}
