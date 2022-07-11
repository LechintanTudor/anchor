use crate::input::{Cursor, Keyboard};

#[derive(Default, Debug)]
pub(crate) struct InputContext {
    pub(crate) keyboard: Keyboard,
    pub(crate) cursor: Cursor,
}
