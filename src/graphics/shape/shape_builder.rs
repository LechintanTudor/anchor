use crate::graphics::shape::{Shape, ShapeVertex};
use crate::graphics::{Color, WgpuContext};
use glam::Vec2;
use lyon::math::Point;
use lyon::path::path::BuilderWithAttributes;
use lyon::path::Path;
use lyon::tessellation::{BuffersBuilder, FillTessellator, FillVertex, VertexBuffers};
use std::mem;

type PathBuilder = BuilderWithAttributes;

impl Shape {
    #[inline]
    pub fn builder() -> ShapeBuilder {
        ShapeBuilder::default()
    }
}

pub struct ShapeBuilder {
    path_builder: PathBuilder,
    active_color: Color,
}

impl ShapeBuilder {
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.active_color = color;
        self
    }

    pub fn begin<P>(&mut self, point: P) -> &mut Self
    where
        P: Into<Vec2>,
    {
        self.path_builder
            .begin(convert_point(point), &self.attributes());

        self
    }

    pub fn line_to<P>(&mut self, point: P) -> &mut Self
    where
        P: Into<Vec2>,
    {
        self.path_builder
            .line_to(convert_point(point), &self.attributes());

        self
    }

    pub fn end(&mut self) -> &mut Self {
        self.path_builder.end(true);
        self
    }

    pub fn build<W>(&mut self, wgpu: W) -> Shape
    where
        W: AsRef<WgpuContext>,
    {
        let path_builder = mem::replace(&mut self.path_builder, Path::builder_with_attributes(4));
        let path = path_builder.build();

        let mut buffers = VertexBuffers::<ShapeVertex, u16>::new();
        let mut buffers_builder = BuffersBuilder::new(&mut buffers, convert_vertex);

        FillTessellator::new()
            .tessellate_path(&path, &Default::default(), &mut buffers_builder)
            .unwrap();

        Shape::new(wgpu, &buffers.vertices, &buffers.indices)
    }

    fn attributes(&self) -> [f32; 4] {
        self.active_color.as_array()
    }
}

impl Default for ShapeBuilder {
    fn default() -> Self {
        Self {
            path_builder: Path::builder_with_attributes(4),
            active_color: Color::WHITE,
        }
    }
}

fn convert_point<P>(point: P) -> Point
where
    P: Into<Vec2>,
{
    let vec = point.into();
    Point::new(vec.x, vec.y)
}

fn convert_vertex(mut vertex: FillVertex) -> ShapeVertex {
    let attributes = vertex.interpolated_attributes();
    let color = Color::rgba(attributes[0], attributes[1], attributes[2], attributes[3]);

    ShapeVertex {
        position: vertex.position().to_array().into(),
        linear_color: color.to_linear_vec4(),
        ..Default::default()
    }
}
