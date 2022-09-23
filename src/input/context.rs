use crate::input::{Cursor, Keyboard, ModifiersState, Mouse};

#[derive(Default, Debug)]
pub(crate) struct InputContext {
    pub(crate) modifiers: ModifiersState,
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

    #[inline]
    pub fn on_frame_end(&mut self) {
        self.keyboard.on_frame_end();
        self.mouse.on_frame_end();
    }
}
