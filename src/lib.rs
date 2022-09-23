#![allow(clippy::module_inception)]

pub(crate) mod utils;

pub mod core;
pub mod graphics;
pub mod input;
pub mod time;
pub mod window;

use crate::core::{Config, GameBuilder, GameResult};

pub use {glam, wgpu, winit};

pub fn run<G>(config: Config, game_builder: G) -> GameResult
where
    G: GameBuilder,
{
    core::run(config, game_builder)
}
