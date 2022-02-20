use crate::core::{GameError, GameResult};
use crate::graphics::FlatPipeline;
use log::error;
use wgpu::*;
use winit::window::Window;

pub struct GraphicsContext {
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) surface: Surface,
    pub(crate) surface_config: SurfaceConfiguration,
    pub(crate) flat_pipeline: FlatPipeline,
}

impl GraphicsContext {
    pub fn new(window: &Window) -> GameResult<Self> {
        pollster::block_on(Self::new_async(window))
    }

    async fn new_async(window: &Window) -> GameResult<Self> {
        let window_size = window.inner_size();

        let instance = Instance::new(Backends::VULKAN);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or(GameError::NoGraphicsAdaptersFound)?;

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .map_err(GameError::CannotConnectToGraphicsDevice)?;

        let surface_config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        let flat_pipeline = FlatPipeline::new(&device, surface_config.format);

        Ok(Self { device, queue, surface, surface_config, flat_pipeline })
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        if width != 0 && height != 0 {
            self.surface_config.width = width;
            self.surface_config.height = height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn draw(&mut self) {
        let output = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(error) => {
                error!("Surface error: {:?}", error);
                return;
            }
        };
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.flat_pipeline.pipeline);
            render_pass.set_vertex_buffer(0, self.flat_pipeline.vertex_buffer.slice(..));
            render_pass.draw(0..3, 0..1);
        }

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
        output.present();
    }
}

pub(crate) mod api {}
