use crate::core::{Context, GamePhase};
use std::time::Duration;

/// Returns the time since the application was started.
#[inline]
pub fn since_start(ctx: &Context) -> Duration {
    ctx.time.since_start()
}

/// Returns the duration of the last frame.
/// If called inside [fixed_update](crate::core::Game::fixed_update), it returns the value of
/// [fixed_delta].
#[inline]
pub fn delta(ctx: &Context) -> Duration {
    if ctx.game_phase == GamePhase::FixedUpdate {
        ctx.time.fixed_delta()
    } else {
        ctx.time.delta()
    }
}

/// Returns the duration of the last frame as seconds.
/// If called inside [fixed_update](crate::core::Game::fixed_update), it returns the value of
/// [fixed_delta_f32].
#[inline]
pub fn delta_f32(ctx: &Context) -> f32 {
    delta(ctx).as_secs_f32()
}

/// Returns the duration of the last frame as seconds.
/// If called inside [fixed_update](crate::core::Game::fixed_update), it returns the value of
/// [fixed_delta_f64].
#[inline]
pub fn delta_f64(ctx: &Context) -> f64 {
    delta(ctx).as_secs_f64()
}

/// Returns the target interval between two fixed updates.
#[inline]
pub fn fixed_delta(ctx: &Context) -> Duration {
    ctx.time.fixed_delta()
}

/// Returns the target interval between two fixed updates as seconds.
#[inline]
pub fn fixed_delta_f32(ctx: &Context) -> f32 {
    fixed_delta(ctx).as_secs_f32()
}

/// Returns the target interval between two fixed updates as seconds.
#[inline]
pub fn fixed_delta_f64(ctx: &Context) -> f64 {
    fixed_delta(ctx).as_secs_f64()
}

/// Returns the interpolation factor for the last two frames. (accumulated unused time before a
/// fixed update divided by the target fixed update interval).
#[inline]
pub fn alpha_f32(ctx: &Context) -> f32 {
    ctx.time.frame_duration_accumulator().as_secs_f32() / ctx.time.fixed_delta().as_secs_f32()
}

/// Returns the interpolation factor for the last two frames. (accumulated unused time before a
/// fixed update divided by the target fixed update interval).
#[inline]
pub fn alpha_f64(ctx: &Context) -> f64 {
    ctx.time.frame_duration_accumulator().as_secs_f64() / ctx.time.fixed_delta().as_secs_f64()
}
