use crate::graphics::{Framebuffer, GraphicsConfig, ShapePipeline, SpritePipeline, TextPipeline};
use winit::window::Window;

const SAMPLE_COUNT: u32 = 4;

pub(crate) struct SurfaceTexture {
    pub texture: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
}

pub(crate) struct GraphicsUpdate {
    pub surface_width: u32,
    pub surface_height: u32,
    pub vsync: bool,
    pub multisample: bool,
}

pub(crate) struct GraphicsContext {
    pub(crate) surface: wgpu::Surface,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
    pub(crate) surface_texture: Option<SurfaceTexture>,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) vsync: bool,
    pub(crate) multisample: bool,
    pub(crate) next_update: Option<GraphicsUpdate>,
    pub(crate) framebuffer: Option<Framebuffer>,
    pub(crate) shape_pipeline: ShapePipeline,
    pub(crate) sprite_pipeline: SpritePipeline,
    pub(crate) text_pipeline: TextPipeline,
}

impl GraphicsContext {
    pub fn new(window: &Window, config: GraphicsConfig) -> Self {
        pollster::block_on(Self::new_async(window, config))
    }

    async fn new_async(window: &Window, config: GraphicsConfig) -> Self {
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

        let present_mode = if config.vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        };

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_width,
            height: window_height,
            present_mode,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &surface_config);

        let (framebuffer, sample_count) = if config.multisample {
            (Some(Framebuffer::new(&device, &surface_config, SAMPLE_COUNT)), SAMPLE_COUNT)
        } else {
            (None, 1)
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
            vsync: config.vsync,
            multisample: config.multisample,
            next_update: None,
            framebuffer,
            shape_pipeline,
            sprite_pipeline,
            text_pipeline,
        }
    }

    pub fn prepare_next_update(&mut self) -> &mut GraphicsUpdate {
        self.next_update.get_or_insert_with(|| GraphicsUpdate {
            surface_width: self.surface_config.width,
            surface_height: self.surface_config.height,
            vsync: self.vsync,
            multisample: self.multisample,
        })
    }

    pub fn on_window_resize(&mut self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            let update = self.prepare_next_update();
            update.surface_width = width;
            update.surface_height = height;
        }
    }

    pub fn prepare(&mut self) {
        self.update_state();
        self.update_surface_texture();
    }

    pub fn present(&mut self) {
        if let Some(surface_texture) = self.surface_texture.take() {
            surface_texture.texture.present();
        }
    }

    fn update_state(&mut self) {
        if let Some(update) = self.next_update.take() {
            // Recreate surface
            self.surface_config.width = update.surface_width;
            self.surface_config.height = update.surface_height;
            self.surface_config.present_mode = vsync_to_present_mode(update.vsync);
            self.surface.configure(&self.device, &self.surface_config);
            self.vsync = update.vsync;

            // Recreate framebuffer
            self.framebuffer = if update.multisample {
                Some(Framebuffer::new(&self.device, &self.surface_config, SAMPLE_COUNT))
            } else {
                None
            };

            // Recreate pipelines
            if self.multisample != update.multisample {
                let sample_count = if update.multisample { SAMPLE_COUNT } else { 1 };

                self.shape_pipeline.recreate_pipeline(
                    &self.device,
                    self.surface_config.format,
                    sample_count,
                );
                self.sprite_pipeline.recreate_pipeline(
                    &self.device,
                    self.surface_config.format,
                    sample_count,
                );
                self.text_pipeline.recreate_pipeline(
                    &self.device,
                    self.surface_config.format,
                    sample_count,
                );

                self.multisample = update.multisample;
            }
        }
    }

    fn update_surface_texture(&mut self) {
        let texture = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(wgpu::SurfaceError::Lost) => {
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }
            Err(_) => return,
        };

        let view = texture.texture.create_view(&Default::default());
        self.surface_texture = Some(SurfaceTexture { texture, view });
    }
}

fn vsync_to_present_mode(vsync: bool) -> wgpu::PresentMode {
    if vsync {
        wgpu::PresentMode::AutoVsync
    } else {
        wgpu::PresentMode::AutoNoVsync
    }
}
