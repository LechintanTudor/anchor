use crate::core::{Context, GamePhase};

#[inline]
pub fn request_exit(ctx: &mut Context) {
    ctx.should_exit = true;
}

#[inline]
pub fn game_phase(ctx: &Context) -> GamePhase {
    ctx.game_phase
}

#[inline]
pub fn set_target_fps(ctx: &mut Context, target_fps: f64) {
    ctx.time.set_target_fps(target_fps)
}

#[inline]
pub fn set_target_tps(ctx: &mut Context, target_tps: f64) {
    ctx.time.set_target_tps(target_tps)
}
