use fontdue::{Font as FontdueFont, FontSettings};
use std::error::Error;
use std::fmt;
use std::sync::Arc;

#[derive(Clone, Copy, Debug)]
pub struct FontError {
    message: &'static str,
}

impl Error for FontError {}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message)
    }
}

#[derive(Clone, Debug)]
pub struct Font(Arc<fontdue::Font>);

impl Font {
    fn new(data: &[u8]) -> Result<Self, FontError> {
        match FontdueFont::from_bytes(data, FontSettings::default()) {
            Ok(font) => Ok(Font(Arc::new(font))),
            Err(message) => Err(FontError { message }),
        }
    }
}
