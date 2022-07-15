use glyph_brush::ab_glyph::InvalidFont as FontError;
use image::ImageError;
use ron::error::Error as RonError;
use std::error::Error;
use std::fmt;
use std::io::Error as IoError;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use winit::error::OsError;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug)]
struct GameErrorData {
    kind: GameErrorKind,
    path: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct GameError(Arc<GameErrorData>);

impl GameError {
    #[inline]
    pub fn new(kind: GameErrorKind, path: Option<PathBuf>) -> Self {
        Self(Arc::new(GameErrorData { kind, path }))
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

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.path.as_deref() {
            Some(path) => {
                write!(f, "error originating from file \"{}\": {}", path.display(), self.0.kind)
            }
            None => fmt::Display::fmt(&self.0.kind, f),
        }
    }
}

#[derive(Debug)]
pub enum GameErrorKind {
    OsError(OsError),
    IoError(IoError),
    RonError(RonError),
    ImageError(ImageError),
    FontError(FontError),
    OtherError(Box<dyn Error + Send + Sync + 'static>),
}

impl fmt::Display for GameErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OsError(e) => fmt::Display::fmt(e, f),
            Self::IoError(e) => fmt::Display::fmt(e, f),
            Self::RonError(e) => fmt::Display::fmt(e, f),
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
