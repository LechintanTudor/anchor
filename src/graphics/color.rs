use bytemuck::{Pod, Zeroable};
use glam::Vec4;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    pub const AQUA: Self = Self::rgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::rgb(1.0, 0.0, 1.0);

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const fn luma(value: f32) -> Self {
        Self { r: value, g: value, b: value, a: 1.0 }
    }

    pub const fn lumaa(value: f32, a: f32) -> Self {
        Self { r: value, g: value, b: value, a }
    }

    pub const fn transparent(a: f32) -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a }
    }

    pub fn as_linear_vec4(&self) -> Vec4 {
        Vec4::new(srgb_to_linear(self.r), srgb_to_linear(self.g), srgb_to_linear(self.b), self.a)
    }
}

impl From<Color> for wgpu::Color {
    fn from(color: Color) -> wgpu::Color {
        wgpu::Color {
            r: srgb_to_linear(color.r) as f64,
            g: srgb_to_linear(color.g) as f64,
            b: srgb_to_linear(color.b) as f64,
            a: color.a as f64,
        }
    }
}

fn srgb_to_linear(value: f32) -> f32 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}
