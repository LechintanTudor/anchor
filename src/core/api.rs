use crate::core::{Context, Window};

#[inline]
pub fn request_exit(ctx: &mut Context) {
    ctx.should_exit = true;
}

#[inline]
pub fn window(ctx: &Context) -> &Window {
    &ctx.window
}

#[inline]
pub fn window_size(ctx: &Context) -> (u32, u32) {
    let size = ctx.window.inner_size();
    (size.width, size.height)
}

#[inline]
pub fn set_cursor_visible(ctx: &Context, cursor_visible: bool) {
    ctx.window.set_cursor_visible(cursor_visible)
}
