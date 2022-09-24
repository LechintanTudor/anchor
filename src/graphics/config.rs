use serde::{Deserialize, Serialize};

/// Configurations for the graphics pipeline.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GraphicsConfig {
    /// Whether to enable vsync.
    pub vsync: bool,
    /// Whether to enable multisampling.
    pub multisample: bool,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self { vsync: true, multisample: false }
    }
}
