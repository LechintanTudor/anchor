use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct TimeConfig {
    pub target_fps: f64,
    pub target_tps: f64,
}

impl TimeConfig {
    #[inline]
    pub fn frame_interval(&self) -> Duration {
        Duration::from_secs(1).div_f64(self.target_fps)
    }

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
