use ggez::error::GameError;
use ggez::event::EventHandler;
use ggez::graphics::Color;
use ggez::{graphics, Context};
use sparsey::World;

pub const TITLE: &'static str = "Anchor";
pub const AUTHOR: &'static str = "2DGamez";
pub const WIDTH: f32 = 960.0;
pub const HEIGHT: f32 = 540.0;

pub struct Game {
    _world: World,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Self {
        Self {
            _world: World::default(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        graphics::clear(ctx, Color::WHITE);
        graphics::present(ctx)?;
        Ok(())
    }
}
