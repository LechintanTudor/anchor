use glam::{Affine2, Mat4, Quat, Vec2, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub translation: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        translation: Vec2::ZERO,
        rotation: 0.0,
        scale: Vec2::ONE,
    };

    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn to_affine2(&self) -> Affine2 {
        Affine2::from_scale_angle_translation(self.scale, self.rotation, self.translation)
    }

    pub fn to_mat4(&self) -> Mat4 {
        let translation = Vec3::new(self.translation.x, self.translation.y, 0.0);
        let rotation = Quat::from_rotation_z(self.rotation);
        let scale = Vec3::new(self.scale.x, self.scale.y, 1.0);
        Mat4::from_scale_rotation_translation(scale, rotation, translation)
    }

    pub fn lerp(&self, other: &Transform, alpha: f32) -> Transform {
        Transform {
            translation: self.translation.lerp(other.translation, alpha),
            rotation: self.rotation + (other.rotation - self.rotation) * alpha,
            scale: self.scale.lerp(other.scale, alpha),
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}
