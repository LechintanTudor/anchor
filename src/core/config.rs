#[derive(Clone, Debug)]
pub struct Config {
    pub window_title: String,
    pub window_size: (u32, u32),
    pub cursor_visible: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: "Anchor Game".to_string(),
            window_size: (640, 480),
            cursor_visible: true,
        }
    }
}
