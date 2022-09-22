use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct GraphicsConfig {
    pub vsync: bool,
    pub multisample: bool,
}

impl Default for GraphicsConfig {
    fn default() -> Self {
        Self { vsync: true, multisample: false }
    }
}
