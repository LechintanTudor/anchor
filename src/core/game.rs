use crate::core;
use crate::core::Context;
use winit::event::VirtualKeyCode;

pub trait Game
where
    Self: Sized + 'static,
{
    fn on_window_resized(&mut self, _ctx: &mut Context, _width: u32, _height: u32) {}

    fn on_key_pressed(&mut self, ctx: &mut Context, key_code: VirtualKeyCode) {
        if key_code == VirtualKeyCode::Q {
            core::request_exit(ctx);
        }
    }

    fn on_key_released(&mut self, _ctx: &mut Context, _key_code: VirtualKeyCode) {}

    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, _ctx: &mut Context) {}
}

pub trait GameBuilder {
    type Game: Game;

    fn build(self, ctx: &mut Context) -> Self::Game;
}

impl<F, G> GameBuilder for F
where
    F: FnOnce(&mut Context) -> G,
    G: Game,
{
    type Game = G;

    fn build(self, ctx: &mut Context) -> Self::Game {
        self(ctx)
    }
}
