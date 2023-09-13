use glam::UVec2;
use std::path::Path;
use std::sync::Arc;

use crate::game::GameResult;
use crate::graphics::GraphicsContext;

#[derive(Debug)]
struct TextureData {
    view: wgpu::TextureView,
    bind_group: wgpu::BindGroup,
}

#[derive(Clone, Debug)]
pub struct Texture {
    data: Arc<TextureData>,
    size: UVec2,
}

impl Texture {
    pub fn from_file<G, P>(graphics: G, path: P) -> GameResult<Self>
    where
        G: AsRef<GraphicsContext>,
        P: AsRef<Path>,
    {
        todo!()
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.data.view
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.data.bind_group
    }

    pub fn size(&self) -> UVec2 {
        self.size
    }
}
