use crate::core::{EventHandler, ShouldRun};
use crate::graphics::{FlatPipeline, GraphicsContext};
use log::error;
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct Game {
    graphics: GraphicsContext,
    flat_pipeline: FlatPipeline,
}

impl Game {
    pub fn new(_event_loop: &EventLoop<()>, main_window: &Window) -> anyhow::Result<Self> {
        let graphics = GraphicsContext::new(main_window)?;
        let flat_pipeline = FlatPipeline::new(&graphics.device, graphics.surface_config.format)?;

        Ok(Game {
            graphics,
            flat_pipeline,
        })
    }
}

impl EventHandler for Game {
    fn window_resize_event(&mut self, new_window_size: PhysicalSize<u32>) {
        self.graphics.resize_surface(new_window_size);
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
        let output = match self.graphics.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(error) => {
                error!("Surface error: {:?}", error);
                return;
            }
        };
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
            render_pass.set_pipeline(&self.flat_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        let command_buffer = encoder.finish();
        self.graphics.queue.submit(Some(command_buffer));
        output.present();
    }
}
