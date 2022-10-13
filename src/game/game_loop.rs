use crate::game::{Config, Context, Game, GameBuilder, GameError, GamePhase, GameResult};
use glam::DVec2;
use winit::event::{DeviceEvent, ElementState, Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub(crate) fn run<G>(config: Config, game_builder: G) -> GameResult
where
    G: GameBuilder,
{
    let event_loop = EventLoop::new();
    let mut ctx = Context::new(&event_loop, config)?;
    let mut game = game_builder.build(&mut ctx)?;

    event_loop.run(move |event, _event_loop, control_flow| {
        let game = &mut game;
        let ctx = &mut ctx;

        match event {
            Event::NewEvents(StartCause::Init) => {
                on_init(ctx, game, control_flow);
            }
            Event::NewEvents(_) => {
                on_frame_start(ctx, game, control_flow);
            }
            Event::DeviceEvent { event, .. } => {
                on_device_event(ctx, game, event);
            }
            Event::WindowEvent { event, .. } => {
                on_window_event(ctx, game, event, control_flow);
            }
            Event::MainEventsCleared => {
                on_update(ctx, game, control_flow);
            }
            Event::RedrawRequested(_) => {
                on_draw(ctx, game, control_flow);
            }
            Event::RedrawEventsCleared => {
                on_frame_end(ctx);
            }
            Event::LoopDestroyed => {
                on_destroy(ctx, game);
            }
            _ => (),
        }
    });
}

fn on_init(ctx: &mut Context, game: &mut impl Game, control_flow: &mut ControlFlow) {
    ctx.game_phase = GamePhase::Init;
    if let Err(error) = game.on_init(ctx) {
        handle_error(game, ctx, error, control_flow);
    }
}

fn on_destroy(ctx: &mut Context, game: &mut impl Game) {
    ctx.game_phase = GamePhase::Destroy;
    game.on_destroy(ctx);
}

fn on_frame_start(ctx: &mut Context, game: &mut impl Game, control_flow: &mut ControlFlow) {
    ctx.time.start_frame();
    ctx.game_phase = GamePhase::Input;

    if ctx.take_should_exit() && game.on_exit_request(ctx) {
        control_flow.set_exit();
    }
}

fn on_device_event(ctx: &mut Context, game: &mut impl Game, event: DeviceEvent) {
    if let DeviceEvent::MouseMotion { delta, .. } = event {
        let delta = DVec2::new(delta.0, delta.1);
        game.on_mouse_motion(ctx, delta);
    }
}

fn on_window_event(
    ctx: &mut Context,
    game: &mut impl Game,
    event: WindowEvent,
    control_flow: &mut ControlFlow,
) {
    match event {
        WindowEvent::CloseRequested => {
            if game.on_exit_request(ctx) {
                control_flow.set_exit();
            }
        }
        WindowEvent::Resized(size)
        | WindowEvent::ScaleFactorChanged { new_inner_size: &mut size, .. } => {
            ctx.graphics.on_window_resize(size.width, size.height);
            game.on_window_resize(ctx, size.width, size.height);
        }
        WindowEvent::KeyboardInput { input, .. } => {
            if let Some(key) = input.virtual_keycode {
                match input.state {
                    ElementState::Pressed => {
                        ctx.input.keyboard.on_key_pressed(key);
                        game.on_key_press(ctx, key);
                    }
                    ElementState::Released => {
                        ctx.input.keyboard.on_key_released(key);
                        game.on_key_release(ctx, key);
                    }
                }
            }
        }
        WindowEvent::ModifiersChanged(modifiers) => {
            ctx.input.modifiers = modifiers;
            game.on_modifiers_change(ctx, modifiers);
        }
        WindowEvent::MouseInput { state, button, .. } => match state {
            ElementState::Pressed => {
                ctx.input.mouse.on_button_pressed(button);
                game.on_mouse_button_press(ctx, button);
            }
            ElementState::Released => {
                ctx.input.mouse.on_button_released(button);
                game.on_mouse_button_release(ctx, button);
            }
        },
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
        _ => (),
    }
}

fn on_update(ctx: &mut Context, game: &mut impl Game, control_flow: &mut ControlFlow) {
    ctx.game_phase = GamePhase::Update;
    if let Err(error) = game.update(ctx) {
        if handle_error(game, ctx, error, control_flow) {
            return;
        }
    }

    ctx.game_phase = GamePhase::FixedUpdate;
    while ctx.time.fixed_update() {
        if let Err(error) = game.fixed_update(ctx) {
            if handle_error(game, ctx, error, control_flow) {
                return;
            }
        }
    }

    ctx.game_phase = GamePhase::LateUpdate;
    if let Err(error) = game.late_update(ctx) {
        if handle_error(game, ctx, error, control_flow) {
            return;
        }
    }

    ctx.window.request_redraw();
}

fn on_draw(ctx: &mut Context, game: &mut impl Game, control_flow: &mut ControlFlow) {
    ctx.game_phase = GamePhase::Draw;
    ctx.graphics.prepare();

    if let Err(error) = game.draw(ctx) {
        if handle_error(game, ctx, error, control_flow) {
            return;
        }
    }

    ctx.graphics.present();
}

fn on_frame_end(ctx: &mut Context) {
    ctx.input.on_frame_end();

    if !ctx.graphics.vsync {
        while !ctx.time.end_frame() {
            std::thread::yield_now();
        }
    }
}

fn handle_error(
    game: &mut impl Game,
    ctx: &mut Context,
    error: GameError,
    control_flow: &mut ControlFlow,
) -> bool {
    if game.handle_error(ctx, error) {
        control_flow.set_exit_with_code(ctx.game_phase.error_exit_code());
        true
    } else {
        false
    }
}
