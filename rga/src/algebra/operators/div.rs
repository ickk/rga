use crate::{
  algebra::{
    operators::{GeometricProduct, Inverse},
    values::{
      Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
      Scalar, Trivector, Vector,
    },
  },
  helpers::return_scalar_nan_binary,
};

pub use ::core::ops::Div;

/// a / b
///
/// Division implemented in terms of the [`GeometricProduct`] of the
/// numerator and the [`Inverse`] of the divisor, i.e. a / b = a ⟑ b⁻¹.
///
/// If the inverse of the divisor does not exist, then the result will be
/// [`IsNan`](crate::IsNan).
///
/// Note: Division by zero is defined to be NaN even for [`struct@Scalar`] values,
/// which differs to IEEE754.
#[inline]
pub fn div<Lhs, Rhs>(a: Lhs, b: Rhs) -> <Lhs as Div<Rhs>>::Output
where
  Lhs: Div<Rhs>,
{
  a.div(b)
}

macro_rules! impl_div {
  (
    Div::div {
      $($lhs:ty, $rhs:ty => $output:ty: $implementation_fn:ident;)*
    }
  ) => {
    /// a / b
    ///
    /// Division implemented in terms of the [`GeometricProduct`] of the
    /// numerator and the [`Inverse`] of the divisor, i.e. a / b = a ⟑ b⁻¹.
    ///
    /// If the inverse of the divisor does not exist, then the result will be
    /// [`IsNan`](crate::IsNan).
    ///
    /// Note: Division by zero is defined to be NaN even for [`struct@Scalar`] values,
    /// which differs to IEEE754.
    $(impl ::core::ops::Div<$rhs> for $lhs {
      type Output = $output;

      /// a / b
      ///
      /// Division implemented in terms of the [`GeometricProduct`] of the
      /// numerator and the [`Inverse`] of the divisor, i.e. a / b = a ⟑ b⁻¹.
      ///
      /// If the inverse of the divisor does not exist, then the result will be
      /// [`IsNan`](crate::IsNan).
      ///
      /// Note: Division by zero is defined to be NaN even for [`struct@Scalar`] values,
      /// which differs to IEEE754.
      #[inline]
      fn div(self, other: $rhs) -> $output {
        $implementation_fn(self, other)
      }
    })*
  };
}

impl_div!(Div::div {
  Multivector, Multivector => Multivector: mul_inverse;
  Multivector, Scalar => Multivector: mul_inverse;
  Multivector, Vector => Multivector: mul_inverse;
  Multivector, Bivector => Multivector: mul_inverse;
  Multivector, Trivector => Multivector: mul_inverse;
  Multivector, Antiscalar => Multivector: mul_inverse;
  Multivector, DualNumber => Multivector: mul_inverse;
  Multivector, OddGrade => Multivector: mul_inverse;
  Multivector, EvenGrade => Multivector: mul_inverse;

  Scalar, Multivector => Multivector: mul_inverse;
  Scalar, Scalar => Scalar: mul_inverse;
  Scalar, Vector => Vector: mul_inverse;
  Scalar, Bivector => Bivector: mul_inverse;
  Scalar, Trivector => Trivector: mul_inverse;
  Scalar, Antiscalar => Antiscalar: mul_inverse;
  Scalar, DualNumber => DualNumber: mul_inverse;
  Scalar, OddGrade => OddGrade: mul_inverse;
  Scalar, EvenGrade => EvenGrade: mul_inverse;

  Vector, Multivector => Multivector: mul_inverse;
  Vector, Scalar => Vector: mul_inverse;
  Vector, Vector => EvenGrade: mul_inverse;
  Vector, Bivector => OddGrade: mul_inverse;
  Vector, Trivector => EvenGrade: mul_inverse;
  Vector, Antiscalar => Trivector: mul_inverse;
  Vector, DualNumber => OddGrade: mul_inverse;
  Vector, OddGrade => EvenGrade: mul_inverse;
  Vector, EvenGrade => OddGrade: mul_inverse;

  Bivector, Multivector => Multivector: mul_inverse;
  Bivector, Scalar => Bivector: mul_inverse;
  Bivector, Vector => OddGrade: mul_inverse;
  Bivector, Bivector => EvenGrade: mul_inverse;
  Bivector, Trivector => OddGrade: mul_inverse;
  Bivector, Antiscalar => Bivector: mul_inverse;
  Bivector, DualNumber => Bivector: mul_inverse;
  Bivector, OddGrade => OddGrade: mul_inverse;
  Bivector, EvenGrade => EvenGrade: mul_inverse;

  Trivector, Multivector => Multivector: mul_inverse;
  Trivector, Scalar => Trivector: mul_inverse;
  Trivector, Vector => EvenGrade: mul_inverse;
  Trivector, Bivector => OddGrade: mul_inverse;
  Trivector, Trivector => EvenGrade: mul_inverse;
  Trivector, Antiscalar => Vector: mul_inverse;
  Trivector, DualNumber => OddGrade: mul_inverse;
  Trivector, OddGrade => EvenGrade: mul_inverse;
  Trivector, EvenGrade => OddGrade: mul_inverse;

  Antiscalar, Multivector => Multivector: mul_inverse;
  Antiscalar, Scalar => Antiscalar: mul_inverse;
  Antiscalar, Vector => Trivector: mul_inverse;
  Antiscalar, Bivector => Bivector: mul_inverse;
  Antiscalar, Trivector => Vector: mul_inverse;
  Antiscalar, Antiscalar => Scalar: return_scalar_nan_binary;
  Antiscalar, DualNumber => Antiscalar: mul_inverse;
  Antiscalar, OddGrade => OddGrade: mul_inverse;
  Antiscalar, EvenGrade => EvenGrade: mul_inverse;

  DualNumber, Multivector => Multivector: mul_inverse;
  DualNumber, Scalar => DualNumber: mul_inverse;
  DualNumber, Vector => OddGrade: mul_inverse;
  DualNumber, Bivector => Bivector: mul_inverse;
  DualNumber, Trivector => OddGrade: mul_inverse;
  DualNumber, Antiscalar => Antiscalar: mul_inverse;
  DualNumber, DualNumber => DualNumber: mul_inverse;
  DualNumber, OddGrade => OddGrade: mul_inverse;
  DualNumber, EvenGrade => EvenGrade: mul_inverse;

  OddGrade, Multivector => Multivector: mul_inverse;
  OddGrade, Scalar => OddGrade: mul_inverse;
  OddGrade, Vector => EvenGrade: mul_inverse;
  OddGrade, Bivector => OddGrade: mul_inverse;
  OddGrade, Trivector => EvenGrade: mul_inverse;
  OddGrade, Antiscalar => OddGrade: mul_inverse;
  OddGrade, DualNumber => OddGrade: mul_inverse;
  OddGrade, OddGrade => EvenGrade: mul_inverse;
  OddGrade, EvenGrade => OddGrade: mul_inverse;

  EvenGrade, Multivector => Multivector: mul_inverse;
  EvenGrade, Scalar => EvenGrade: mul_inverse;
  EvenGrade, Vector => OddGrade: mul_inverse;
  EvenGrade, Bivector => EvenGrade: mul_inverse;
  EvenGrade, Trivector => OddGrade: mul_inverse;
  EvenGrade, Antiscalar => EvenGrade: mul_inverse;
  EvenGrade, DualNumber => EvenGrade: mul_inverse;
  EvenGrade, OddGrade => OddGrade: mul_inverse;
  EvenGrade, EvenGrade => EvenGrade: mul_inverse;
});

#[inline]
fn mul_inverse<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as GeometricProduct<Rhs>>::Output
where
  Lhs: GeometricProduct<Rhs>,
  Rhs: Inverse,
{
  a.geometric_product(b.inverse())
}

#[cfg(test)]
mod tests {
  use {
    crate::{
      algebra::{operators::*, values::*},
      test_values::*,
    },
    ::approx::assert_ulps_eq,
  };

  #[test]
  fn definition() {
    assert_ulps_eq!(
      dbg!(MULTIVECTOR_A / MULTIVECTOR_B),
      dbg!(MULTIVECTOR_A.geometric_product(MULTIVECTOR_B.inverse())),
      epsilon = 0.00_000_000_000_1
    );
  }

  #[test]
  fn no_inverse() {
    assert!(dbg!(MULTIVECTOR_A / Vector(0., 0., 0., 1.)).is_nan());
  }
}
