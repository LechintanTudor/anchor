use crate::core::{Context, FramePhase};
use std::time::Duration;

#[inline]
pub fn since_start(ctx: &Context) -> Duration {
    ctx.time.since_start()
}

#[inline]
pub fn delta(ctx: &Context) -> Duration {
    if ctx.frame_phase == FramePhase::FixedUpdate {
        ctx.time.fixed_delta()
    } else {
        ctx.time.delta()
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
    ctx.time.fixed_delta()
}

#[inline]
pub fn fixed_delta_f32(ctx: &Context) -> f32 {
    fixed_delta(ctx).as_secs_f32()
}

#[inline]
pub fn fixed_delta_f64(ctx: &Context) -> f64 {
    fixed_delta(ctx).as_secs_f64()
}

#[inline]
pub fn alpha_f32(ctx: &Context) -> f32 {
    ctx.time.frame_duration_accumulator().as_secs_f32() / ctx.time.fixed_delta().as_secs_f32()
}

#[inline]
pub fn alpha_f64(ctx: &Context) -> f64 {
    ctx.time.frame_duration_accumulator().as_secs_f64() / ctx.time.fixed_delta().as_secs_f64()
}
