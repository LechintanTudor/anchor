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
    fn write(&self, vertexes: &mut Vec<ShapeVertex>, indexes: &mut Vec<u32>) {
        let base_index = u32::try_from(vertexes.len()).expect("Vertex index overflow");

        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let linear_color = self.color.to_linear_vec4();

        vertexes.extend([
            ShapeVertex::new(Vec2::new(-half_width, half_height), linear_color),
            ShapeVertex::new(Vec2::new(-half_width, -half_height), linear_color),
            ShapeVertex::new(Vec2::new(half_width, -half_height), linear_color),
            ShapeVertex::new(Vec2::new(half_width, half_height), linear_color),
        ]);

        indexes.extend([
            base_index,
            base_index + 1,
            base_index + 3,
            base_index + 3,
            base_index + 2,
            base_index + 1,
        ]);
    }
}
