use std::error::Error;
use std::fmt;
use wgpu::RequestDeviceError;
use winit::error::OsError;

pub type GameResult<T> = Result<T, GameError>;

#[derive(Debug)]
pub enum GameError {
    CannotCreateWindow(OsError),
    NoGraphicsAdaptersFound,
    CannotConnectToGraphicsDevice(RequestDeviceError),
}

impl Error for GameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::CannotCreateWindow(e) => Some(e),
            Self::NoGraphicsAdaptersFound => None,
            Self::CannotConnectToGraphicsDevice(e) => Some(e),
        }
    }
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotCreateWindow(e) => write!(f, "Cannot create window: {}", e),
            Self::NoGraphicsAdaptersFound => write!(f, "No graphics adapters found"),
            Self::CannotConnectToGraphicsDevice(e) => {
                write!(f, "Cannot connect to graphics device: {}", e)
            }
        }
    }
}
