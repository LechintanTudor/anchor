mod config;
mod context;
mod error;
mod fps_limiter;
mod game;
mod game_loop;

pub(crate) use self::fps_limiter::*;
pub(crate) use self::game_loop::*;

pub use self::config::*;
pub use self::context::*;
pub use self::error::*;
pub use self::game::*;

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
pub fn request_exit(ctx: &mut Context) {
    ctx.should_exit = true;
}
