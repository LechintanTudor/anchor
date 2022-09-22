use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct WindowConfig {
    pub title: String,
    pub size: (u32, u32),
    pub resizable: bool,
    pub icon_path: String,
    pub cursor_visible: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Anchor Game".to_string(),
            size: (800, 450),
            resizable: true,
            icon_path: String::new(),
            cursor_visible: true,
        }
    }
}
