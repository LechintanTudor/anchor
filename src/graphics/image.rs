use image::RgbaImage;

#[derive(Clone, Debug)]
pub struct Image(RgbaImage);

impl Image {
    pub(crate) fn new(image: RgbaImage) -> Self {
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
    pub fn data(&self) -> &[u8] {
        self.0.as_raw()
    }

    #[inline]
    pub fn into_data(self) -> Vec<u8> {
        self.0.into_raw()
    }
}
