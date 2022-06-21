use image::ImageError;
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use winit::error::OsError;

pub type GameResult<T> = Result<T, Box<GameError>>;

#[derive(Debug)]
pub enum GameError {
    WindowError(OsError),
    FileError(FileError),
    ImageError(ImageError),
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::WindowError(e) => Some(e),
            Self::FileError(e) => Some(e),
            Self::ImageError(e) => Some(e),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WindowError(e) => fmt::Display::fmt(e, f),
            Self::FileError(e) => fmt::Display::fmt(e, f),
            Self::ImageError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl From<FileError> for GameError {
    fn from(error: FileError) -> Self {
        Self::FileError(error)
    }
}

impl From<ImageError> for GameError {
    fn from(error: ImageError) -> Self {
        Self::ImageError(error)
    }
}

#[derive(Debug)]
pub struct FileError {
    path: PathBuf,
    error: IoError,
}

impl FileError {
    pub(crate) fn new(path: PathBuf, error: IoError) -> Self {
        Self { path, error }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn error(&self) -> &IoError {
        &self.error
    }
}

impl Error for FileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\": {}", self.path.display(), self.error)
    }
}
