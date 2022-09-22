use crate::graphics::GraphicsConfig;
use crate::time::TimeConfig;
use crate::window::WindowConfig;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    pub window: WindowConfig,
    pub time: TimeConfig,
    pub graphics: GraphicsConfig,
}
