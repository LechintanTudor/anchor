use crate::core::{GameErrorKind, GameResult};
use image::error::ImageError;
use image::RgbaImage;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Image(RgbaImage);

impl Image {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        assert!((width * height) as usize == data.len());
        Image(RgbaImage::from_vec(width, height, data).unwrap())
    }

    pub fn load_from_file<P>(path: P) -> GameResult<Image>
    where
        P: AsRef<Path>,
    {
        fn inner(path: &Path) -> GameResult<Image> {
            match image::open(path) {
                Ok(image) => Ok(Image(image.into_rgba8())),
                Err(error) => match error {
                    ImageError::IoError(error) => {
                        Err(GameErrorKind::IoError(error).into_error_with_path(path))
                    }
                    error => Err(GameErrorKind::ImageError(error).into_error_with_path(path)),
                },
            }
        }

        inner(path.as_ref())
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
