use crate::input::MouseButton;
use crate::utils::SmallVecSet;

pub struct Mouse {
    pressed_buttons: SmallVecSet<MouseButton, 6>,
    just_pressed_buttons: SmallVecSet<MouseButton, 6>,
    just_released_buttons: SmallVecSet<MouseButton, 6>,
}
