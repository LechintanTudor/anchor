#[derive(Clone, Debug)]
pub struct Config {
    pub window_title: String,
    pub window_size: (u32, u32),
    pub vsync: bool,
    pub frames_per_second: f64,
    pub fixed_updates_per_second: f64,
    pub max_fixed_updates_per_frame: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: "Anchor Game".to_string(),
            window_size: (640, 480),
            vsync: true,
            frames_per_second: 60.0,
            fixed_updates_per_second: 60.0,
            max_fixed_updates_per_frame: 3.0,
        }
    }
}
