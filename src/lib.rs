pub(crate) mod utils;

use crate::core::{Config, GameBuilder, GameResult};

pub mod core;
pub mod input;

pub fn run<G>(config: Config, game_builder: G) -> GameResult<()>
where
    G: GameBuilder,
{
    core::run(config, game_builder)
}
