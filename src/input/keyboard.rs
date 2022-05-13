pub use winit::event::VirtualKeyCode as Key;

use crate::utils::SmallVecSet;

#[derive(Debug, Default)]
pub(crate) struct Keyboard {
    pressed_keys: SmallVecSet<Key, 12>,
    just_pressed_keys: SmallVecSet<Key, 12>,
    just_released_keys: SmallVecSet<Key, 12>,
}

impl Keyboard {
    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn was_key_just_pressed(&self, key: Key) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    pub fn was_key_just_released(&self, key: Key) -> bool {
        self.just_released_keys.contains(&key)
    }

    pub fn pressed_keys(&self) -> &[Key] {
        self.pressed_keys.as_slice()
    }

    pub fn just_pressed_keys(&self) -> &[Key] {
        self.just_pressed_keys.as_slice()
    }

    pub fn just_released_keys(&self) -> &[Key] {
        self.just_released_keys.as_slice()
    }

    pub fn on_frame_end(&mut self) {
        self.just_pressed_keys.clear();
        self.just_released_keys.clear();
    }

    pub fn on_key_pressed(&mut self, key: Key) {
        if self.pressed_keys.insert(key) {
            self.just_pressed_keys.insert(key);
        }
    }

    pub fn on_key_released(&mut self, key: Key) {
        if self.pressed_keys.remove(&key) {
            self.just_released_keys.insert(key);
        }
    }

    pub fn on_focus_lost(&mut self) {
        for key in self.pressed_keys.drain() {
            self.just_released_keys.insert(key);
        }
    }
}
