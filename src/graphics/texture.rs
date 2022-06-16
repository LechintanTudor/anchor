use crate::graphics::Image;
use std::sync::Arc;

struct TextureData {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
}

#[derive(Clone)]
pub struct Texture {
    data: Arc<TextureData>,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(image: &Image, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let width = image.width();
        let height = image.height();

        let size = wgpu::Extent3d { width, height, depth_or_array_layers: 1 };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            image.bytes(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * width),
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { data: Arc::new(TextureData { texture, texture_view }), width, height }
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
        &self.data.texture_view
    }
}
