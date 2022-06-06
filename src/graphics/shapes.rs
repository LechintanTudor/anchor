use crate::graphics::{Color, Shape, ShapeVertex, Vec2};

#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub width: f32,
    pub height: f32,
    pub color: Color,
}

impl Rect {
    pub fn new(width: f32, height: f32, color: Color) -> Self {
        Self { width, height, color }
    }
}

unsafe impl Shape for Rect {
    type Vertexes = <[ShapeVertex; 4] as IntoIterator>::IntoIter;
    type Indexes = <[u32; 6] as IntoIterator>::IntoIter;

    fn vertexes(&self) -> Self::Vertexes {
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let linear_color = self.color.to_linear_vec4();

        let vertexes = [
            ShapeVertex::new(Vec2::new(-half_width, half_height), linear_color),
            ShapeVertex::new(Vec2::new(-half_width, -half_height), linear_color),
            ShapeVertex::new(Vec2::new(half_width, -half_height), linear_color),
            ShapeVertex::new(Vec2::new(half_width, half_height), linear_color),
        ];

        vertexes.into_iter()
    }

    fn indexes(&self) -> Self::Indexes {
        [0, 1, 3, 3, 1, 2].into_iter()
    }
}
