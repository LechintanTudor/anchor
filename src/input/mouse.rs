use crate::input::MouseButton;
use crate::utils::VecSet;

#[derive(Default, Debug)]
pub(crate) struct Mouse {
    pressed_buttons: VecSet<MouseButton>,
    just_pressed_buttons: VecSet<MouseButton>,
    just_released_buttons: VecSet<MouseButton>,
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

    pub fn pressed_buttons(&self) -> &[MouseButton] {
        self.pressed_buttons.as_slice()
    }

    pub fn just_pressed_buttons(&self) -> &[MouseButton] {
        self.just_pressed_buttons.as_slice()
    }

    pub fn just_released_buttons(&self) -> &[MouseButton] {
        self.just_released_buttons.as_slice()
    }

    pub fn on_frame_end(&mut self) {
        self.just_pressed_buttons.clear();
        self.just_released_buttons.clear();
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
