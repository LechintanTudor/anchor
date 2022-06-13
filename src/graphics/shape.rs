use crate::graphics::ShapeVertex;

pub unsafe trait Shape {
    type Vertexes: Iterator<Item = ShapeVertex>;
    type Indexes: Iterator<Item = u32>;

    fn vertexes(&self) -> Self::Vertexes;

    fn indexes(&self) -> Self::Indexes;
}
