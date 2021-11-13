use std::borrow::Cow;

use winit::dpi::Size;

pub struct Config {
    window_title: Cow<'static, str>,
    window_size: Size,
}

impl Config {
    pub fn with_window_title<S>(self, window_title: S) -> Self
    where
        S: Into<Cow<'static, str>>,
    {
        Self {
            window_title: window_title.into(),
            ..self
        }
    }

    pub fn with_window_size<S>(self, window_size: S) -> Self
    where
        S: Into<Size>,
    {
        Self {
            window_size: window_size.into(),
            ..self
        }
    }

    pub fn window_title(&self) -> &str {
        &self.window_title
    }

    pub fn window_size(&self) -> Size {
        self.window_size
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window_title: Cow::Borrowed("ANCHOR"),
            window_size: Size::Logical((960.0, 540.0).into()),
        }
    }
}
