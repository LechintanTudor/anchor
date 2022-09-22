use crate::core::{Context, FramePhase};

#[inline]
pub fn request_exit(ctx: &mut Context) {
    ctx.should_exit = true;
}

#[inline]
pub fn frame_phase(ctx: &Context) -> FramePhase {
    ctx.frame_phase
}

#[inline]
pub fn set_target_fps(ctx: &mut Context, target_fps: f64) {
    ctx.time.set_target_fps(target_fps)
}

#[inline]
pub fn set_target_tps(ctx: &mut Context, target_tps: f64) {
    ctx.time.set_target_tps(target_tps)
}
