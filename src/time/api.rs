use crate::platform::{Context, FramePhase};
use std::time::Duration;

#[inline]
pub fn delta(ctx: &Context) -> Duration {
    if ctx.frame_phase == FramePhase::FixedUpdate {
        ctx.timer.fixed_delta()
    } else {
        ctx.timer.delta()
    }
}

#[inline]
pub fn delta_f32(ctx: &Context) -> f32 {
    delta(ctx).as_secs_f32()
}

#[inline]
pub fn delta_f64(ctx: &Context) -> f64 {
    delta(ctx).as_secs_f64()
}

#[inline]
pub fn fixed_delta(ctx: &Context) -> Duration {
    ctx.timer.fixed_delta()
}

#[inline]
pub fn fixed_delta_f32(ctx: &Context) -> f32 {
    fixed_delta(ctx).as_secs_f32()
}

#[inline]
pub fn fixed_delta_f64(ctx: &Context) -> f64 {
    fixed_delta(ctx).as_secs_f64()
}
