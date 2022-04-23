mod keyboard;

use crate::core::Context;

pub use self::keyboard::*;

#[inline]
pub fn is_key_pressed(ctx: &Context, key: Key) -> bool {
    ctx.keyboard.is_key_pressed(key)
}

#[inline]
pub fn was_key_just_pressed(ctx: &Context, key: Key) -> bool {
    ctx.keyboard.was_key_just_pressed(key)
}

#[inline]
pub fn was_key_just_released(ctx: &Context, key: Key) -> bool {
    ctx.keyboard.was_key_just_released(key)
}

#[inline]
pub fn pressed_keys(ctx: &Context) -> &[Key] {
    ctx.keyboard.pressed_keys()
}

#[inline]
pub fn just_pressed_keys(ctx: &Context) -> &[Key] {
    ctx.keyboard.just_pressed_keys()
}

#[inline]
pub fn just_released_keys(ctx: &Context) -> &[Key] {
    ctx.keyboard.just_released_keys()
}
