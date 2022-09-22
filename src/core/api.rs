use crate::core::{Config, Context, FramePhase, GameErrorKind, GameResult, Window};
use std::path::Path;

pub fn load_config<P>(path: P) -> GameResult<Config>
where
    P: AsRef<Path>,
{
    fn inner(path: &Path) -> GameResult<Config> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| GameErrorKind::IoError(e).into_error().with_source_path(path))?;

        let config = ron::from_str::<Config>(&data)
            .map_err(|e| GameErrorKind::RonError(e).into_error().with_source_path(path))?;

        Ok(config)
    }

    inner(path.as_ref())
}

#[inline]
pub fn request_exit(ctx: &mut Context) {
    ctx.should_exit = true;
}

#[inline]
pub fn window(ctx: &Context) -> &Window {
    &ctx.window
}

#[inline]
pub fn set_cursor_visible(ctx: &Context, cursor_visible: bool) {
    ctx.window.set_cursor_visible(cursor_visible);
}

#[inline]
pub fn window_size(ctx: &Context) -> (u32, u32) {
    let size = ctx.window.inner_size();
    (size.width, size.height)
}

#[inline]
pub fn frame_phase(ctx: &Context) -> FramePhase {
    ctx.frame_phase
}
