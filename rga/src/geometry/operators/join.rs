use crate::{
  algebra::operators::WedgeProduct,
  geometry::objects::{Line, Plane, Point},
};

/// a ∧ b
#[inline]
pub fn join<Lhs, Rhs>(a: Lhs, b: Rhs) -> <Lhs as Join<Rhs>>::Output
where
  Lhs: Join<Rhs>,
{
  a.join(b)
}

/// a ∧ b
pub trait Join<Rhs> {
  type Output;

  /// a ∧ b
  fn join(self, b: Rhs) -> Self::Output;
}

impl Join<Point> for Point {
  type Output = Line;

  #[inline]
  fn join(self, b: Point) -> Self::Output {
    Line(self.0.wedge(b.0))
  }
}

impl Join<Point> for Line {
  type Output = Plane;

  #[inline]
  fn join(self, b: Point) -> Self::Output {
    Plane(self.0.wedge(b.0))
  }
}

impl Join<Line> for Point {
  type Output = Plane;

  #[inline]
  fn join(self, b: Line) -> Self::Output {
    Plane(self.0.wedge(b.0))
  }
}
