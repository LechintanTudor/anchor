use crate::platform::{Config, Context, FramePhase, Game, GameBuilder, GameError, GameResult};
use glam::DVec2;
use log::info;
use winit::event::{DeviceEvent, ElementState, Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

pub(crate) fn run<G>(config: Config, game_builder: G) -> GameResult<()>
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
                info!("Starting Anchor...");
            }
            Event::NewEvents(_) => {
                ctx.timer.start_frame();
                ctx.frame_phase = FramePhase::Input;

                if ctx.take_should_exit() && game.on_exit_requested(ctx) {
                    control_flow.set_exit();
                    return;
                }
            }
            Event::DeviceEvent { event, .. } => match event {
                DeviceEvent::MouseMotion { delta, .. } => {
                    let delta = DVec2::new(delta.0, delta.1);
                    game.on_mouse_motion(ctx, delta);
                }
                _ => (),
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    if game.on_exit_requested(ctx) {
                        control_flow.set_exit();
                    }
                }
                WindowEvent::Resized(size) => {
                    let (width, height) = (size.width, size.height);
                    ctx.graphics.on_window_resized(width, height);
                    game.on_window_resized(ctx, width, height);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        match input.state {
                            ElementState::Pressed => {
                                ctx.input.keyboard.on_key_pressed(key);
                                game.on_key_pressed(ctx, key);
                            }
                            ElementState::Released => {
                                ctx.input.keyboard.on_key_released(key);
                                game.on_key_released(ctx, key);
                            }
                        }
                    }
                }
                WindowEvent::MouseInput { state, button, .. } => match state {
                    ElementState::Pressed => {
                        ctx.input.mouse.on_button_pressed(button);
                        game.on_mouse_button_pressed(ctx, button);
                    }
                    ElementState::Released => {
                        ctx.input.mouse.on_button_released(button);
                        game.on_mouse_button_released(ctx, button);
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
                    game.on_cursor_moved(ctx, position);
                }
                WindowEvent::Focused(false) => {
                    ctx.input.keyboard.on_focus_lost();
                    ctx.input.mouse.on_focus_lost();
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                ctx.frame_phase = FramePhase::Update;
                if let Err(error) = game.update(ctx) {
                    if handle_error(game, ctx, FramePhase::Update, error, control_flow) {
                        return;
                    }
                }

                ctx.frame_phase = FramePhase::FixedUpdate;
                while ctx.timer.fixed_update() {
                    if let Err(error) = game.fixed_update(ctx) {
                        if handle_error(game, ctx, FramePhase::FixedUpdate, error, control_flow) {
                            return;
                        }
                    }
                }

                ctx.frame_phase = FramePhase::LateUpdate;
                if let Err(error) = game.late_update(ctx) {
                    if handle_error(game, ctx, FramePhase::LateUpdate, error, control_flow) {
                        return;
                    }
                }

                ctx.frame_phase = FramePhase::Draw;
                ctx.graphics.update_surface_texture();

                if let Err(error) = game.draw(ctx) {
                    if handle_error(game, ctx, FramePhase::Draw, error, control_flow) {
                        return;
                    }
                }

                ctx.input.on_frame_end();

                if let Some(surface_texture) = ctx.graphics.surface_texture.take() {
                    surface_texture.texture.present();
                }

                if !ctx.graphics.vsync {
                    while !ctx.timer.end_frame() {
                        std::thread::yield_now();
                    }
                }
            }
            Event::LoopDestroyed => {
                info!("Shutting down Anchor...");
            }
            _ => (),
        }
    });
}

fn handle_error<G>(
    game: &mut G,
    ctx: &mut Context,
    phase: FramePhase,
    error: GameError,
    control_flow: &mut ControlFlow,
) -> bool
where
    G: Game,
{
    if game.on_error(ctx, phase, error) {
        control_flow.set_exit_with_code(phase.error_exit_code());
        true
    } else {
        false
    }
}
