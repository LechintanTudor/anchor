#[derive(Default, Debug)]
pub(crate) struct Cursor {
    pub(crate) last_position: (f64, f64),
    pub(crate) hovers_window: bool,
}

impl Cursor {
    pub fn position(&self) -> Option<(f64, f64)> {
        self.hovers_window.then_some(self.last_position)
    }
}
