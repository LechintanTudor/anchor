use crate::core::{EventHandler, ShouldRun};
use crate::graphics::Graphics;
use log::error;
use winit::dpi::PhysicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct Game {
    graphics: Graphics,
}

impl Game {
    pub fn new(_event_loop: &EventLoop<()>, main_window: &Window) -> anyhow::Result<Self> {
        Ok(Game {
            graphics: Graphics::new(main_window)?,
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
        self.graphics.queue.submit(Some(command_buffer));
        output.present();
    }
}
