use crate::{
  algebra::operators::AntiwedgeProduct,
  geometry::objects::{Line, Plane, Point},
};

/// a ∨ b
#[inline]
pub fn meet<Lhs, Rhs>(a: Lhs, b: Rhs) -> <Lhs as Meet<Rhs>>::Output
where
  Lhs: Meet<Rhs>,
{
  a.meet(b)
}

/// a ∨ b
pub trait Meet<Rhs> {
  type Output;

  /// a ∨ b
  fn meet(self, b: Rhs) -> Self::Output;
}

impl Meet<Plane> for Plane {
  type Output = Line;

  #[inline]
  fn meet(self, b: Plane) -> Self::Output {
    Line(self.0.antiwedge(b.0))
  }
}

impl Meet<Plane> for Line {
  type Output = Point;

  #[inline]
  fn meet(self, b: Plane) -> Self::Output {
    Point(self.0.antiwedge(b.0))
  }
}

impl Meet<Line> for Plane {
  type Output = Point;

  #[inline]
  fn meet(self, b: Line) -> Self::Output {
    Point(self.0.antiwedge(b.0))
  }
}
