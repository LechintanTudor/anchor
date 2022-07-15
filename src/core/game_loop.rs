use crate::core::{
    Config, Context, FpsLimiter, Game, GameBuilder, GameError, GameErrorKind, GameResult,
    ShouldYield,
};
use crate::graphics;
use log::info;
use winit::dpi::Size;
use winit::event::{ElementState, Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
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
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
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
                WindowEvent::CursorEntered { .. } => {
                    ctx.input.cursor.hovers_window = true;
                }
                WindowEvent::CursorLeft { .. } => {
                    ctx.input.cursor.hovers_window = false;
                }
                WindowEvent::CursorMoved { position, .. } => {
                    ctx.input.cursor.last_position = position.into();
                }
                WindowEvent::Focused(false) => {
                    ctx.input.keyboard.on_focus_lost();
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                if ctx.should_exit {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                if fps_limiter.begin() == ShouldYield::Yes {
                    std::thread::yield_now();
                }

                let mut updated = false;

                while fps_limiter.update() {
                    if let Err(error) = game.update(ctx) {
                        handle_error(&error, control_flow);
                        return;
                    }

                    updated = true;
                }

                if updated {
                    let frame = match game.draw(ctx) {
                        Ok(frame) => frame,
                        Err(error) => {
                            handle_error(&error, control_flow);
                            return;
                        }
                    };

                    graphics::display(ctx, frame);
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

fn handle_error(error: &GameError, control_flow: &mut ControlFlow) {
    println!("{}", error);
    *control_flow = ControlFlow::Exit;
}
