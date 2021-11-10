mod game;

use game::Game;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::{event, ContextBuilder};

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new(game::TITLE, game::AUTHOR)
        .window_setup(WindowSetup::default().title(game::TITLE))
        .window_mode(WindowMode::default().dimensions(game::WIDTH, game::HEIGHT))
        .build()
        .unwrap();

    let game = Game::new(&mut ctx);
    event::run(ctx, event_loop, game);
}
