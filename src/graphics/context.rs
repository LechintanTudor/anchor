use crate::graphics::{Framebuffer, ShapePipeline, SpritePipeline, TextPipeline};
use winit::window::Window;

pub(crate) struct SurfaceTexture {
    pub texture: wgpu::SurfaceTexture,
    pub texture_view: wgpu::TextureView,
}

pub(crate) struct MultisampleData {
    pub sample_count: u32,
    pub framebuffer: Framebuffer,
}

pub(crate) struct GraphicsContext {
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) surface_texture: Option<SurfaceTexture>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) vsync: bool,
    pub(crate) multisample_data: Option<MultisampleData>,
    pub(crate) shape_pipeline: ShapePipeline,
    pub(crate) sprite_pipeline: SpritePipeline,
    pub(crate) text_pipeline: TextPipeline,
}

impl GraphicsContext {
    pub(crate) fn new(window: &Window, vsync: bool, sample_count: u32) -> Self {
        pollster::block_on(Self::new_async(window, vsync, sample_count))
    }

    async fn new_async(window: &Window, vsync: bool, sample_count: u32) -> Self {
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
                    label: Some("graphics_context_device"),
                    features: wgpu::Features::default(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("No suitable graphics device found");

        let surface_format = surface
            .get_supported_formats(&adapter)
            .first()
            .copied()
            .expect("No suitable surface format found");

        let present_mode =
            if vsync { wgpu::PresentMode::AutoVsync } else { wgpu::PresentMode::AutoNoVsync };

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_width,
            height: window_height,
            present_mode,
        };

        surface.configure(&device, &surface_config);

        let multisample_data = if sample_count > 1 {
            Some(MultisampleData {
                sample_count,
                framebuffer: Framebuffer::new(&device, &surface_config, sample_count),
            })
        } else {
            None
        };

        let shape_pipeline = ShapePipeline::new(&device, surface_format, sample_count);
        let sprite_pipeline = SpritePipeline::new(&device, surface_format, sample_count);
        let text_pipeline = TextPipeline::new(&device, surface_format, sample_count);

        Self {
            surface,
            surface_config,
            surface_texture: None,
            device,
            queue,
            vsync,
            multisample_data,
            shape_pipeline,
            sprite_pipeline,
            text_pipeline,
        }
    }

    pub(crate) fn on_window_resized(&mut self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);

            if let Some(multisample_data) = self.multisample_data.as_mut() {
                multisample_data.framebuffer = Framebuffer::new(
                    &self.device,
                    &self.surface_config,
                    multisample_data.sample_count,
                );
            }
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
