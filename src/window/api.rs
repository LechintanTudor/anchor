use crate::core::{Context, GameErrorKind, GameResult};
use crate::graphics::Image;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::window::{Icon, Window};

#[inline]
pub fn window(ctx: &Context) -> &Window {
    &ctx.window
}

#[inline]
pub fn set_title(ctx: &Context, title: &str) {
    ctx.window.set_title(title);
}

#[inline]
pub fn set_position(ctx: &Context, x: u32, y: u32) {
    ctx.window.set_outer_position(PhysicalPosition::new(x, y));
}

#[inline]
pub fn set_size(ctx: &Context, width: u32, height: u32) {
    ctx.window.set_inner_size(PhysicalSize::new(width, height));
}

#[inline]
pub fn set_resizable(ctx: &Context, resizable: bool) {
    ctx.window.set_resizable(resizable)
}

#[inline]
pub fn set_cursor_visible(ctx: &Context, cursor_visible: bool) {
    ctx.window.set_cursor_visible(cursor_visible)
}

#[inline]
pub fn set_icon(ctx: &Context, image: Option<Image>) -> GameResult<()> {
    let icon = image.map(create_icon).transpose()?;
    ctx.window.set_window_icon(icon);
    Ok(())
}

#[inline]
pub fn size(ctx: &Context) -> (u32, u32) {
    ctx.window.inner_size().into()
}

#[inline]
pub fn scale_factor(ctx: &Context) -> f64 {
    ctx.window.scale_factor()
}

pub(crate) fn create_icon(image: Image) -> GameResult<Icon> {
    let width = image.width();
    let height = image.height();
    let data = image.into_data();

    Icon::from_rgba(data, width, height)
        .map_err(|e| GameErrorKind::OtherError(Box::new(e)).into_error())
}
