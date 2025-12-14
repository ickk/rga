use {
  crate::algebra::{
    operators::{DotProduct, Unitize, WeightNorm, WeightNormSquared},
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
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    self.0.weight_norm_squared()
  }
}
impl WeightNorm for Magnitude {
  #[inline]
  fn weight_norm(self) -> Antiscalar {
    self.0.weight_norm()
  }
}

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
