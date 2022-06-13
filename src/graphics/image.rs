use image::RgbaImage;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Image(RgbaImage);

impl Image {
    pub fn load<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let image = image::open(path).unwrap().into_rgba8();
        Self(image)
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.0.width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.0.height()
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        self.0.as_raw()
    }
}
