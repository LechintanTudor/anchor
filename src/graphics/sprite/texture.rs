use crate::game::GameResult;
use crate::graphics::GraphicsContext;
use glam::UVec2;
use std::path::Path;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[derive(Debug)]
struct TextureData {
    view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

#[derive(Clone, Debug)]
pub struct Texture {
    data: Arc<TextureData>,
    size: UVec2,
}

impl Texture {
    pub fn from_file<G, P>(graphics: &G, path: P) -> GameResult<Self>
    where
        G: AsRef<GraphicsContext>,
        P: AsRef<Path>,
    {
        let graphics = graphics.as_ref();
        let image = image::open(path)?.to_rgba8();

        let view = graphics
            .device()
            .create_texture_with_data(
                graphics.queue(),
                &wgpu::TextureDescriptor {
                    label: Some("rgba_texture"),
                    size: wgpu::Extent3d {
                        width: image.width(),
                        height: image.height(),
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
                &image,
            )
            .create_view(&Default::default());

        let bind_group = graphics.device().create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("rgba_texture_bind_group"),
            layout: &graphics.texture_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            }],
        });

        Ok(Self {
            data: Arc::new(TextureData { view, bind_group }),
            size: UVec2::new(image.width(), image.height()),
        })
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.data.view
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.data.bind_group
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }
}

impl Eq for Texture {
    // Empty
}
