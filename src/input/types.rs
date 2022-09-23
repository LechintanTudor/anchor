use glam::{DVec2, Vec2};
use winit::event::MouseScrollDelta;

pub type Key = winit::event::VirtualKeyCode;
pub type MouseButton = winit::event::MouseButton;
pub type ModifiersState = winit::event::ModifiersState;

#[derive(Clone, Copy, Debug)]
pub enum ScrollDelta {
    LineDelta(Vec2),
    PixelDelta(DVec2),
}

impl From<MouseScrollDelta> for ScrollDelta {
    fn from(delta: MouseScrollDelta) -> Self {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => Self::LineDelta(Vec2::new(x, y)),
            MouseScrollDelta::PixelDelta(position) => {
                Self::PixelDelta(DVec2::new(position.x, position.y))
            }
        }
    }
}
