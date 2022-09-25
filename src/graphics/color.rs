use bytemuck::{Pod, Zeroable};
use glam::Vec4;

/// Color with alpha channel in the sRGB color space.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Color {
    /// Value of the red channel.
    pub r: f32,
    /// Value of the green channel.
    pub g: f32,
    /// Value of the blue channel.
    pub b: f32,
    /// Value of the alpha channel
    pub a: f32,
}

#[allow(missing_docs)]
impl Color {
    pub const WHITE: Self = Self::rgb(1.0, 1.0, 1.0);
    pub const BLACK: Self = Self::rgb(0.0, 0.0, 0.0);
    pub const RED: Self = Self::rgb(1.0, 0.0, 0.0);
    pub const GREEN: Self = Self::rgb(0.0, 1.0, 0.0);
    pub const BLUE: Self = Self::rgb(0.0, 0.0, 1.0);
    pub const YELLOW: Self = Self::rgb(1.0, 1.0, 0.0);
    pub const AQUA: Self = Self::rgb(0.0, 1.0, 1.0);
    pub const MAGENTA: Self = Self::rgb(1.0, 0.0, 1.0);

    /// Creates an opaque color from the given values.
    #[inline]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    /// Creates a color from the given values.
    #[inline]
    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates an opaque grayscale color with the given value.
    #[inline]
    pub const fn luma(value: f32) -> Self {
        Self { r: value, g: value, b: value, a: 1.0 }
    }

    /// Creates a grayscale color with the given value and opacity.
    #[inline]
    pub const fn lumaa(value: f32, a: f32) -> Self {
        Self { r: value, g: value, b: value, a }
    }

    /// Creates a transparent white with the given opacity.
    #[inline]
    pub const fn transparent(a: f32) -> Self {
        Self { r: 1.0, g: 1.0, b: 1.0, a }
    }

    /// Returns the color as a tuple with values stored in the linear RGB color space.
    #[inline]
    pub fn to_linear_tuple(&self) -> (f32, f32, f32, f32) {
        (srgb_to_linear(self.r), srgb_to_linear(self.g), srgb_to_linear(self.b), self.a)
    }

    /// Returns the color as an array with values stored in the linear RGB color space.
    #[inline]
    pub fn to_linear_array(&self) -> [f32; 4] {
        [srgb_to_linear(self.r), srgb_to_linear(self.g), srgb_to_linear(self.b), self.a]
    }

    /// Returns the color as a [Vec4] with values stored in the linear RGB color space.
    #[inline]
    pub fn to_linear_vec4(&self) -> Vec4 {
        Vec4::new(srgb_to_linear(self.r), srgb_to_linear(self.g), srgb_to_linear(self.b), self.a)
    }
}

impl From<(f32, f32, f32)> for Color {
    #[inline]
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    #[inline]
    fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
        Self::rgba(r, g, b, a)
    }
}

impl From<[f32; 3]> for Color {
    #[inline]
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self::rgb(r, g, b)
    }
}

impl From<[f32; 4]> for Color {
    #[inline]
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Self::rgba(r, g, b, a)
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
