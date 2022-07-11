pub(crate) mod utils;

use crate::core::{Config, GameBuilder, GameResult};

pub mod core;
pub mod graphics;
pub mod input;

pub use glam;
pub use winit;
pub use wgpu;

pub fn run<G>(config: Config, game_builder: G) -> GameResult<()>
where
    G: GameBuilder,
{
    core::run(config, game_builder)
}
