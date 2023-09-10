use crate::input::{Cursor, Keyboard, Modifiers, Mouse};

#[derive(Default, Debug)]
pub(crate) struct InputContext {
    pub(crate) modifiers: Modifiers,
    pub(crate) keyboard: Keyboard,
    pub(crate) mouse: Mouse,
    pub(crate) cursor: Cursor,
}

impl InputContext {
    #[inline]
    pub fn on_focus_lost(&mut self) {
        self.keyboard.on_focus_lost();
        self.mouse.on_focus_lost();
    }
}
