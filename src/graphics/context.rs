use crate::graphics::{ShapePipeline, SpritePipeline, SpritePipeline2};
use glam::Mat4;
use winit::window::Window;

pub(crate) struct GraphicsContext {
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_format: wgpu::TextureFormat,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) shape_pipeline: ShapePipeline,
    pub(crate) sprite_pipeline: SpritePipeline,
    pub(crate) sprite_pipeline2: SpritePipeline2,
}

impl GraphicsContext {
    pub(crate) fn new(window: &Window) -> Self {
        pollster::block_on(Self::new_async(window))
    }

    async fn new_async(window: &Window) -> Self {
        let (window_width, window_height) = window.inner_size().into();

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("No suitable graphics adapter found");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("No suitable graphics device found");

        let surface_format = surface.get_supported_formats(&adapter)[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_width,
            height: window_height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &surface_config);

        let shape_pipeline = ShapePipeline::new(&device, surface_format);
        let sprite_pipeline = SpritePipeline::new(&device, surface_format);
        let sprite_pipeline2 = SpritePipeline2::new(&device, surface_format);

        Self {
            surface,
            surface_format,
            surface_config,
            device,
            queue,
            shape_pipeline,
            sprite_pipeline,
            sprite_pipeline2,
        }
    }

    pub(crate) fn reconfigure_surface(&mut self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub(crate) fn window_ortho_matrix(&self) -> Mat4 {
        let width = self.surface_config.width as f32;
        let height = self.surface_config.height as f32;
        Mat4::orthographic_rh(0.0, width, 0.0, height, 0.0, 1.0)
    }
}
