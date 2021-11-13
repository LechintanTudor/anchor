use crate::core::{EventHandler, ShouldRun};
use log::error;
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::Window;

#[allow(dead_code)]
pub struct Game {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    surface: wgpu::Surface,
}

impl Game {
    pub fn new(event_loop: &EventLoop<()>, main_window: &Window) -> anyhow::Result<Self> {
        Ok(pollster::block_on(Self::new_async(event_loop, main_window)))
    }

    async fn new_async(_event_loop: &EventLoop<()>, main_window: &Window) -> Self {
        let window_size = main_window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(main_window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: window_size.width,
            height: window_size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        Self {
            instance,
            adapter,
            device,
            queue,
            surface_config,
            surface,
        }
    }
}

impl EventHandler for Game {
    fn window_resize_event(&mut self, new_window_size: PhysicalSize<u32>) {
        if new_window_size.width != 0 && new_window_size.height != 0 {
            self.surface_config.width = new_window_size.width;
            self.surface_config.height = new_window_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    fn key_press_event(&mut self, key_code: VirtualKeyCode) -> ShouldRun {
        if key_code == VirtualKeyCode::Escape {
            ShouldRun::No
        } else {
            ShouldRun::Yes
        }
    }

    fn update(&mut self) -> ShouldRun {
        ShouldRun::Yes
    }

    fn draw(&mut self) {
        let output = match self.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(error) => {
                error!("Surface error: {:?}", error);
                return;
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("render_encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
        }

        let command_buffer = encoder.finish();
        self.queue.submit(Some(command_buffer));
        output.present();
    }
}
