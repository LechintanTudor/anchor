use crate::input::{Cursor, Keyboard, Mouse};

#[derive(Default, Debug)]
pub(crate) struct InputContext {
    pub(crate) keyboard: Keyboard,
    pub(crate) mouse: Mouse,
    pub(crate) cursor: Cursor,
}
