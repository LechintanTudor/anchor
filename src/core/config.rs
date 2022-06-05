use winit::dpi::Size as WindowSize;

pub struct Config {
    pub(crate) window_title: String,
    pub(crate) window_size: WindowSize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: "Anchor Game".to_string(),
            window_size: WindowSize::Logical((640.0, 480.0).into()),
        }
    }
}

impl Config {
    pub fn window_title<S>(mut self, window_title: S) -> Self
    where
        S: Into<String>,
    {
        self.window_title = window_title.into();
        self
    }

    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_size = WindowSize::Physical((width, height).into());
        self
    }

    pub fn logical_window_size(mut self, width: f64, height: f64) -> Self {
        self.window_size = WindowSize::Logical((width, height).into());
        self
    }
}
