use crate::{
  algebra::values::{Bivector, Trivector, Vector},
  geometry::objects::{Line, Plane, Point},
};

/// b ∨ (a ∧ b<sup>☆</sup>)
#[inline]
pub fn orthogonal_projection<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as OrthogonalProjection<Rhs>>::Output
where
  Lhs: OrthogonalProjection<Rhs>,
{
  a.orthogonal_projection(b)
}

/// b ∨ (a ∧ b<sup>☆</sup>)
pub trait OrthogonalProjection<Rhs> {
  type Output;

  /// b ∨ (a ∧ b<sup>☆</sup>)
  fn orthogonal_projection(self, b: Rhs) -> Self::Output;
}

impl OrthogonalProjection<Line> for Point {
  type Output = Point;

  #[rustfmt::skip]
  #[inline]
  fn orthogonal_projection(self, b: Line) -> Self::Output {
    let Point(Vector { e1: l1, e2: l2, e3: l3, e4: l4 }) = self;
    let Line(Bivector {
      e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    }) = b;

    let x = r12*r42 - r31*r43;
    let y = r23*r43 - r12*r41;
    let z = r31*r41 - r23*r42;

    let w = r41*r41 + r42*r42 + r43*r43;
    let a =  l1*r41 +  l2*r42 +  l3*r43;

    let e1 = a*r41 + l4*x;
    let e2 = a*r42 + l4*y;
    let e3 = a*r43 + l4*z;
    let e4 =         l4*w;

    Point(Vector { e1, e2, e3, e4 })
  }
}

impl OrthogonalProjection<Plane> for Point {
  type Output = Point;

  #[rustfmt::skip]
  #[inline]
  fn orthogonal_projection(self, b: Plane) -> Self::Output {
    let Point(Vector { e1: l1, e2: l2, e3: l3, e4: l4 }) = self;
    let Plane(Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }) = b;

    let x0 = r423*r423;
    let y0 = r431*r431;
    let z0 = r412*r412;

    let x1 = r431*r412;
    let y1 = r412*r423;
    let z1 = r423*r431;

    let a = x0 + y0 + z0;
    let b = l4*a;
    let c = l4*r321;

    let x2 = c*r423;
    let y2 = c*r431;
    let z2 = c*r412;

    let x3 = l1*z0 - l3*y1;
    let y3 = l2*x0 - l1*z1;
    let z3 = l3*y0 - l2*x1;

    let x4 = l1*y0 - l2*z1;
    let y4 = l2*z0 - l3*x1;
    let z4 = l3*x0 - l1*y1;

    let e1 = x3 + x4 - x2;
    let e2 = y3 + y4 - y2;
    let e3 = z3 + z4 - z2;

    let e4 = b;

    Point(Vector { e1, e2, e3, e4 })
  }
}

impl OrthogonalProjection<Plane> for Line {
  type Output = Line;

  #[rustfmt::skip]
  #[inline]
  fn orthogonal_projection(self, b: Plane) -> Self::Output {
    let Line(Bivector {
      e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    }) = self;
    let Plane(Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }) = b;

    let a = l23*r423 + l31*r431 + l12*r412;

    let x = l43*r431 - l42*r412;
    let y = l41*r412 - l43*r423;
    let z = l42*r423 - l41*r431;

    let e41 = y*r412 - z*r431;
    let e42 = z*r423 - x*r412;
    let e43 = x*r431 - y*r423;

    let e23 = a*r423 - x*r321;
    let e31 = a*r431 - y*r321;
    let e12 = a*r412 - z*r321;

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
        orthogonal_projection(point, line),
        Point(antiwedge(line.0, weight_expansion(point.0, line.0)))
      );
    }
    {
      let point = Point(grade_1(MULTIVECTOR_A));
      let plane = Plane(grade_3(MULTIVECTOR_B));

      assert_eq!(
        orthogonal_projection(point, plane),
        Point(antiwedge(plane.0, weight_expansion(point.0, plane.0)))
      );
    }
    {
      let line = Line(grade_2(MULTIVECTOR_A));
      let plane = Plane(grade_3(MULTIVECTOR_B));

      assert_eq!(
        orthogonal_projection(line, plane),
        Line(antiwedge(plane.0, weight_expansion(line.0, plane.0)))
      );
    }
  }
}
