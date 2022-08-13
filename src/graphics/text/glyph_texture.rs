use wgpu::util::DeviceExt;

pub(crate) type GlyphTextureBounds = glyph_brush::Rectangle<u32>;

pub(crate) struct GlyphTexture {
    pub texture: wgpu::Texture,
    pub texture_view: wgpu::TextureView,
}

impl GlyphTexture {
    pub fn new(width: u32, height: u32, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        assert!(width != 0 && height != 0);

        let data = vec![0_u8; (width * height) as usize];

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("text_batch_texture"),
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            },
            &data,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, texture_view }
    }

    pub fn write(&mut self, bounds: GlyphTextureBounds, data: &[u8], queue: &wgpu::Queue) {
        let (offset_x, offset_y, width, height) =
            (bounds.min[0], bounds.min[1], bounds.width(), bounds.height());

        assert!(data.len() == (width * height) as usize);

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: (offset_y * width + offset_x) as u64,
                bytes_per_row: std::num::NonZeroU32::new(width),
                rows_per_image: std::num::NonZeroU32::new(height),
            },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
    }
}
