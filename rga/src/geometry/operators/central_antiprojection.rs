use crate::{
  algebra::values::{Bivector, Trivector, Vector},
  geometry::objects::{Line, Plane, Point},
};

/// b ∧ (a ∨ b<sup>★</sup>)
#[inline]
pub fn central_antiprojection<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as CentralAntiprojection<Rhs>>::Output
where
  Lhs: CentralAntiprojection<Rhs>,
{
  a.central_antiprojection(b)
}

/// b ∧ (a ∨ b<sup>★</sup>)
pub trait CentralAntiprojection<Rhs> {
  type Output;

  /// b ∧ (a ∨ b<sup>★</sup>)
  fn central_antiprojection(self, b: Rhs) -> Self::Output;
}

impl CentralAntiprojection<Point> for Plane {
  type Output = Plane;

  #[rustfmt::skip]
  #[inline]
  fn central_antiprojection(self, b: Point) -> Self::Output {
    let Plane(Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }) = self;
    let Point(Vector { e1: r1, e2: r2, e3: r3, e4: r4 }) = b;

    let c41 = l412*r2 - l431*r3;
    let c42 = l423*r3 - l412*r1;
    let c43 = l431*r1 - l423*r2;
    let c23 = -l321*r1;
    let c31 = -l321*r2;
    let c12 = -l321*r3;

    let e423 = r4*c23 + r3*c42 - r2*c43;
    let e431 = r4*c31 + r1*c43 - r3*c41;
    let e412 = r4*c12 + r2*c41 - r1*c42;
    let e321 = - r1*c23 - r3*c12 - r2*c31;

    Plane(Trivector { e423, e431, e412, e321 })
  }
}

impl CentralAntiprojection<Line> for Plane {
  type Output = Plane;

  #[rustfmt::skip]
  #[inline]
  fn central_antiprojection(self, b: Line) -> Self::Output {
    let Plane(Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }) = self;
    let Line(Bivector {
      e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    }) = b;

    let x = l321*r23;
    let y = l321*r31;
    let z = l321*r12;

    let a    = r23*l423 + r31*l431 + r12*l412;
    let e321 = r23*x    + r31*y    + r12*z;

    let e423 = a*r23 + y*r43 - z*r42;
    let e431 = a*r31 + z*r41 - x*r43;
    let e412 = a*r12 + x*r42 - y*r41;

    Plane(Trivector { e423, e431, e412, e321 })
  }
}

impl CentralAntiprojection<Point> for Line {
  type Output = Line;

  #[rustfmt::skip]
  #[inline]
  fn central_antiprojection(self, b: Point) -> Self::Output {
    let Line(Bivector {
      e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12
    }) = self;
    let Point(Vector { e1: r1, e2: r2, e3: r3, e4: r4 }) = b;

    let x = l12*r2 - l31*r3;
    let y = l23*r3 - l12*r1;
    let z = l31*r1 - l23*r2;

    let a = l41*r1 + l42*r2 + l43*r3;

    let e41 = a*r1 - x*r4;
    let e42 = a*r2 - y*r4;
    let e43 = a*r3 - z*r4;
    let e23 = y*r3 - z*r2;
    let e31 = z*r1 - x*r3;
    let e12 = x*r2 - y*r1;

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
      let plane = Plane(grade_3(MULTIVECTOR_A));
      let point = Point(grade_1(MULTIVECTOR_B));

      assert_eq!(
        central_antiprojection(plane, point),
        Plane(wedge(point.0, bulk_contraction(plane.0, point.0)))
      );
    }
    {
      let plane = Plane(grade_3(MULTIVECTOR_A));
      let line = Line(grade_2(MULTIVECTOR_B));

      assert_eq!(
        central_antiprojection(plane, line),
        Plane(wedge(line.0, bulk_contraction(plane.0, line.0)))
      );
    }
    {
      let line = Line(grade_2(MULTIVECTOR_A));
      let point = Point(grade_1(MULTIVECTOR_B));

      assert_eq!(
        central_antiprojection(line, point),
        Line(wedge(point.0, bulk_contraction(line.0, point.0)))
      );
    }
  }
}
