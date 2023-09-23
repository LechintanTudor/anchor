use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Anchor {
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

impl From<Anchor> for Vec2 {
    fn from(anchor: Anchor) -> Vec2 {
        match anchor {
            Anchor::TopLeft => Vec2::new(0.0, 0.0),
            Anchor::TopCenter => Vec2::new(0.5, 0.0),
            Anchor::TopRight => Vec2::new(1.0, 0.0),
            Anchor::CenterLeft => Vec2::new(0.0, 0.5),
            Anchor::Center => Vec2::new(0.5, 0.5),
            Anchor::CenterRight => Vec2::new(1.0, 0.5),
            Anchor::BottomLeft => Vec2::new(0.0, 1.0),
            Anchor::BottomCenter => Vec2::new(0.5, 1.0),
            Anchor::BottomRight => Vec2::new(1.0, 1.0),
        }
    }
}
