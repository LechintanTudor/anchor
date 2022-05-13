use crate::graphics::{Color, Transform};
use glam::f32::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct ShapeVertexData {
    pub position: Vec2,
}

impl ShapeVertexData {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}

pub unsafe trait Shape {
    type Vertexes: Iterator<Item = ShapeVertexData>;
    type Indexes: Iterator<Item = u32>;

    fn vertexes(&self) -> Self::Vertexes;

    fn indexes(&self) -> Self::Indexes;
}

#[derive(Clone, Copy, Default, Debug)]
pub struct ShapeStyle {
    pub color: Color,
    pub transform: Transform,
}

impl ShapeStyle {
    pub const fn new(color: Color, transform: Transform) -> Self {
        Self { color, transform }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rectangle {
    pub half_width: f32,
    pub half_height: f32,
}

impl Rectangle {
    pub fn with_dimensions(width: f32, height: f32) -> Self {
        Self { half_width: width / 2.0, half_height: height / 2.0 }
    }
}

unsafe impl Shape for Rectangle {
    type Vertexes = <[ShapeVertexData; 4] as IntoIterator>::IntoIter;
    type Indexes = <[u32; 6] as IntoIterator>::IntoIter;

    fn vertexes(&self) -> Self::Vertexes {
        [
            ShapeVertexData::new(Vec2::new(-self.half_width, -self.half_height)),
            ShapeVertexData::new(Vec2::new(-self.half_width, self.half_height)),
            ShapeVertexData::new(Vec2::new(self.half_width, self.half_height)),
            ShapeVertexData::new(Vec2::new(self.half_width, -self.half_height)),
        ]
        .into_iter()
    }

    fn indexes(&self) -> Self::Indexes {
        [0, 1, 2, 0, 2, 3].into_iter()
    }
}
