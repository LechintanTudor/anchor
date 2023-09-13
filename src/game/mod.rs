mod config;
mod context;
mod error;

use crate::graphics::Canvas;

pub use self::config::*;
pub use self::context::*;
pub use self::error::*;

use glam::UVec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShouldExit {
    No,
    Yes,
}

impl ShouldExit {
    pub fn should_exit(&self) -> bool {
        matches!(self, Self::Yes)
    }
}

#[allow(unused_variables)]
pub trait Game {
    fn on_init(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn fixed_update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = Canvas::new(ctx);
        canvas.present();
        Ok(())
    }

    fn on_exit_request(&mut self, ctx: &mut Context) -> ShouldExit {
        ShouldExit::Yes
    }

    fn on_window_resize(&mut self, ctx: &mut Context, size: UVec2) {
        // Empty
    }

    fn on_exit(&mut self, ctx: &mut Context) {
        // Empty
    }

    fn handle_error(&mut self, ctx: &mut Context, error: GameError) -> ShouldExit {
        eprintln!("{error}");
        ShouldExit::Yes
    }
}

pub trait GameBuilder: Sized {
    type Game: Game;

    fn build_game(self, ctx: &mut Context) -> GameResult<Self::Game>;
}

impl<F, G> GameBuilder for F
where
    F: FnOnce(&mut Context) -> GameResult<G>,
    G: Game,
{
    type Game = G;

    fn build_game(self, ctx: &mut Context) -> GameResult<Self::Game> {
        self(ctx)
    }
}
