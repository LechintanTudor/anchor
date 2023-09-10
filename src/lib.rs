#![allow(clippy::module_inception)]

pub mod game;
pub mod graphics;
pub mod input;
pub mod time;
pub mod window;

use crate::game::{Config, GameBuilder, GameResult};

pub use {anyhow, glam, wgpu, winit};

/// Creates the game and starts the event loop.
pub fn run<G>(config: Config, game_builder: G) -> GameResult
where
    G: GameBuilder,
{
    game::run(config, game_builder)
}
