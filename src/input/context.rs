use crate::input::{Cursor, Keyboard, Mouse};

#[derive(Default, Debug)]
pub(crate) struct InputContext {
    pub(crate) keyboard: Keyboard,
    pub(crate) mouse: Mouse,
    pub(crate) cursor: Cursor,
}

impl InputContext {
    #[inline]
    pub fn on_frame_end(&mut self) {
        self.keyboard.on_frame_end();
        self.mouse.on_frame_end();
    }
}
