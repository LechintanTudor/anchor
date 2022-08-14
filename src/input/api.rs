use crate::input::Key;
use crate::platform::Context;
use glam::DVec2;

#[inline]
pub fn is_key_pressed(ctx: &Context, key: Key) -> bool {
    ctx.input.keyboard.is_key_pressed(key)
}

#[inline]
pub fn was_key_just_pressed(ctx: &Context, key: Key) -> bool {
    ctx.input.keyboard.was_key_just_pressed(key)
}

#[inline]
pub fn was_key_just_released(ctx: &Context, key: Key) -> bool {
    ctx.input.keyboard.was_key_just_released(key)
}

#[inline]
pub fn pressed_keys(ctx: &Context) -> &[Key] {
    ctx.input.keyboard.pressed_keys()
}

#[inline]
pub fn just_pressed_keys(ctx: &Context) -> &[Key] {
    ctx.input.keyboard.just_pressed_keys()
}

#[inline]
pub fn just_released_keys(ctx: &Context) -> &[Key] {
    ctx.input.keyboard.just_released_keys()
}

#[inline]
pub fn cursor_position(ctx: &Context) -> Option<DVec2> {
    ctx.input.cursor.position()
}

#[inline]
pub fn last_cursor_position(ctx: &Context) -> DVec2 {
    ctx.input.cursor.last_position
}

#[inline]
pub fn cursor_hovers_window(ctx: &Context) -> bool {
    ctx.input.cursor.hovers_window
}
