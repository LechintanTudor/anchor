use crate::input::MouseButton;
use rustc_hash::FxHashSet;

#[derive(Default, Debug)]
pub(crate) struct Mouse {
    pressed_buttons: FxHashSet<MouseButton>,
    just_pressed_buttons: FxHashSet<MouseButton>,
    just_released_buttons: FxHashSet<MouseButton>,
}

impl Mouse {
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        self.pressed_buttons.contains(&button)
    }

    pub fn was_button_just_pressed(&self, button: MouseButton) -> bool {
        self.just_pressed_buttons.contains(&button)
    }

    pub fn was_button_just_released(&self, button: MouseButton) -> bool {
        self.just_released_buttons.contains(&button)
    }

    pub fn on_button_pressed(&mut self, button: MouseButton) {
        if self.pressed_buttons.insert(button) {
            self.just_pressed_buttons.insert(button);
        }
    }

    pub fn on_button_released(&mut self, button: MouseButton) {
        if self.pressed_buttons.remove(&button) {
            self.just_released_buttons.insert(button);
        }
    }

    pub fn on_focus_lost(&mut self) {
        for button in self.pressed_buttons.drain() {
            self.just_released_buttons.insert(button);
        }
    }
}
