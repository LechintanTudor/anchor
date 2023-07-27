use crate::input::KeyCode;
use rustc_hash::FxHashSet;

#[derive(Debug, Default)]
pub(crate) struct Keyboard {
    pressed_keys: FxHashSet<KeyCode>,
    just_pressed_keys: FxHashSet<KeyCode>,
    just_released_keys: FxHashSet<KeyCode>,
}

impl Keyboard {
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn was_key_just_pressed(&self, key: KeyCode) -> bool {
        self.just_pressed_keys.contains(&key)
    }

    pub fn was_key_just_released(&self, key: KeyCode) -> bool {
        self.just_released_keys.contains(&key)
    }

    pub fn on_frame_end(&mut self) {
        self.just_pressed_keys.clear();
        self.just_released_keys.clear();
    }

    pub fn on_key_pressed(&mut self, key: KeyCode) {
        if self.pressed_keys.insert(key) {
            self.just_pressed_keys.insert(key);
        }
    }

    pub fn on_key_released(&mut self, key: KeyCode) {
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
