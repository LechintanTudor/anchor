use crate::graphics::Image;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Clone, Debug)]
pub struct Texture {
    view: Arc<wgpu::TextureView>,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(image: &Image, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let width = image.width();
        let height = image.height();

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: None,
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            },
            image.data(),
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { view: Arc::new(view), width, height }
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
