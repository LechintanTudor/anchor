use crate::core::{Context, GameResult};
use crate::graphics::Frame;
use crate::input::Key;

#[allow(unused_variables)]
pub trait Game
where
    Self: Sized + 'static,
{
    fn on_window_resized(&mut self, ctx: &mut Context, width: u32, height: u32) {}

    fn on_key_pressed(&mut self, ctx: &mut Context, key: Key) {}

    fn on_key_released(&mut self, ctx: &mut Context, key: Key) {}

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<Frame> {
        Ok(Frame::default())
    }
}

pub trait GameBuilder {
    type Game: Game;

    fn build(self, ctx: &mut Context) -> GameResult<Self::Game>;
}

impl<F, G> GameBuilder for F
where
    F: FnOnce(&mut Context) -> GameResult<G>,
    G: Game,
{
    type Game = G;

    fn build(self, ctx: &mut Context) -> GameResult<Self::Game> {
        self(ctx)
    }
}
