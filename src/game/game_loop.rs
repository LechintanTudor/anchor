use std::thread;

use crate::game::{Config, Context, Game, GameBuilder, GamePhase, GameResult};
use glam::{DVec2, UVec2};
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, ElementState, Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;

#[must_use]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShouldExit {
    No,
    Yes,
}

impl ShouldExit {
    pub fn should_exit(&self) -> bool {
        matches!(self, ShouldExit::Yes)
    }
}

pub(crate) fn run<G>(config: Config, game_builder: G) -> GameResult
where
    G: GameBuilder,
{
    let event_loop = EventLoop::new()?;
    let mut ctx = Context::new(&event_loop, config)?;
    let mut game = game_builder.build(&mut ctx)?;

    event_loop.run(move |event, event_loop| {
        let game = &mut game;
        let ctx = &mut ctx;

        match event {
            Event::NewEvents(StartCause::Init) => {
                ctx.game_phase = GamePhase::Init;
                if let Err(error) = game.on_init(ctx) {
                    if game.handle_error(ctx, error).should_exit() {
                        event_loop.exit();
                    }
                }
            }
            Event::NewEvents(StartCause::Poll) => {
                if !ctx.graphics.vsync {
                    while !ctx.time.end_frame() {
                        thread::yield_now();
                    }
                }

                ctx.time.start_frame();
                ctx.game_phase = GamePhase::FrameStart;

                if ctx.take_should_exit().should_exit() && game.on_exit_request(ctx).should_exit() {
                    event_loop.exit();
                    return;
                }

                update_window(game, ctx);

                if update(game, ctx).should_exit() {
                    event_loop.exit();
                    return;
                }

                ctx.window.window.request_redraw();
                ctx.game_phase = GamePhase::Input;
            }
            Event::DeviceEvent { event, .. } => {
                on_device_event(game, ctx, event);
            }
            Event::WindowEvent { event, .. } => {
                if on_window_event(game, ctx, event).should_exit() {
                    event_loop.exit();
                }
            }
            Event::LoopExiting => {
                ctx.game_phase = GamePhase::Exit;
                game.on_exit(ctx);
            }
            _ => (),
        }
    })?;

    Ok(())
}

fn on_device_event(game: &mut impl Game, ctx: &mut Context, event: DeviceEvent) {
    if let DeviceEvent::MouseMotion { delta, .. } = event {
        let delta = DVec2::new(delta.0, delta.1);
        game.on_mouse_motion(ctx, delta);
    }
}

fn on_window_event(game: &mut impl Game, ctx: &mut Context, event: WindowEvent) -> ShouldExit {
    match event {
        WindowEvent::CloseRequested => {
            if game.on_exit_request(ctx).should_exit() {
                return ShouldExit::Yes;
            }
        }
        WindowEvent::Resized(size) => {
            on_window_resize(game, ctx, size.width, size.height, false);
        }
        WindowEvent::KeyboardInput { event, .. } => {
            match event.state {
                ElementState::Pressed => {
                    ctx.input.keyboard.on_key_pressed(event.physical_key);
                    game.on_key_press(ctx, event.physical_key);
                }
                ElementState::Released => {
                    ctx.input.keyboard.on_key_released(event.physical_key);
                    game.on_key_release(ctx, event.physical_key);
                }
            }
        }
        WindowEvent::ModifiersChanged(modifiers) => {
            ctx.input.modifiers = modifiers;
            game.on_modifiers_change(ctx, modifiers);
        }
        WindowEvent::MouseInput { state, button, .. } => {
            match state {
                ElementState::Pressed => {
                    ctx.input.mouse.on_button_pressed(button);
                    game.on_mouse_button_press(ctx, button);
                }
                ElementState::Released => {
                    ctx.input.mouse.on_button_released(button);
                    game.on_mouse_button_release(ctx, button);
                }
            }
        }
        WindowEvent::CursorEntered { .. } => {
            ctx.input.cursor.hovers_window = true;
        }
        WindowEvent::CursorLeft { .. } => {
            ctx.input.cursor.hovers_window = false;
        }
        WindowEvent::CursorMoved { position, .. } => {
            let position = DVec2::new(position.x, position.y);
            ctx.input.cursor.last_position = position;
            game.on_cursor_move(ctx, position);
        }
        WindowEvent::MouseWheel { delta, .. } => {
            game.on_scroll(ctx, delta.into());
        }
        WindowEvent::Focused(false) => {
            ctx.input.on_focus_lost();
        }
        WindowEvent::RedrawRequested => {
            if draw(game, ctx).should_exit() {
                return ShouldExit::Yes;
            }
        }
        _ => (),
    }

    ShouldExit::No
}

fn update_window(game: &mut impl Game, ctx: &mut Context) {
    let Some(update) = ctx.window.next_update.take() else {
        return;
    };

    let size = PhysicalSize::new(update.window_size.x, update.window_size.y);

    if let Some(size) = ctx.window.window.request_inner_size(size) {
        on_window_resize(game, ctx, size.width, size.height, true);
    }
}

fn on_window_resize(
    game: &mut impl Game,
    ctx: &mut Context,
    width: u32,
    height: u32,
    is_programatic: bool,
) {
    ctx.window.window_size = UVec2::new(width, height);
    ctx.graphics.on_window_resize(width, height);
    game.on_window_resize(ctx, width, height, is_programatic);
}

fn update(game: &mut impl Game, ctx: &mut Context) -> ShouldExit {
    ctx.game_phase = GamePhase::Update;
    if let Err(error) = game.update(ctx) {
        if game.handle_error(ctx, error).should_exit() {
            return ShouldExit::Yes;
        }
    }

    ctx.game_phase = GamePhase::FixedUpdate;
    while ctx.time.fixed_update() {
        if let Err(error) = game.fixed_update(ctx) {
            if game.handle_error(ctx, error).should_exit() {
                return ShouldExit::Yes;
            }
        }
    }

    ctx.game_phase = GamePhase::LateUpdate;
    if let Err(error) = game.late_update(ctx) {
        if game.handle_error(ctx, error).should_exit() {
            return ShouldExit::Yes;
        }
    }

    ShouldExit::No
}

fn draw(game: &mut impl Game, ctx: &mut Context) -> ShouldExit {
    ctx.game_phase = GamePhase::Draw;
    ctx.graphics.prepare();

    if let Err(error) = game.draw(ctx) {
        if game.handle_error(ctx, error).should_exit() {
            return ShouldExit::Yes;
        }
    }

    ctx.graphics.present();
    ShouldExit::No
}
