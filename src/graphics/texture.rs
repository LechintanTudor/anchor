use crate::core::{Context, GameResult};
use crate::graphics::Image;
use std::path::Path;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Clone, Debug)]
pub struct Texture {
    view: Arc<wgpu::TextureView>,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn from_image(ctx: &Context, image: &Image) -> Self {
        let width = image.width();
        let height = image.height();

        let texture_descriptor = wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        };

        let view = ctx
            .graphics
            .device
            .create_texture_with_data(&ctx.graphics.queue, &texture_descriptor, image.data())
            .create_view(&Default::default());

        Self { view: Arc::new(view), width, height }
    }

    pub fn load_from_file<P>(ctx: &Context, path: P) -> GameResult<Self>
    where
        P: AsRef<Path>,
    {
        let image = Image::load_from_file(path)?;
        Ok(Self::from_image(ctx, &image))
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }
}
