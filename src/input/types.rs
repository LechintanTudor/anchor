use glam::{DVec2, Vec2};
use winit::event::MouseScrollDelta;

/// Keyboard key code.
pub type KeyCode = winit::keyboard::KeyCode;

/// Mouse button.
pub type MouseButton = winit::event::MouseButton;

/// State of the keyboard modifiers.
pub type Modifiers = winit::event::Modifiers;

/// Scroll distance in lines or pixels.
#[derive(Clone, Copy, Debug)]
pub enum ScrollDelta {
    /// Scroll distance in lines.
    LineDelta(Vec2),
    /// Scroll distance in pixels.
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
