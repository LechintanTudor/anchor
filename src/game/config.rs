use crate::graphics::GraphicsConfig;
use crate::time::TimeConfig;
use crate::window::WindowConfig;
use serde::{Deserialize, Serialize};

/// Groups together configurations for the various modules that make up the crate.
#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Configuration for the [window](crate::window) module.
    pub window: WindowConfig,
    /// Configuration for the [time](crate::time) module.
    pub time: TimeConfig,
    /// Configuration for the [graphics](crate::graphics) module.
    pub graphics: GraphicsConfig,
}
