use crate::graphics::shape::{Shape, ShapeVertex};
use crate::graphics::{Bounds, Color, WgpuContext};
use glam::Vec2;
use lyon::geom::Box2D;
use lyon::math::{Angle, Point, Vector};
use lyon::path::builder::BorderRadii as LyonBorderRadii;
use lyon::path::path::BuilderWithAttributes;
use lyon::path::traits::PathBuilder as _;
use lyon::path::{Path, Winding};
use lyon::tessellation::{BuffersBuilder, FillTessellator, FillVertex, VertexBuffers};
use std::mem;

type PathBuilder = BuilderWithAttributes;

#[derive(Clone, Copy, Default, Debug)]
pub struct BorderRadii {
    pub top_left: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
    pub top_right: f32,
}

impl From<f32> for BorderRadii {
    fn from(value: f32) -> Self {
        Self {
            top_left: value,
            bottom_left: value,
            bottom_right: value,
            top_right: value,
        }
    }
}

impl From<[f32; 4]> for BorderRadii {
    fn from(values: [f32; 4]) -> Self {
        Self {
            top_left: values[0],
            bottom_left: values[1],
            bottom_right: values[2],
            top_right: values[3],
        }
    }
}

impl Shape {
    #[inline]
    pub fn builder() -> ShapeBuilder {
        Default::default()
    }
}

#[derive(Clone)]
pub struct ShapeBuilder {
    path_builder: PathBuilder,
    active_color: Color,
}

impl ShapeBuilder {
    #[inline]
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

    #[inline]
    pub fn end(&mut self) -> &mut Self {
        self.path_builder.end(true);
        self
    }

    pub fn rect<B>(&mut self, bounds: B) -> &mut Self
    where
        B: Into<Bounds>,
    {
        let bounds = bounds.into();
        let bounds = Box2D {
            min: convert_point(bounds.top_left()),
            max: convert_point(bounds.bottom_right()),
        };

        self.path_builder
            .add_rectangle(&bounds, Winding::Positive, &self.attributes());

        self
    }

    pub fn rounded_rect<B, R>(&mut self, bounds: B, radii: R) -> &mut Self
    where
        B: Into<Bounds>,
        R: Into<BorderRadii>,
    {
        let bounds = bounds.into();
        let bounds = Box2D {
            min: convert_point(bounds.top_left()),
            max: convert_point(bounds.bottom_right()),
        };

        let radii = radii.into();
        let radii = LyonBorderRadii {
            top_left: radii.top_left,
            bottom_left: radii.bottom_left,
            bottom_right: radii.bottom_right,
            top_right: radii.top_right,
        };

        self.path_builder.add_rounded_rectangle(
            &bounds,
            &radii,
            Winding::Positive,
            &self.attributes(),
        );

        self
    }

    pub fn ellipse<C, R>(&mut self, center: C, radii: R, x_rotation: f32) -> &mut Self
    where
        C: Into<Vec2>,
        R: Into<Vec2>,
    {
        self.path_builder.add_ellipse(
            convert_point(center),
            convert_vector(radii),
            Angle::radians(x_rotation),
            Winding::Positive,
            &self.attributes(),
        );

        self
    }

    pub fn circle<C>(&mut self, center: C, radius: f32) -> &mut Self
    where
        C: Into<Vec2>,
    {
        self.path_builder.add_circle(
            convert_point(center),
            radius,
            Winding::Positive,
            &self.attributes(),
        );

        self
    }

    pub fn build<W>(&mut self, wgpu: W) -> Shape
    where
        W: AsRef<WgpuContext>,
    {
        let path_builder = mem::replace(&mut self.path_builder, new_path_builder());
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
    #[inline]
    fn default() -> Self {
        Self {
            path_builder: new_path_builder(),
            active_color: Color::WHITE,
        }
    }
}

fn new_path_builder() -> BuilderWithAttributes {
    Path::builder_with_attributes(4)
}

fn convert_point<P>(point: P) -> Point
where
    P: Into<Vec2>,
{
    let vec = point.into();
    Point::new(vec.x, vec.y)
}

fn convert_vector<V>(vector: V) -> Vector
where
    V: Into<Vec2>,
{
    let vec = vector.into();
    Vector::new(vec.x, vec.y)
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
