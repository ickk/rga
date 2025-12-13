use crate::{
  algebra::values::{Bivector, Trivector, Vector},
  geometry::objects::{Line, Plane, Point},
};

/// *u* ∨ (𝐞₄ ∧ *u*<sup>☆</sup>)
///
/// The point contained in *u* that is closest to the origin.
#[inline]
pub fn support<M>(u: M) -> Point
where
  M: Support,
{
  u.support()
}

/// *u* ∨ (𝐞₄ ∧ *u*<sup>☆</sup>)
///
/// The point contained in *u* that is closest to the origin.
pub trait Support {
  /// *u* ∨ (𝐞₄ ∧ *u*<sup>☆</sup>)
  ///
  /// The point contained in *u* that is closest to the origin.
  fn support(self) -> Point;
}

impl Support for Line {
  #[inline]
  fn support(self) -> Point {
    let Line(Bivector {
      e41: m41,
      e42: m42,
      e43: m43,
      e23: m23,
      e31: m31,
      e12: m12,
    }) = self;

    let e1 = m12 * m42 - m31 * m43;
    let e2 = m23 * m43 - m12 * m41;
    let e3 = m31 * m41 - m23 * m42;
    let e4 = m41 * m41 + m42 * m42 + m43 * m43;

    Point(Vector { e1, e2, e3, e4 })
  }
}

impl Support for Plane {
  #[inline]
  fn support(self) -> Point {
    let Plane(Trivector {
      e423,
      e431,
      e412,
      e321,
    }) = self;

    let e1 = -e321 * e423;
    let e2 = -e321 * e431;
    let e3 = -e321 * e412;
    let e4 = e423 * e423 + e431 * e431 + e412 * e412;

    Point(Vector { e1, e2, e3, e4 })
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
      let line: Line = Line(grade_2(MULTIVECTOR_A));
      assert_eq!(
        support(line),
        Point(antiwedge(line.0, weight_expansion(Vector::E4, line.0)))
      );
      assert_eq!(support(line), orthogonal_projection(Point::ORIGIN, line));
    }

    {
      let plane: Plane = Plane(grade_3(MULTIVECTOR_A));
      assert_eq!(
        support(plane),
        Point(antiwedge(plane.0, weight_expansion(Vector::E4, plane.0)))
      );
      assert_eq!(support(plane), orthogonal_projection(Point::ORIGIN, plane));
    }
  }
}
