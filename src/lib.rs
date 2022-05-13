pub(crate) mod utils;

pub mod core;
pub mod graphics;
pub mod input;

use crate::core::{Config, GameBuilder, GameResult};

pub fn run<G>(config: Config, game_builder: G) -> GameResult<()>
where
    G: GameBuilder,
{
    core::run(config, game_builder)
}
