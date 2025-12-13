#[cfg(feature = "_math")]
use crate::algebra::operators::WeightNorm;
use {
  crate::algebra::{
    operators::{DotProduct, Unitize, WeightNormSquared},
    values::{unit::impl_from_unit, Antiscalar, DualNumber, Scalar, Unit},
  },
  ::core::ops::Mul,
};

/// A homogeneous magnitude
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Magnitude(pub DualNumber);

impl Unitize for Magnitude {}

impl_from_unit!(impl From<Unit<Magnitude>> for Magnitude);

impl WeightNormSquared for Magnitude {
  fn weight_norm_squared(self) -> Antiscalar {
    self.0.weight_norm_squared()
  }
}
#[cfg(feature = "_math")]
impl WeightNorm for Magnitude {}

impl<Rhs> DotProduct<Rhs> for Magnitude
where
  DualNumber: DotProduct<Rhs>,
{
  fn dot(self, b: Rhs) -> Scalar {
    self.0.dot(b)
  }
}

impl<Rhs> Mul<Rhs> for Magnitude
where
  DualNumber: Mul<Rhs, Output = DualNumber>,
{
  type Output = Self;
  fn mul(self, rhs: Rhs) -> Self::Output {
    Self(self.0 * rhs)
  }
}
