//! Provides functions for querying game loop timers.

mod api;
mod config;

pub(crate) mod context;

pub(crate) use self::context::*;

pub use self::api::*;
pub use self::config::*;
