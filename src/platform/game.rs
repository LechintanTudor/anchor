use crate::graphics::{self, Color};
use crate::input::Key;
use crate::platform::{Context, GameError, GameErrorOrigin, GameResult};
use glam::DVec2;
use winit::event::MouseButton;

#[allow(unused_variables)]
pub trait Game
where
    Self: Sized + 'static,
{
    fn on_window_resized(&mut self, ctx: &mut Context, width: u32, height: u32) {}

    fn on_exit_requested(&mut self, ctx: &mut Context) -> bool {
        true
    }

    fn on_error(&mut self, ctx: &mut Context, origin: GameErrorOrigin, error: GameError) -> bool {
        eprintln!("{:?}: {}", origin, error);
        true
    }

    fn on_key_pressed(&mut self, ctx: &mut Context, key: Key) {}

    fn on_key_released(&mut self, ctx: &mut Context, key: Key) {}

    fn on_mouse_button_pressed(&mut self, ctx: &mut Context, button: MouseButton) {}

    fn on_mouse_button_released(&mut self, ctx: &mut Context, button: MouseButton) {}

    fn on_mouse_motion(&mut self, ctx: &mut Context, delta: DVec2) {}

    fn on_cursor_moved(&mut self, ctx: &mut Context, position: DVec2) {}

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::display(ctx, Color::BLACK, &mut []);
        Ok(())
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
