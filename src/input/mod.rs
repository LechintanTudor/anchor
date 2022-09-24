#![forbid(missing_docs)]

//! Provides functions for querying the state of input devices. 

mod api;
mod context;
mod cursor;
mod keyboard;
mod mouse;
mod types;

pub(crate) use self::context::*;
pub(crate) use self::cursor::*;
pub(crate) use self::keyboard::*;
pub(crate) use self::mouse::*;

pub use self::api::*;
pub use self::types::*;
