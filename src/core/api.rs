use crate::core::{Context, GamePhase};

/// Signals to the game that exit was requested.
#[inline]
pub fn request_exit(ctx: &mut Context) {
    ctx.should_exit = true;
}

/// Returns the current [GamePhase].
#[inline]
pub fn game_phase(ctx: &Context) -> GamePhase {
    ctx.game_phase
}

/// Sets the target frames per second. For the framerate to be capped, vsync must be disabled.
#[inline]
pub fn set_target_fps(ctx: &mut Context, target_fps: f64) {
    ctx.time.set_target_fps(target_fps)
}

/// Sets the target fixed updates per second (ticks).
#[inline]
pub fn set_target_tps(ctx: &mut Context, target_tps: f64) {
    ctx.time.set_target_tps(target_tps)
}
