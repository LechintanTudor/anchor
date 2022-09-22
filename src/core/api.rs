use crate::core::{Context, FramePhase};

#[inline]
pub fn request_exit(ctx: &mut Context) {
    ctx.should_exit = true;
}

#[inline]
pub fn frame_phase(ctx: &Context) -> FramePhase {
    ctx.frame_phase
}
