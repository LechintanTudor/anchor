#![allow(dead_code)]

mod core;
mod game;
mod graphics;

use crate::core::Config;
use crate::game::Game;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    core::run(Config::default(), Game::new)
}
