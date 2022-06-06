use crate::graphics::{Vec2, Vec4};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ShapeVertex {
    pub position: Vec2,
    _padding: [f32; 2],
    pub linear_color: Vec4,
}

impl ShapeVertex {
    pub const fn new(position: Vec2, linear_color: Vec4) -> Self {
        Self { position, _padding: [0.0; 2], linear_color }
    }
}

pub unsafe trait Shape {
    type Vertexes: Iterator<Item = ShapeVertex>;
    type Indexes: Iterator<Item = u32>;

    fn vertexes(&self) -> Self::Vertexes;

    fn indexes(&self) -> Self::Indexes;
}
