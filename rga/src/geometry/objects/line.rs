use {
  crate::algebra::{
    operators::{Unitize, WeightNormSquared},
    values::{unit::impl_from_unit, Antiscalar, Bivector, Unit},
  },
  ::core::ops::Mul,
};

/// A 3D line represented by plücker coordinates
///
/// Plücker coordinates are a set of 6 homogeneous coordinates in projective
/// 3-space.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Line(pub Bivector);

impl Line {
  /// A line coindicent with the x-axis
  pub const X: Self = Line(Bivector::E41);
  /// A line coindicent with the y-axis
  pub const Y: Self = Line(Bivector::E42);
  /// A line coindicent with the z-axis
  pub const Z: Self = Line(Bivector::E43);
}

impl Unit<Line> {
  /// A unit line coincident with the x-axis
  pub const X: Self = Unit::new_assume_unit(Line::X);
  /// A unit line coincident with the y-axis
  pub const Y: Self = Unit::new_assume_unit(Line::Y);
  /// A unit line coincident with the z-axis
  pub const Z: Self = Unit::new_assume_unit(Line::Z);
}

impl Unitize for Line {}

impl_from_unit!(impl From<Unit<Line>> for Line);

impl WeightNormSquared for Line {
  fn weight_norm_squared(self) -> Antiscalar {
    self.0.weight_norm_squared()
  }
}

impl<Rhs> Mul<Rhs> for Line
where
  Bivector: Mul<Rhs, Output = Bivector>,
{
  type Output = Self;
  fn mul(self, rhs: Rhs) -> Self::Output {
    Self(self.0 * rhs)
  }
}
