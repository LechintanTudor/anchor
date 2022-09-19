#![allow(clippy::module_inception)]

pub(crate) mod utils;

pub mod graphics;
pub mod input;
pub mod platform;
pub mod time;

use crate::platform::{Config, GameBuilder, GameResult};

pub use {glam, wgpu, winit};

pub fn run<G>(config: Config, game_builder: G) -> GameResult<()>
where
    G: GameBuilder,
{
    platform::run(config, game_builder)
}
