#![allow(clippy::module_inception)]
#![allow(clippy::wrong_self_convention)]

pub mod game;
pub mod graphics;
pub mod time;

pub use {anyhow, glam, wgpu, winit};

use crate::game::{Config, Context, Game, GameBuilder, GameResult, ShouldExit};
use crate::time::GamePhase;
use glam::{DVec2, UVec2};
use std::thread;
use winit::event::{Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;

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
                if !ctx.graphics.vsync() {
                    while !ctx.time.frame_ended() {
                        thread::yield_now();
                    }
                }

                ctx.time.start_frame();

                if update(game, ctx).should_exit() {
                    event_loop.exit();
                    return;
                }

                ctx.graphics.window().request_redraw();
                ctx.time.phase = GamePhase::Input;
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
                    WindowEvent::KeyboardInput {
                        event,
                        is_synthetic,
                        ..
                    } => {
                        game.on_key_event(ctx, event, is_synthetic);
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        game.on_mouse_event(ctx, state.is_pressed(), button);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        let position = DVec2::new(position.x, position.y);
                        game.on_cursor_move(ctx, position);
                    }
                    WindowEvent::RedrawRequested => {
                        if ctx.graphics.update_surface_texture() {
                            ctx.time.phase = GamePhase::Draw;
                            if let Err(error) = game.draw(ctx) {
                                if game.handle_error(ctx, error).should_exit() {
                                    event_loop.exit();
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
            Event::LoopExiting => {
                ctx.time.phase = GamePhase::Exit;
                game.on_exit(ctx);
            }
            _ => (),
        }
    })?;

    Ok(())
}

fn update<G>(game: &mut G, ctx: &mut Context) -> ShouldExit
where
    G: Game,
{
    ctx.time.phase = GamePhase::Update;
    if let Err(error) = game.update(ctx) {
        if game.handle_error(ctx, error).should_exit() {
            return ShouldExit::Yes;
        }
    }

    ctx.time.phase = GamePhase::FixedUpdate;
    while ctx.time.fixed_update() {
        if let Err(error) = game.fixed_update(ctx) {
            if game.handle_error(ctx, error).should_exit() {
                return ShouldExit::Yes;
            }
        }
    }

    ctx.time.phase = GamePhase::LateUpdate;
    if let Err(error) = game.late_update(ctx) {
        if game.handle_error(ctx, error).should_exit() {
            return ShouldExit::Yes;
        }
    }

    ShouldExit::No
}
