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
    source_path_chain: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct GameError(Box<GameErrorData>);

impl GameError {
    #[inline]
    pub fn new(kind: GameErrorKind) -> Self {
        Self(Box::new(GameErrorData { kind, source_path_chain: vec![] }))
    }

    pub fn with_source_path<P>(mut self, path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        self.0.source_path_chain.push(path.into());
        self
    }

    #[inline]
    pub fn kind(&self) -> &GameErrorKind {
        &self.0.kind
    }

    #[inline]
    pub fn source_path_chain(&self) -> PathChainIter {
        PathChainIter(self.0.source_path_chain.iter())
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
        writeln!(f, "{}", self.0.kind)?;

        if let Some((path, parent_paths)) = self.0.source_path_chain.split_first() {
            writeln!(f, "    error originating from '{}'", path.display())?;

            for parent_path in parent_paths {
                writeln!(f, "        referenced in '{}'", parent_path.display())?;
            }
        }

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
        GameError::new(self)
    }
}

pub struct PathChainIter<'a>(std::slice::Iter<'a, PathBuf>);

impl<'a> Iterator for PathChainIter<'a> {
    type Item = &'a Path;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(PathBuf::as_path)
    }
}
