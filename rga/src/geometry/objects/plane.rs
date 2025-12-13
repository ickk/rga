use {
  crate::algebra::{
    operators::{Unitize, WeightNormSquared},
    values::{unit::impl_from_unit, Antiscalar, Trivector, Unit},
  },
  ::core::ops::Mul,
};

/// A 3D plane represented by a normal and position
///
/// The normal & position are the 4 coordinates of a trivector in projective
/// 3-space. The position is proportional to the magnitude of the normal
/// trivector. A unitized plane has a normal with magnitude 1, and the position
/// is the signed distance to the origin.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Plane(pub Trivector);

impl Plane {
  /// A plane at the horizon
  pub const HORIZON: Self = Plane(Trivector::E321);
}

impl Unit<Plane> {
  /// A unit plane at the horizon
  pub const HORIZON: Self = Unit::new_assume_unit(Plane::HORIZON);
}

impl Unitize for Plane {}

impl_from_unit!(impl From<Unit<Plane>> for Plane);

impl WeightNormSquared for Plane {
  fn weight_norm_squared(self) -> Antiscalar {
    self.0.weight_norm_squared()
  }
}

impl<Rhs> Mul<Rhs> for Plane
where
  Trivector: Mul<Rhs, Output = Trivector>,
{
  type Output = Self;
  fn mul(self, rhs: Rhs) -> Self::Output {
    Self(self.0 * rhs)
  }
}
