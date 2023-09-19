use crate::graphics::WgpuContext;
use glam::UVec2;
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub struct GlyphTexture {
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
    size: UVec2,
}

impl GlyphTexture {
    pub fn new<S>(
        wgpu: &WgpuContext,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        size: S,
    ) -> Self
    where
        S: Into<UVec2>,
    {
        let size = size.into();
        assert_ne!(size.x, 0);
        assert_ne!(size.y, 0);

        let texture = wgpu.device().create_texture_with_data(
            wgpu.queue(),
            &wgpu::TextureDescriptor {
                label: Some("glyph_texture"),
                size: wgpu::Extent3d {
                    width: size.x,
                    height: size.y,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            &vec![0; (size.x * size.y) as _],
        );

        let view = texture.create_view(&Default::default());

        let bind_group = wgpu.device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glyph_texture_bind_group"),
            layout: texture_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
        });

        Self {
            texture,
            view,
            bind_group,
            size,
        }
    }

    pub fn copy(&self, queue: &wgpu::Queue, x: u32, y: u32, w: u32, h: u32, data: &[u8]) {
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            data,
            wgpu::ImageDataLayout {
                offset: (y * w + x) as _,
                bytes_per_row: Some(w),
                rows_per_image: Some(h),
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }
}
