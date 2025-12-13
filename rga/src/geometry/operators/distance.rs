use crate::{algebra::DualNumber, geometry::objects::Magnitude};
#[cfg(feature = "_math")]
use crate::{
  geometry::objects::{Line, Plane, Point},
  optional::math::Math,
};

/// The euclidean distance between two objects as a homogeneous magnitude
#[inline]
pub fn distance<Lhs, Rhs>(a: Lhs, b: Rhs) -> Magnitude
where
  Lhs: Distance<Rhs>,
{
  a.distance(b)
}

/// The signed euclidean distance between two objects as a homogeneous
/// magnitude
#[inline]
pub fn signed_distance<Lhs, Rhs>(a: Lhs, b: Rhs) -> Magnitude
where
  Lhs: SignedDistance<Rhs>,
{
  a.signed_distance(b)
}

/// The euclidean distance between two objects as a homogeneous magnitude
pub trait Distance<Rhs> {
  /// The euclidean distance between two objects as a homogeneous magnitude
  fn distance(self, b: Rhs) -> Magnitude;
}

/// The signed euclidean distance between two objects as a homogeneous
/// magnitude
pub trait SignedDistance<Rhs> {
  /// The signed euclidean distance between two objects as a homogeneous
  /// magnitude
  fn signed_distance(self, b: Rhs) -> Magnitude;
}

impl<Lhs, Rhs> Distance<Rhs> for Lhs
where
  Lhs: SignedDistance<Rhs>,
{
  #[inline]
  fn distance(self, b: Rhs) -> Magnitude {
    let signed = self.signed_distance(b);
    // note: in general need to also take the absolute value of the e1234
    // component, however the signed distances of all geometry types that
    // currently exist already satisfy this constraint.
    Magnitude(DualNumber(signed.0.s.abs(), signed.0.e1234))
  }
}

#[cfg(feature = "_math")]
impl SignedDistance<Point> for Point {
  #[inline]
  fn signed_distance(self, b: Point) -> Magnitude {
    let (Point(a), Point(b)) = (self, b);
    let x = a.e4 * b.e1 - a.e1 * b.e4;
    let y = a.e4 * b.e2 - a.e2 * b.e4;
    let z = a.e4 * b.e3 - a.e3 * b.e4;
    let s = Math::sqrt(x * x + y * y + z * z);
    // discard the sign for point-point distance
    let e1234 = (a.e4 * b.e4).abs();
    Magnitude(DualNumber(s, e1234))
  }
}

#[cfg(feature = "_math")]
impl SignedDistance<Line> for Point {
  #[inline]
  fn signed_distance(self, b: Line) -> Magnitude {
    let (Point(a), Line(b)) = (self, b);
    let s = {
      let x = a.e4 * b.e23 + a.e3 * b.e42 - a.e2 * b.e43;
      let y = a.e4 * b.e31 + a.e1 * b.e43 - a.e3 * b.e41;
      let z = a.e4 * b.e12 + a.e2 * b.e41 - a.e1 * b.e42;
      Math::sqrt(x * x + y * y + z * z)
    };
    let e1234 = {
      let x = a.e4 * b.e41;
      let y = a.e4 * b.e42;
      let z = a.e4 * b.e43;
      Math::sqrt(x * x + y * y + z * z)
    };
    Magnitude(DualNumber(s, e1234))
  }
}

#[cfg(feature = "_math")]
impl SignedDistance<Point> for Line {
  #[inline]
  fn signed_distance(self, b: Point) -> Magnitude {
    SignedDistance::signed_distance(b, self)
  }
}

#[cfg(feature = "_math")]
impl SignedDistance<Plane> for Point {
  #[inline]
  fn signed_distance(self, b: Plane) -> Magnitude {
    let (Point(a), Plane(b)) = (self, b);
    let s = a.e1 * b.e423 + a.e2 * b.e431 + a.e3 * b.e412 + a.e4 * b.e321;
    let e1234 = {
      let x = a.e4 * b.e423;
      let y = a.e4 * b.e431;
      let z = a.e4 * b.e412;
      Math::sqrt(x * x + y * y + z * z)
    };
    Magnitude(DualNumber(s, e1234))
  }
}

#[cfg(feature = "_math")]
impl SignedDistance<Point> for Plane {
  #[inline]
  fn signed_distance(self, b: Point) -> Magnitude {
    SignedDistance::signed_distance(b, self)
  }
}

#[cfg(feature = "_math")]
impl SignedDistance<Line> for Line {
  #[inline]
  fn signed_distance(self, b: Line) -> Magnitude {
    let (Line(a), Line(b)) = (self, b);
    let s = -((a.e41 * b.e23 + a.e23 * b.e41)
      + (a.e42 * b.e31 + a.e31 * b.e42)
      + (a.e43 * b.e12 + a.e12 * b.e43));
    let e1234 = {
      let x = a.e42 * b.e43 - a.e43 * b.e42;
      let y = a.e43 * b.e41 - a.e41 * b.e43;
      let z = a.e41 * b.e42 - a.e42 * b.e41;
      Math::sqrt(x * x + y * y + z * z)
    };
    Magnitude(DualNumber(s, e1234))
  }
}

#[cfg(feature = "_math")]
#[cfg(test)]
mod tests {
  use crate::{
    algebra::operators::*,
    geometry::{distance, objects::*},
    test_values::*,
  };

  #[test]
  fn point_point() {
    let point_a: Point = Point(grade_1(MULTIVECTOR_A));
    let point_b: Point = Point(grade_1(MULTIVECTOR_B));
    assert_eq!(
      unitize(dbg!(distance(point_a, point_b))),
      unitize(Magnitude(dbg!(
        bulk_norm(attitude(wedge(point_a.0, point_b.0)))
          + weight_norm(wedge(point_a.0, attitude(point_b.0)))
      )))
    );
  }

  #[test]
  fn point_line() {
    let point: Point = Point(grade_1(MULTIVECTOR_A));
    let line: Line = Line(grade_2(MULTIVECTOR_B));
    assert_eq!(
      unitize(dbg!(distance(point, line))),
      unitize(Magnitude(dbg!(
        bulk_norm(attitude(wedge(point.0, line.0)))
          + weight_norm(wedge(point.0, attitude(line.0)))
      )))
    );
  }

  #[test]
  fn line_point() {
    let line: Line = Line(grade_2(MULTIVECTOR_A));
    let point: Point = Point(grade_1(MULTIVECTOR_B));
    assert_eq!(
      unitize(dbg!(distance(line, point))),
      unitize(Magnitude(dbg!(
        bulk_norm(attitude(wedge(line.0, point.0)))
          + weight_norm(wedge(line.0, attitude(point.0)))
      )))
    );
  }

  #[test]
  fn point_plane() {
    let point: Point = Point(grade_1(MULTIVECTOR_A));
    let plane: Plane = Plane(grade_3(MULTIVECTOR_B));
    assert_eq!(
      unitize(dbg!(distance(point, plane))),
      unitize(Magnitude(dbg!(
        bulk_norm(attitude(wedge(point.0, plane.0)))
          + weight_norm(wedge(point.0, attitude(plane.0)))
      )))
    );
  }

  #[test]
  fn plane_point() {
    let plane: Plane = Plane(grade_3(MULTIVECTOR_A));
    let point: Point = Point(grade_1(MULTIVECTOR_B));
    assert_eq!(
      unitize(dbg!(distance(plane, point))),
      unitize(Magnitude(dbg!(
        bulk_norm(attitude(wedge(plane.0, point.0)))
          + weight_norm(wedge(plane.0, attitude(point.0)))
      )))
    );
  }

  #[test]
  fn line_line() {
    let line_a: Line = Line(grade_2(MULTIVECTOR_A));
    let line_b: Line = Line(grade_2(MULTIVECTOR_B));
    assert_eq!(
      unitize(dbg!(distance(line_a, line_b))),
      unitize(Magnitude(dbg!(
        bulk_norm(attitude(wedge(line_a.0, line_b.0)))
          + weight_norm(wedge(line_a.0, attitude(line_b.0)))
      )))
    );
  }
}
