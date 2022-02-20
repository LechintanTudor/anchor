use crate::core::{
    Config, Context, FpsLimiter, Game, GameBuilder, GameError, GameResult, ShouldRun, ShouldYield,
};
use log::{error, info};
use std::thread;
use winit::event::{ElementState, Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub fn run<G>(config: Config, game_builder: G) -> GameResult<()>
where
    G: GameBuilder,
{
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(config.window_title)
        .with_inner_size(config.window_size)
        .build(&event_loop)
        .map_err(GameError::CannotCreateWindow)?;

    let mut ctx = Context { window, should_exit: false };
    let mut game = game_builder.build(&mut ctx)?;
    let mut fps_limiter = FpsLimiter::new(60, 3);

    event_loop.run(move |event, _event_loop, control_flow| {
        let ctx = &mut ctx;

        match event {
            Event::NewEvents(StartCause::Init) => {
                info!("Starting Anchor...");
            }
            Event::WindowEvent { window_id, event } if window_id == ctx.window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(new_inner_size) => {
                        game.on_window_resized(ctx, new_inner_size.width, new_inner_size.height);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        let width = new_inner_size.width;
                        let height = new_inner_size.height;
                        game.on_window_resized(ctx, width, height);
                    }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(key_code) = input.virtual_keycode {
                            match input.state {
                                ElementState::Pressed => game.on_key_pressed(ctx, key_code),
                                ElementState::Released => game.on_key_released(ctx, key_code),
                            }
                        }
                    }
                    _ => (),
                }
            }
            Event::MainEventsCleared => {
                if fps_limiter.begin() == ShouldYield::Yes {
                    thread::yield_now();
                }

                while fps_limiter.update() == ShouldRun::Yes {
                    if ctx.should_exit {
                        *control_flow = ControlFlow::Exit;
                        break;
                    }

                    if let Err(error) = game.update(ctx) {
                        handle_error(error, control_flow);
                        return;
                    }
                }

                if let Err(error) = game.draw(ctx) {
                    handle_error(error, control_flow);
                }
            }
            Event::LoopDestroyed => {
                info!("Shutting down Anchor...");
            }
            _ => (),
        }
    });
}

fn handle_error(error: GameError, control_flow: &mut ControlFlow) {
    error!("{}", error);
    *control_flow = ControlFlow::Exit;
}
