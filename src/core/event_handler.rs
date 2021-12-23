use crate::core::ShouldRun;
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub trait GameBuilder {
    type Game: EventHandler;

    fn build(self, event_loop: &EventLoop<()>, main_window: &Window) -> anyhow::Result<Self::Game>;
}

impl<F, G> GameBuilder for F
where
    F: FnOnce(&EventLoop<()>, &Window) -> anyhow::Result<G>,
    G: EventHandler,
{
    type Game = G;

    fn build(self, event_loop: &EventLoop<()>, main_window: &Window) -> anyhow::Result<Self::Game> {
        self(event_loop, main_window)
    }
}

pub trait EventHandler
where
    Self: Sized + 'static,
{
    fn window_resize_event(&mut self, _new_window_size: PhysicalSize<u32>) {}

    fn key_press_event(&mut self, _key_code: VirtualKeyCode) -> ShouldRun {
        ShouldRun::Yes
    }

    fn key_release_event(&mut self, _key_code: VirtualKeyCode) -> ShouldRun {
        ShouldRun::Yes
    }

    fn update(&mut self) -> ShouldRun;

    fn draw(&mut self);
}
