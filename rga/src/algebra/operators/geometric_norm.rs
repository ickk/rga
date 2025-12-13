#[cfg(feature = "_math")]
use crate::algebra::operators::{BulkNorm, WeightNorm};
use crate::algebra::values::DualNumber;

/// ||u||
///
/// ||u|| = ||u||<sub>●</sub> + ||u||<sub>○</sub>
///
/// Note: Requires either the `std` or `libm` feature to be enabled.
#[inline]
pub fn geometric_norm<M: GeometricNorm>(u: M) -> DualNumber {
  u.geometric_norm()
}

/// ||u||
///
/// ||u|| = ||u||<sub>●</sub> + ||u||<sub>○</sub>
///
/// Note: Requires either the `std` or `libm` feature to be enabled.
pub trait GeometricNorm {
  /// ||u||
  ///
  /// ||u|| = ||u||<sub>●</sub> + ||u||<sub>○</sub>
  ///
  /// Note: Requires either the `std` or `libm` feature to be enabled.
  #[doc(alias = "norm")]
  fn geometric_norm(self) -> DualNumber;
}

#[cfg(feature = "_math")]
impl<T> GeometricNorm for T
where
  T: WeightNorm + BulkNorm + Copy,
{
  #[inline]
  fn geometric_norm(self) -> DualNumber {
    self.bulk_norm() + self.weight_norm()
  }
}
