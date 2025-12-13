use crate::{
  algebra::values::{Bivector, Trivector, Vector},
  geometry::objects::{Line, Plane, Point},
};

/// b ∨ (a ∧ b<sup>★</sup>)
#[inline]
pub fn central_projection<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as CentralProjection<Rhs>>::Output
where
  Lhs: CentralProjection<Rhs>,
{
  a.central_projection(b)
}

/// b ∨ (a ∧ b<sup>★</sup>)
pub trait CentralProjection<Rhs> {
  type Output;

  /// b ∨ (a ∧ b<sup>★</sup>)
  fn central_projection(self, b: Rhs) -> Self::Output;
}

impl CentralProjection<Line> for Point {
  type Output = Point;

  #[rustfmt::skip]
  #[inline]
  fn central_projection(self, b: Line) -> Self::Output {
    let Point(Vector { e1: l1, e2: l2, e3: l3, .. }) = self;
    let Line(Bivector {
      e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    }) = b;

    let x = l3*r31 - l2*r12;
    let y = l1*r12 - l3*r23;
    let z = l2*r23 - l1*r31;

    let e1 = y*r12 - z*r31;
    let e2 = z*r23 - x*r12;
    let e3 = x*r31 - y*r23;
    let e4 = x*r41 + y*r42 + z*r43;

    Point(Vector { e1, e2, e3, e4 })
  }
}

impl CentralProjection<Plane> for Point {
  type Output = Point;

  #[rustfmt::skip]
  #[inline]
  fn central_projection(self, b: Plane) -> Self::Output {
    let Point(Vector { e1: l1, e2: l2, e3: l3, .. }) = self;
    let Plane(Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }) = b;

    let x = l1*r321;
    let y = l2*r321;
    let z = l3*r321;

    let e1 = x*r321;
    let e2 = y*r321;
    let e3 = z*r321;
    let e4 = - x*r423 - y*r431 - z*r412;

    Point(Vector { e1, e2, e3, e4 })
  }
}

impl CentralProjection<Plane> for Line {
  type Output = Line;

  #[rustfmt::skip]
  #[inline]
  fn central_projection(self, b: Plane) -> Self::Output {
    let Line(Bivector { e23: l23, e31: l31, e12: l12, .. }) = self;
    let Plane(Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }) = b;

    let x = l23*r321;
    let y = l31*r321;
    let z = l12*r321;

    let e41 = z*r431 - y*r412;
    let e42 = x*r412 - z*r423;
    let e43 = y*r423 - x*r431;
    let e23 = x*r321;
    let e31 = y*r321;
    let e12 = z*r321;

    Line(Bivector { e41, e42, e43, e23, e31, e12 })
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    algebra::operators::*,
    geometry::{objects::*, operators::*},
    test_values::*,
  };

  #[test]
  fn definition() {
    {
      let point = Point(grade_1(MULTIVECTOR_A));
      let line = Line(grade_2(MULTIVECTOR_B));

      assert_eq!(
        central_projection(point, line),
        Point(antiwedge(line.0, bulk_expansion(point.0, line.0)))
      );
    }
    {
      let point = Point(grade_1(MULTIVECTOR_A));
      let plane = Plane(grade_3(MULTIVECTOR_B));

      assert_eq!(
        central_projection(point, plane),
        Point(antiwedge(plane.0, bulk_expansion(point.0, plane.0)))
      );
    }
    {
      let line = Line(grade_2(MULTIVECTOR_A));
      let plane = Plane(grade_3(MULTIVECTOR_B));

      assert_eq!(
        central_projection(line, plane),
        Line(antiwedge(plane.0, bulk_expansion(line.0, plane.0)))
      );
    }
  }
}
