use wgpu::util::DeviceExt;

pub(crate) struct TextImage {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl TextImage {
    #[inline]
    pub fn new(width: u32, height: u32) -> Self {
        Self { data: vec![0; width as usize * height as usize], width, height }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        let mut data = vec![0; width as usize * height as usize];
        let copy_width = self.width.min(width) as usize;
        let copy_height = self.height.min(height) as usize;

        for i in 0..copy_height {
            unsafe {
                let src = self.data.as_mut_ptr().add(i * self.width as usize);
                let dst = data.as_mut_ptr().add(i * width as usize);
                std::ptr::copy_nonoverlapping(src, dst, copy_width);
            }
        }

        self.data = data;
        self.width = width;
        self.height = height;
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }
}

#[derive(Debug)]
pub(crate) struct TextTexture {
    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,
}

impl TextTexture {
    pub fn new(image: &TextImage, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        assert!(image.width != 0 && image.height != 0);

        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("text_texture"),
                size: wgpu::Extent3d {
                    width: image.width,
                    height: image.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            },
            &image.data,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, texture_view }
    }
}
