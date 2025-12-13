use crate::{
  algebra::values::{Bivector, Trivector, Vector},
  geometry::objects::{Line, Plane, Point},
};

/// u ∧ (𝐞₃₂₁ ∨ u<sup>★</sup>) = u ∧ (𝐞̄₄ ∨ u<sup>★</sup>)
#[inline]
pub fn antisupport<M>(u: M) -> Plane
where
  M: Antisupport,
{
  u.antisupport()
}

/// u ∧ (𝐞₃₂₁ ∨ u<sup>★</sup>) = u ∧ (𝐞̄₄ ∨ u<sup>★</sup>)
pub trait Antisupport {
  /// u ∧ (𝐞₃₂₁ ∨ u<sup>★</sup>) = u ∧ (𝐞̄₄ ∨ u<sup>★</sup>)
  fn antisupport(self) -> Plane;
}

impl Antisupport for Point {
  #[rustfmt::skip]
  #[inline]
  fn antisupport(self) -> Plane {
    let Point(Vector { e1: m1, e2: m2, e3: m3, e4: m4 }) = self;

    let e423 = -m4*m1;
    let e431 = -m4*m2;
    let e412 = -m4*m3;
    let e321 = m1*m1 + m3*m3 + m2*m2;

    Plane(Trivector { e423, e431, e412, e321 })
  }
}

impl Antisupport for Line {
  #[rustfmt::skip]
  #[inline]
  fn antisupport(self) -> Plane {
    let Line(Bivector {
      e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
    }) = self;

    let e423 = m43*m31 - m42*m12;
    let e431 = m41*m12 - m43*m23;
    let e412 = m42*m23 - m41*m31;
    let e321 = m23*m23 + m12*m12 + m31*m31;

    Plane(Trivector { e423, e431, e412, e321 })
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    algebra::{operators::*, values::*},
    geometry::{objects::*, operators::*},
    test_values::*,
  };

  #[test]
  fn definition() {
    {
      let point = Point(grade_1(MULTIVECTOR_A));
      assert_eq!(
        antisupport(point),
        Plane(wedge(
          point.0,
          bulk_contraction(right_complement(Vector::E4), point.0)
        ))
      );
      assert_eq!(
        antisupport(point),
        central_antiprojection(Plane::HORIZON, point)
      );
    }

    {
      let line = Line(grade_2(MULTIVECTOR_A));
      assert_eq!(
        antisupport(line),
        Plane(wedge(
          line.0,
          bulk_contraction(right_complement(Vector::E4), line.0)
        ))
      );
      assert_eq!(
        antisupport(line),
        central_antiprojection(Plane::HORIZON, line)
      );
    }
  }
}
