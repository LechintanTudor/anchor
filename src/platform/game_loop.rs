use crate::platform::{
    Config, Context, FpsLimiter, Game, GameBuilder, GameError, GameErrorKind, GameErrorOrigin,
    GameResult, ShouldYield,
};
use glam::DVec2;
use log::info;
use winit::dpi::Size;
use winit::event::{DeviceEvent, ElementState, Event, StartCause, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub(crate) fn run<G>(config: Config, game_builder: G) -> GameResult<()>
where
    G: GameBuilder,
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.window_title)
        .with_inner_size(Size::Physical(config.window_size.into()))
        .build(&event_loop)
        .map_err(|error| GameError::new(GameErrorKind::OsError(error), None))?;

    window.set_cursor_visible(config.cursor_visible);

    let mut ctx = Context::new(window);

    let mut game = game_builder.build(&mut ctx)?;
    let mut fps_limiter = FpsLimiter::new(60, 3);

    event_loop.run(move |event, _event_loop, control_flow| {
        let ctx = &mut ctx;

        match event {
            Event::NewEvents(StartCause::Init) => {
                info!("Starting Anchor...");
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
                        return;
                    }
                }
                WindowEvent::Resized(size) => {
                    let (width, height) = (size.width, size.height);
                    ctx.graphics.reconfigure_surface(width, height);
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
                        game.on_mouse_button_pressed(ctx, button);
                    }
                    ElementState::Released => {
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
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                if ctx.take_should_exit() && game.on_exit_requested(ctx) {
                    control_flow.set_exit();
                    return;
                }

                if fps_limiter.begin() == ShouldYield::Yes {
                    std::thread::yield_now();
                }

                let mut updated = false;

                while fps_limiter.update() {
                    if let Err(error) = game.update(ctx) {
                        if game.on_error(ctx, GameErrorOrigin::Update, error) {
                            control_flow.set_exit_with_code(1);
                            return;
                        }
                    }

                    updated = true;
                }

                if updated {
                    ctx.graphics.update_surface_texture();

                    if let Err(error) = game.draw(ctx) {
                        if game.on_error(ctx, GameErrorOrigin::Draw, error) {
                            control_flow.set_exit_with_code(2);
                            return;
                        }
                    }

                    if let Some(surface_texture) = ctx.graphics.surface_texture.take() {
                        surface_texture.texture.present();
                    }

                    ctx.input.keyboard.on_frame_end();
                }
            }
            Event::LoopDestroyed => {
                info!("Shutting down Anchor...");
            }
            _ => (),
        }
    });
}
