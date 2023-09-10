use crate::game::{GameError, GameResult};
use image::error::ImageError;
use image::RgbaImage;
use std::path::Path;

/// RGBA image stored in main memory.
#[derive(Clone, Debug)]
pub struct Image(pub(crate) RgbaImage);

impl Image {
    /// Creates an image with the given dimensions and data. Panics if `width * height * 4 !=
    /// data.len()`.
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        assert!((width * height * 4) as usize == data.len());
        Image(RgbaImage::from_vec(width, height, data).unwrap())
    }

    /// Loads the image from the given `path`.
    pub fn load_from_file<P>(path: P) -> GameResult<Image>
    where
        P: AsRef<Path>,
    {
        fn inner(path: &Path) -> GameResult<Image> {
            match image::open(path) {
                Ok(image) => Ok(Image(image.into_rgba8())),
                Err(error) => {
                    let context = match error {
                        ImageError::IoError(_) => {
                            format!("Failed to read image file '{}'", path.display())
                        }
                        _ => format!("Failed to parse image file '{}'", path.display()),
                    };

                    Err(GameError::new(error).context(context))
                }
            }
        }

        inner(path.as_ref())
    }

    /// Returns the width of the image.
    #[inline]
    pub fn width(&self) -> u32 {
        self.0.width()
    }

    /// Returns the height of the image.
    #[inline]
    pub fn height(&self) -> u32 {
        self.0.height()
    }

    /// Returns the image data as a byte slice.
    #[inline]
    pub fn data(&self) -> &[u8] {
        self.0.as_raw()
    }

    /// Consumes the image and returns the underlying data.
    #[inline]
    pub fn into_data(self) -> Vec<u8> {
        self.0.into_raw()
    }
}
