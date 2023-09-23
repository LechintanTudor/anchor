use std::sync::Arc;

#[derive(Debug)]
struct SharedBindGroupLayoutsData {
    projection: wgpu::BindGroupLayout,
    texture: wgpu::BindGroupLayout,
    sampler: wgpu::BindGroupLayout,
}

impl SharedBindGroupLayoutsData {
    fn new(device: &wgpu::Device) -> Self {
        let projection = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("projection_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let texture = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            }],
        });

        let sampler = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sampler_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            }],
        });

        Self {
            projection,
            texture,
            sampler,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SharedBindGroupLayouts(Arc<SharedBindGroupLayoutsData>);

impl SharedBindGroupLayouts {
    pub fn new(device: &wgpu::Device) -> Self {
        Self(Arc::new(SharedBindGroupLayoutsData::new(device)))
    }

    #[inline]
    pub fn projection(&self) -> &wgpu::BindGroupLayout {
        &self.0.projection
    }

    #[inline]
    pub fn texture(&self) -> &wgpu::BindGroupLayout {
        &self.0.texture
    }

    #[inline]
    pub fn sampler(&self) -> &wgpu::BindGroupLayout {
        &self.0.sampler
    }
}
