use glam::DVec2;

#[derive(Default, Debug)]
pub(crate) struct Cursor {
    pub(crate) last_position: DVec2,
    pub(crate) hovers_window: bool,
}

impl Cursor {
    pub fn position(&self) -> Option<DVec2> {
        self.hovers_window.then_some(self.last_position)
    }
}
