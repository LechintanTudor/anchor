use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub window_title: String,
    pub window_size: (u32, u32),
    pub cursor_visible: bool,
    pub vsync: bool,
    pub sample_count: u32,
    pub target_frames_per_second: f64,
    pub target_fixed_updates_per_second: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: "Anchor Game".to_string(),
            window_size: (640, 480),
            cursor_visible: true,
            vsync: true,
            sample_count: 1,
            target_frames_per_second: 60.0,
            target_fixed_updates_per_second: 60.0,
        }
    }
}
