use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum RelativeAnchor {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl From<RelativeAnchor> for Vec2 {
    fn from(anchor: RelativeAnchor) -> Vec2 {
        match anchor {
            RelativeAnchor::TopLeft => Vec2::new(0.0, 0.0),
            RelativeAnchor::TopCenter => Vec2::new(0.5, 0.0),
            RelativeAnchor::TopRight => Vec2::new(1.0, 0.0),
            RelativeAnchor::CenterLeft => Vec2::new(0.0, 0.5),
            RelativeAnchor::Center => Vec2::new(0.5, 0.5),
            RelativeAnchor::CenterRight => Vec2::new(1.0, 0.5),
            RelativeAnchor::BottomLeft => Vec2::new(0.0, 1.0),
            RelativeAnchor::BottomCenter => Vec2::new(0.5, 1.0),
            RelativeAnchor::BottomRight => Vec2::new(1.0, 1.0),
        }
    }
}
