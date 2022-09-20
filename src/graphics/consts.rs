pub mod anchors {
    use glam::Vec2;

    pub const ANCHOR_TOP_LEFT: Vec2 = Vec2::new(-0.5, -0.5);
    pub const ANCHOR_TOP_CENTER: Vec2 = Vec2::new(0.0, -0.5);
    pub const ANCHOR_TOP_RIGHT: Vec2 = Vec2::new(0.5, -0.5);
    pub const ANCHOR_CENTER_LEFT: Vec2 = Vec2::new(-0.5, 0.0);
    pub const ANCHOR_CENTER: Vec2 = Vec2::new(0.0, 0.0);
    pub const ANCHOR_CENTER_RIGHT: Vec2 = Vec2::new(0.5, 0.0);
    pub const ANCHOR_BOTTOM_LEFT: Vec2 = Vec2::new(-0.5, 0.5);
    pub const ANCHOR_BOTTOM_CENTER: Vec2 = Vec2::new(0.0, 0.5);
    pub const ANCHOR_BOTTOM_RIGHT: Vec2 = Vec2::new(0.5, 0.5);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FilterMode {
    Nearest = 0,
    Linear = 1,
}

impl From<FilterMode> for wgpu::FilterMode {
    fn from(filter_mode: FilterMode) -> Self {
        match filter_mode {
            FilterMode::Nearest => Self::Nearest,
            FilterMode::Linear => Self::Linear,
        }
    }
}
