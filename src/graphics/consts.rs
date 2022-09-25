use glam::Vec2;

/// Anchors the entity to the top left of its bounding box.
pub const ANCHOR_TOP_LEFT: Vec2 = Vec2::new(-0.5, -0.5);

/// Anchors the entity to the top center of its bounding box.
pub const ANCHOR_TOP_CENTER: Vec2 = Vec2::new(0.0, -0.5);

/// Anchors the entity to the top right of its bounding box.
pub const ANCHOR_TOP_RIGHT: Vec2 = Vec2::new(0.5, -0.5);

/// Anchors the entity to the center left of its bounding box.
pub const ANCHOR_CENTER_LEFT: Vec2 = Vec2::new(-0.5, 0.0);

/// Anchors the entity to the center of its bounding box.
pub const ANCHOR_CENTER: Vec2 = Vec2::new(0.0, 0.0);

/// Anchors the entity to the center right of its bounding box.
pub const ANCHOR_CENTER_RIGHT: Vec2 = Vec2::new(0.5, 0.0);

/// Anchors the entity to the bottom left of its bounding box.
pub const ANCHOR_BOTTOM_LEFT: Vec2 = Vec2::new(-0.5, 0.5);

/// Anchors the entity to the bottom center its bounding box.
pub const ANCHOR_BOTTOM_CENTER: Vec2 = Vec2::new(0.0, 0.5);

/// Anchors the entity to the bottom right its bounding box.
pub const ANCHOR_BOTTOM_RIGHT: Vec2 = Vec2::new(0.5, 0.5);

/// Filters to apply when sampling textures.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FilterMode {
    /// Textures are pixelated when magnified.
    Nearest = 0,
    /// Textures are smooth, but blurry when magnified.
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

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub(crate) enum BatchStatus {
    #[default]
    Empty,
    NonEmpty,
    Ready,
}
