use {
  crate::algebra::{
    operators::{Unitize, WeightNormSquared},
    values::{unit::impl_from_unit, Antiscalar, Unit, Vector},
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
}

impl Unit<Point> {
  /// A unit point at the origin
  pub const ORIGIN: Self = Unit::new_assume_unit(Point::ORIGIN);
}

impl Unitize for Point {}

impl_from_unit!(impl From<Unit<Point>> for Point);

impl WeightNormSquared for Point {
  fn weight_norm_squared(self) -> Antiscalar {
    self.0.weight_norm_squared()
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
