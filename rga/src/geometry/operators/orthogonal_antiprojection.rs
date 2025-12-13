use crate::{
  algebra::values::{Bivector, Trivector, Vector},
  geometry::objects::{Line, Plane, Point},
};

/// b ∧ (a ∨ b<sup>☆</sup>)
#[inline]
pub fn orthogonal_antiprojection<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as OrthogonalAntiprojection<Rhs>>::Output
where
  Lhs: OrthogonalAntiprojection<Rhs>,
{
  a.orthogonal_antiprojection(b)
}

/// b ∧ (a ∨ b<sup>☆</sup>)
pub trait OrthogonalAntiprojection<Rhs> {
  type Output;

  /// b ∧ (a ∨ b<sup>☆</sup>)
  fn orthogonal_antiprojection(self, b: Rhs) -> Self::Output;
}

impl OrthogonalAntiprojection<Line> for Plane {
  type Output = Plane;

  #[rustfmt::skip]
  #[inline]
  fn orthogonal_antiprojection(self, b: Line) -> Plane {
    let Plane(Trivector{ e423: l423, e431: l431, e412: l412, .. }) = self;
    let Line(Bivector{
      e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    }) = b;

    let x = l412*r42 - l431*r43;
    let y = l423*r43 - l412*r41;
    let z = l431*r41 - l423*r42;

    let e423 = r43*y - r42*z;
    let e431 = r41*z - r43*x;
    let e412 = r42*x - r41*y;
    let e321 = r23*x + r12*z + r31*y;

    Plane(Trivector { e423, e431, e412, e321 })
  }
}

impl OrthogonalAntiprojection<Point> for Plane {
  type Output = Plane;

  #[rustfmt::skip]
  #[inline]
  fn orthogonal_antiprojection(self, b: Point) -> Self::Output {
    let Plane(Trivector{ e423: l423, e431: l431, e412: l412, .. }) = self;
    let Point(Vector { e1: r1, e2: r2, e3: r3, e4: r4 }) = b;

    let x = r4*l423;
    let y = r4*l431;
    let z = r4*l412;

    let e423 = r4*x;
    let e431 = r4*y;
    let e412 = r4*z;
    let e321 = - r1*x - r2*y - r3*z;

    Plane(Trivector { e423, e431, e412, e321 })
  }
}

impl OrthogonalAntiprojection<Point> for Line {
  type Output = Line;

  #[rustfmt::skip]
  #[inline]
  fn orthogonal_antiprojection(self, b: Point) -> Line {
    let Line(Bivector{ e41: l41, e42: l42, e43: l43, .. }) = self;
    let Point(Vector { e1: r1, e2: r2, e3: r3, e4: r4 }) = b;

    let x = r4*l41;
    let y = r4*l42;
    let z = r4*l43;

    let e41 = r4*x;
    let e42 = r4*y;
    let e43 = r4*z;
    let e23 = r2*z - r3*y;
    let e31 = r3*x - r1*z;
    let e12 = r1*y - r2*x;

    Line(Bivector{ e41, e42, e43, e23, e31, e12 })
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
      let line = Line(grade_2(MULTIVECTOR_B));

      assert_eq!(
        orthogonal_antiprojection(plane, line),
        Plane(wedge(line.0, weight_contraction(plane.0, line.0)))
      );
    }
    {
      let plane = Plane(grade_3(MULTIVECTOR_A));
      let point = Point(grade_1(MULTIVECTOR_B));

      assert_eq!(
        orthogonal_antiprojection(plane, point),
        Plane(wedge(point.0, weight_contraction(plane.0, point.0)))
      );
    }
    {
      let line = Line(grade_2(MULTIVECTOR_A));
      let point = Point(grade_1(MULTIVECTOR_B));

      assert_eq!(
        orthogonal_antiprojection(line, point),
        Line(wedge(point.0, weight_contraction(line.0, point.0)))
      );
    }
  }
}
