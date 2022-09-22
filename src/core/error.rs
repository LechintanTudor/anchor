use glyph_brush::ab_glyph::InvalidFont as FontError;
use image::ImageError;
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use winit::error::OsError;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug)]
struct GameErrorData {
    kind: GameErrorKind,
    path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct GameError(Box<GameErrorData>);

impl GameError {
    #[inline]
    pub fn new(kind: GameErrorKind, path: Option<PathBuf>) -> Self {
        Self(Box::new(GameErrorData { kind, path }))
    }

    #[inline]
    pub fn kind(&self) -> &GameErrorKind {
        &self.0.kind
    }

    #[inline]
    pub fn path(&self) -> Option<&Path> {
        self.0.path.as_deref()
    }
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.0.kind {
            GameErrorKind::OsError(e) => Some(e),
            GameErrorKind::IoError(e) => Some(e),
            GameErrorKind::ImageError(e) => Some(e),
            GameErrorKind::FontError(e) => Some(e),
            GameErrorKind::OtherError(e) => Some(Box::as_ref(e) as _),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = self.0.path.as_ref() {
            writeln!(f, "error originating from file \"{}\"", path.display())?;
        }

        writeln!(f, "{}", self.0.kind)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum GameErrorKind {
    OsError(OsError),
    IoError(IoError),
    ImageError(ImageError),
    FontError(FontError),
    OtherError(Box<dyn Error + Send + Sync + 'static>),
}

impl fmt::Display for GameErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OsError(e) => fmt::Display::fmt(e, f),
            Self::IoError(e) => fmt::Display::fmt(e, f),
            Self::ImageError(e) => fmt::Display::fmt(e, f),
            Self::FontError(e) => fmt::Display::fmt(e, f),
            Self::OtherError(e) => fmt::Display::fmt(e, f),
        }
    }
}

impl GameErrorKind {
    #[inline]
    pub fn into_error(self) -> GameError {
        GameError::new(self, None)
    }

    pub fn into_error_with_path<P>(self, path: P) -> GameError
    where
        P: Into<PathBuf>,
    {
        GameError::new(self, Some(path.into()))
    }
}
