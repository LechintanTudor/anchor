pub mod game;
pub mod graphics;

use crate::game::{Config, Context, Game, GameBuilder, GameResult};
use glam::UVec2;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
pub use {glam, wgpu, winit};

pub fn run<G>(game_builder: G, config: Config) -> GameResult
where
    G: GameBuilder,
{
    let event_loop = EventLoop::new()?;
    let mut ctx = Context::new(&event_loop, &config)?;
    let mut game = game_builder.build_game(&mut ctx)?;

    event_loop.run(move |event, event_loop| {
        let ctx = &mut ctx;
        let game = &mut game;

        match event {
            Event::NewEvents(StartCause::Init) => {
                if let Err(error) = game.on_init(ctx) {
                    if game.handle_error(ctx, error).should_exit() {
                        event_loop.exit();
                    }
                }
            }
            Event::NewEvents(StartCause::Poll) => {
                if let Err(error) = game.update(ctx) {
                    if game.handle_error(ctx, error).should_exit() {
                        event_loop.exit();
                        return;
                    }
                }

                ctx.graphics.window().request_redraw();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => {
                        if game.on_exit_request(ctx).should_exit() {
                            event_loop.exit();
                        }
                    }
                    WindowEvent::Resized(size) => {
                        let size = UVec2::new(size.width, size.height);
                        ctx.graphics.resize_surface(size);
                        game.on_window_resize(ctx, size);
                    }
                    WindowEvent::RedrawRequested => {
                        if let Err(error) = game.draw(ctx) {
                            if game.handle_error(ctx, error).should_exit() {
                                event_loop.exit();
                            }
                        }
                    }
                    _ => (),
                }
            }
            Event::LoopExiting => {
                game.on_exit(ctx);
            }
            _ => (),
        }
    })?;

    Ok(())
}
