use {
  crate::{
    algebra::{
      operators::{Unitize, WeightNorm, WeightNormSquared},
      values::{unit::impl_from_unit, Antiscalar, Unit, Vector},
    },
    F,
  },
  ::core::ops::Mul,
};

/// A 3D point represented by homogeneous coordinates
///
/// Homogeneous coordinates are a set of 4 coordinates in projective 3-space.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point(pub Vector);

impl Point {
  /// A point at the origin
  pub const ORIGIN: Self = Point(Vector::E4);

  #[inline]
  pub const fn new(x: F, y: F, z: F, w: F) -> Self {
    Point(Vector {
      e1: x,
      e2: y,
      e3: z,
      e4: w,
    })
  }

  #[inline]
  pub const fn new_unit(x: F, y: F, z: F) -> Unit<Point> {
    Unit::new_assume_unit(Point(Vector {
      e1: x,
      e2: y,
      e3: z,
      e4: 1.,
    }))
  }
}

impl Unit<Point> {
  /// A unit point at the origin
  pub const ORIGIN: Self = Unit::new_assume_unit(Point::ORIGIN);

  #[inline]
  pub const fn new(x: F, y: F, z: F) -> Self {
    Point::new_unit(x, y, z)
  }
}

impl From<(F, F, F, F)> for Point {
  #[inline]
  fn from((x, y, z, w): (F, F, F, F)) -> Self {
    Point::new(x, y, z, w)
  }
}

impl From<[F; 4]> for Point {
  #[inline]
  fn from([x, y, z, w]: [F; 4]) -> Self {
    Point::new(x, y, z, w)
  }
}

impl From<(F, F, F)> for Unit<Point> {
  #[inline]
  fn from((x, y, z): (F, F, F)) -> Self {
    Point::new_unit(x, y, z)
  }
}

impl From<[F; 3]> for Unit<Point> {
  #[inline]
  fn from([x, y, z]: [F; 3]) -> Self {
    Point::new_unit(x, y, z)
  }
}

impl Unitize for Point {}

impl_from_unit!(impl From<Unit<Point>> for Point);

impl WeightNormSquared for Point {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    self.0.weight_norm_squared()
  }
}
impl WeightNorm for Point {
  #[inline]
  fn weight_norm(self) -> Antiscalar {
    self.0.weight_norm()
  }
}

impl<Rhs> Mul<Rhs> for Point
where
  Vector: Mul<Rhs, Output = Vector>,
{
  type Output = Self;
  fn mul(self, rhs: Rhs) -> Self::Output {
    Self(self.0 * rhs)
  }
}
