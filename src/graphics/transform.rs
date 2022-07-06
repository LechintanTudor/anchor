use glam::{Affine2, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Default for Transform {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Transform {
    pub const DEFAULT: Self = Self { translation: Vec2::ZERO, rotation: 0.0, scale: Vec2::ONE };

    #[inline]
    pub fn from_translation(translation: Vec2) -> Self {
        Self { translation, ..Self::DEFAULT }
    }

    #[inline]
    pub fn to_affine2(&self) -> Affine2 {
        Affine2::from_scale_angle_translation(self.scale, self.rotation, self.translation)
    }
}
