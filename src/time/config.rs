use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Configuration for game loop timers.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct TimeConfig {
    /// Target frames per second.
    pub target_fps: f64,
    /// Target fixed updates per second.
    pub target_tps: f64,
}

impl TimeConfig {
    /// Returns the target time interval between two consecutive frames.
    #[inline]
    pub fn frame_interval(&self) -> Duration {
        Duration::from_secs(1).div_f64(self.target_fps)
    }

    /// Returns the target time interval between two consecutive fixed updates (ticks).
    #[inline]
    pub fn fixed_update_interval(&self) -> Duration {
        Duration::from_secs(1).div_f64(self.target_tps)
    }
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self { target_fps: 60.0, target_tps: 60.0 }
    }
}
