use crate::{
  algebra::operators::{GradeSelect, SandwichAntiproduct},
  geometry::{
    objects::{Line, Plane, Point},
    transformations::{Flector, Inversion, Motor, Reflection},
  },
};

/// Transform an object
#[inline]
pub fn transform<Lhs, Rhs>(transformation: Lhs, object: Rhs) -> Rhs
where
  Lhs: Transform<Rhs>,
{
  transformation.transform(object)
}

/// Transform an object
pub trait Transform<Object> {
  /// Transform an object
  fn transform(&self, object: Object) -> Object;
}

impl Transform<Point> for Reflection {
  fn transform(&self, point: Point) -> Point {
    Point(self.0.antisandwich(point.0))
  }
}

impl Transform<Line> for Reflection {
  fn transform(&self, line: Line) -> Line {
    Line(self.0.antisandwich(line.0))
  }
}

impl Transform<Plane> for Reflection {
  fn transform(&self, plane: Plane) -> Plane {
    Plane(self.0.antisandwich(plane.0))
  }
}

impl Transform<Point> for Inversion {
  fn transform(&self, point: Point) -> Point {
    Point(self.0.antisandwich(point.0))
  }
}

impl Transform<Line> for Inversion {
  fn transform(&self, line: Line) -> Line {
    Line(self.0.antisandwich(line.0))
  }
}

impl Transform<Plane> for Inversion {
  fn transform(&self, plane: Plane) -> Plane {
    Plane(self.0.antisandwich(plane.0))
  }
}

impl Transform<Point> for Motor {
  fn transform(&self, point: Point) -> Point {
    let transformed = self.0.antisandwich(point.0);
    // We assume that the motor satisfies IsGeometric, and discard the e321
    // component of the result which is expected to be 0.
    let transformed = transformed.grade_1();
    Point(transformed)
  }
}

impl Transform<Line> for Motor {
  fn transform(&self, line: Line) -> Line {
    Line(self.0.antisandwich(line.0))
  }
}

impl Transform<Plane> for Motor {
  fn transform(&self, plane: Plane) -> Plane {
    let transformed = self.0.antisandwich(plane.0);
    // We assume that the motor satisfies IsGeometric, and discard the vector
    // components which are expected to be zero.
    let transformed = transformed.grade_3();
    Plane(transformed)
  }
}

impl Transform<Point> for Flector {
  fn transform(&self, point: Point) -> Point {
    let transformed = self.0.antisandwich(point.0);
    // We assume that the motor satisfies IsGeometric, and discard the e321
    // component of the result which is expected to be 0.
    let transformed = transformed.grade_1();
    Point(transformed)
  }
}

impl Transform<Line> for Flector {
  fn transform(&self, line: Line) -> Line {
    Line(self.0.antisandwich(line.0))
  }
}

impl Transform<Plane> for Flector {
  fn transform(&self, plane: Plane) -> Plane {
    let transformed = self.0.antisandwich(plane.0);
    // We assume that the motor satisfies IsGeometric, and discard the vector
    // components of the result which are expected to be 0.
    let transformed = transformed.grade_3();
    Plane(transformed)
  }
}
