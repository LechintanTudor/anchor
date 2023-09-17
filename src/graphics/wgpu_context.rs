use std::sync::Arc;

#[derive(Debug)]
struct WgpuContextData {
    device: wgpu::Device,
    queue: wgpu::Queue,
}

#[derive(Clone, Debug)]
pub struct WgpuContext(Arc<WgpuContextData>);

impl WgpuContext {
    pub fn new(device: wgpu::Device, queue: wgpu::Queue) -> Self {
        Self(Arc::new(WgpuContextData { device, queue }))
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.0.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.0.queue
    }
}

impl AsRef<WgpuContext> for WgpuContext {
    fn as_ref(&self) -> &WgpuContext {
        self
    }
}
