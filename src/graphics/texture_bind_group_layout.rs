use std::ops::Deref;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct TextureBindGroupLayout(Arc<wgpu::BindGroupLayout>);

impl TextureBindGroupLayout {
    pub fn new(device: &wgpu::Device) -> Self {
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        Self(Arc::new(texture_bind_group_layout))
    }

    pub fn get(&self) -> &wgpu::BindGroupLayout {
        &self.0
    }
}

impl Deref for TextureBindGroupLayout {
    type Target = wgpu::BindGroupLayout;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
