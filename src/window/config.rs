use serde::{Deserialize, Serialize};

/// Configuration for the application window.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct WindowConfig {
    /// Title of the window.
    pub title: String,
    /// Initial size of the window.
    pub size: (u32, u32),
    /// Whether the window is resizable.
    pub resizable: bool,
    /// Path to the window icon, if any.
    pub icon_path: String,
    /// Whether the cursor is visible when it hovers the window.
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
