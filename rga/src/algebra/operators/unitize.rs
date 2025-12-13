#[cfg(feature = "_math")]
use crate::algebra::values::{
  Bivector, EvenGrade, Multivector, OddGrade, Trivector,
};
use {
  crate::algebra::{
    operators::{WeightNorm, WeightNormSquared},
    values::{Antiscalar, DualNumber, Scalar, Unit, Vector},
  },
  ::core::ops::Mul,
};

/// û
///
/// Weight Normalisation.
///
/// Scale the value so that the weight is normalised to a unit magnitude of 𝟙.
#[inline]
pub fn unitize<M>(u: M) -> Unit<M>
where
  M: Unitize + WeightNorm,
{
  u.unitize()
}

/// û
///
/// Weight Normalisation.
///
/// Scale the value so that the weight is normalised to have a unit magnitude
/// of 𝟙.
pub trait Unitize:
  WeightNormSquared + Mul<Scalar, Output = Self> + Copy
where
  Self: Copy,
{
  /// û
  ///
  /// Weight Normalisation.
  ///
  /// Scale the value so that the weight is normalised to have a unit magnitude
  /// of 𝟙.
  #[inline]
  fn unitize(self) -> Unit<Self>
  where
    Self: WeightNorm,
  {
    Unit(
      self
        * Scalar {
          s: 1. / self.weight_norm().e1234,
        },
    )
  }

  /// Reunitize the value
  ///
  /// This may be needed if the inner value becomes denormalised due to
  /// numerical imprecision in repeated computations.
  ///
  /// See also [`Self::reunitize_fast`].
  #[inline]
  fn reunitize(unit: &mut Unit<Self>)
  where
    Self: WeightNorm,
  {
    *unit = Self::unitize(unit.0);
  }

  /// Perform a fast approximate reunitization using a single step of Newton's
  /// method
  ///
  /// This can be a relatively cheap way to keep values unitized when they
  /// are subject to denormalisation due to numerical imprecision in repeated
  /// computations.
  #[inline]
  fn reunitize_fast(unit: &mut Unit<Self>) {
    unit.0 = unit.0
      * Scalar {
        s: 0.5 * (3. - unit.0.weight_norm_squared().e1234),
      };
  }
}

#[cfg(feature = "_math")]
mod unitize_impls {
  use super::*;
  impl Unitize for Multivector {}
  impl Unitize for Bivector {}
  impl Unitize for Trivector {}
  impl Unitize for OddGrade {}
  impl Unitize for EvenGrade {}
}

impl Unitize for Vector {}
impl Unitize for Antiscalar {}
impl Unitize for DualNumber {}

#[cfg(test)]
mod tests {
  #[cfg(feature = "_math")]
  #[test]
  fn definition() {
    use crate::{
      algebra::{operators::*, values::*},
      test_values::*,
    };

    let unit: Unit<Multivector> = unitize(MULTIVECTOR_A);
    assert_eq!(weight_norm(unit), Antiscalar { e1234: 1. });
  }
}
