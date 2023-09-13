pub mod shape;

mod camera;
mod camera_manager;
mod canvas;
mod color;
mod drawable;
mod utils;
mod wgpu_context;

pub use self::camera::*;
pub use self::camera_manager::*;
pub use self::canvas::*;
pub use self::color::*;
pub use self::drawable::*;
pub use self::wgpu_context::*;

pub(crate) use self::utils::*;

use crate::game::{Config, GameResult};
use crate::graphics::shape::ShapeRenderer;
use anyhow::anyhow;
use glam::UVec2;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window, WindowBuilder};

#[derive(Debug)]
pub struct GraphicsContext {
    pub wgpu: WgpuContext,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    window: Window,
    pub(crate) camera_manager: CameraManager,
    pub(crate) shape_renderer: ShapeRenderer,
}

impl GraphicsContext {
    pub fn new(event_loop: &EventLoopWindowTarget<()>, config: &Config) -> GameResult<Self> {
        pollster::block_on(Self::new_async(event_loop, config))
    }

    async fn new_async(
        event_loop: &EventLoopWindowTarget<()>,
        config: &Config,
    ) -> GameResult<Self> {
        let window = WindowBuilder::new()
            .with_title(&config.window_title)
            .with_inner_size(PhysicalSize::<u32>::from(config.window_size))
            .build(event_loop)?;

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window)? };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or_else(|| anyhow!("No suitable graphics adapter found"))?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("graphics_context_device"),
                    ..Default::default()
                },
                None,
            )
            .await?;

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities
            .formats
            .iter()
            .find(|format| format.is_srgb())
            .or_else(|| surface_capabilities.formats.first())
            .copied()
            .ok_or_else(|| anyhow!("No suitable surface format found"))?;

        let present_mode = if config.vsync {
            wgpu::PresentMode::AutoVsync
        } else {
            wgpu::PresentMode::AutoNoVsync
        };

        let window_size = window.inner_size();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            view_formats: vec![],
            width: window_size.width,
            height: window_size.height,
            present_mode,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &surface_config);

        let wgpu = WgpuContext::new(device, queue);
        let camera_manager = CameraManager::new(wgpu.clone());
        let shape_renderer =
            ShapeRenderer::new(wgpu.clone(), &camera_manager, surface_config.format, 1);

        Ok(Self { wgpu, surface, surface_config, window, camera_manager, shape_renderer })
    }

    pub fn device(&self) -> &wgpu::Device {
        self.wgpu.device()
    }

    pub fn queue(&self) -> &wgpu::Queue {
        self.wgpu.queue()
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn surface_size(&self) -> UVec2 {
        UVec2::new(self.surface_config.width, self.surface_config.height)
    }

    pub fn get_surface_texture(&self) -> Option<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            Ok(texture) => Some(texture),
            Err(wgpu::SurfaceError::Lost) => {
                self.configure_surface();
                None
            }
            _ => None,
        }
    }

    pub fn resize_surface(&mut self, surface_size: UVec2) {
        if surface_size.x == 0 || surface_size.y == 0 {
            return;
        }

        self.surface_config.width = surface_size.x;
        self.surface_config.height = surface_size.y;
        self.configure_surface();
    }

    pub fn configure_surface(&self) {
        self.surface.configure(self.device(), &self.surface_config);
    }
}
