use crate::core::{Config, EventHandler, GameBuilder, ShouldRun};
use log::info;
use std::time::{Duration, Instant};
use winit::event::{ElementState, Event, StartCause, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const TARGET_TPS: u32 = 60;
const MAX_UPDATES_PER_FRAME: u32 = 3;

pub fn run<G>(config: Config, game_builder: G) -> anyhow::Result<()>
where
    G: GameBuilder,
{
    let event_loop = EventLoop::new();
    let main_window = WindowBuilder::new()
        .with_title(config.window_title())
        .with_inner_size(config.window_size())
        .build(&event_loop)?;

    let mut game = game_builder.build(&event_loop, &main_window)?;

    let target_frame_time = Duration::from_secs(1) / TARGET_TPS;
    let max_accumulator = MAX_UPDATES_PER_FRAME * target_frame_time;
    let mut old_time = Instant::now();
    let mut accumulator = Duration::ZERO;

    event_loop.run(move |event, _event_loop, control_flow| match event {
        Event::NewEvents(StartCause::Init) => {
            info!("Starting tankage client...");
        }
        Event::WindowEvent { window_id, event } if window_id == main_window.id() => match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
            }
            WindowEvent::Resized(new_window_size) => {
                game.window_resize_event(new_window_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                game.window_resize_event(*new_inner_size);
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(key_code) = input.virtual_keycode {
                    let should_run = match input.state {
                        ElementState::Pressed => game.key_press_event(key_code),
                        ElementState::Released => game.key_press_event(key_code),
                    };

                    if should_run == ShouldRun::No {
                        *control_flow = ControlFlow::Exit;
                    }
                }
            }
            _ => (),
        },
        Event::MainEventsCleared => {
            let current_time = Instant::now();
            let last_frame_time = current_time - old_time;

            old_time = current_time;
            accumulator += last_frame_time;

            if accumulator > max_accumulator {
                accumulator = max_accumulator;
            }

            while accumulator >= target_frame_time {
                accumulator -= target_frame_time;

                match game.update() {
                    ShouldRun::Yes => main_window.request_redraw(),
                    ShouldRun::No => *control_flow = ControlFlow::Exit,
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == main_window.id() => {
            game.draw();
        }
        Event::LoopDestroyed => {
            info!("Shutting down tankage client...");
        }
        _ => (),
    });
}
