use crate::game::Context;
use crate::input::{KeyCode, Modifiers, MouseButton};
use glam::DVec2;

/// Returns the state of the keyboard modifiers.
#[inline]
pub fn modifiers(ctx: &Context) -> Modifiers {
    ctx.input.modifiers
}

/// Returns whether the given key is pressed.
#[inline]
pub fn is_key_pressed(ctx: &Context, key: KeyCode) -> bool {
    ctx.input.keyboard.is_key_pressed(key)
}

/// Returns whether the given key was pressed this frame.
#[inline]
pub fn was_key_just_pressed(ctx: &Context, key: KeyCode) -> bool {
    ctx.input.keyboard.was_key_just_pressed(key)
}

/// Returns whether the given key was released this frame.
#[inline]
pub fn was_key_just_released(ctx: &Context, key: KeyCode) -> bool {
    ctx.input.keyboard.was_key_just_released(key)
}

/// Returns whether the given mouse button is pressed.
#[inline]
pub fn is_mouse_button_pressed(ctx: &Context, button: MouseButton) -> bool {
    ctx.input.mouse.is_button_pressed(button)
}

/// Returns whether the given mouse button was just pressed.
#[inline]
pub fn was_mouse_button_just_pressed(ctx: &Context, button: MouseButton) -> bool {
    ctx.input.mouse.was_button_just_pressed(button)
}

/// Returns whether the given mouse button was just released.
#[inline]
pub fn was_mouse_button_just_released(ctx: &Context, button: MouseButton) -> bool {
    ctx.input.mouse.was_button_just_released(button)
}

/// Returns the cursor position if the cursor hovers the window.
#[inline]
pub fn cursor_position(ctx: &Context) -> Option<DVec2> {
    ctx.input.cursor.position()
}

/// Returns the last detected cursor position.
#[inline]
pub fn last_cursor_position(ctx: &Context) -> DVec2 {
    ctx.input.cursor.last_position
}

/// Returns whether the cursor hovers the window.
#[inline]
pub fn cursor_hovers_window(ctx: &Context) -> bool {
    ctx.input.cursor.hovers_window
}
