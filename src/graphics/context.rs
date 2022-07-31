use crate::graphics::{OldRawWindowHandleWrapper, ShapePipeline, SpritePipeline, TextPipeline};
use winit::window::Window;

pub(crate) struct SurfaceTexture {
    pub texture: wgpu::SurfaceTexture,
    pub texture_view: wgpu::TextureView,
}

pub(crate) struct GraphicsContext {
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_format: wgpu::TextureFormat,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) surface_texture: Option<SurfaceTexture>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) shape_pipeline: ShapePipeline,
    pub(crate) sprite_pipeline: SpritePipeline,
    pub(crate) text_pipeline: TextPipeline,
}

impl GraphicsContext {
    pub(crate) fn new(window: &Window) -> Self {
        pollster::block_on(Self::new_async(window))
    }

    async fn new_async(window: &Window) -> Self {
        let (window_width, window_height) = window.inner_size().into();

        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let surface = unsafe { instance.create_surface(&OldRawWindowHandleWrapper(window)) };

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
        let text_pipeline = TextPipeline::new(&device, surface_format);

        Self {
            surface,
            surface_format,
            surface_config,
            device,
            queue,
            shape_pipeline,
            sprite_pipeline,
            text_pipeline,
            surface_texture: Default::default(),
        }
    }

    pub(crate) fn reconfigure_surface(&mut self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub(crate) fn update_surface_texture(&mut self) {
        let texture = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost) => {
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
            Err(_) => return,
        };
        let texture_view = texture.texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.surface_texture = Some(SurfaceTexture { texture, texture_view });
    }
}
