use crate::algebra::values::{
  Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade, Scalar,
  Trivector, Vector,
};

/// u • u
#[inline]
pub fn bulk_norm_squared<M>(u: M) -> Scalar
where
  M: BulkNormSquared,
{
  u.bulk_norm_squared()
}

/// u • u
pub trait BulkNormSquared {
  /// u • u
  #[doc(alias = "norm")]
  fn bulk_norm_squared(self) -> Scalar;
}

/// ||u||<sub>●</sub>
///
/// Note: Requires either the `std` or `libm` feature to be enabled.
#[inline]
pub fn bulk_norm<M>(u: M) -> Scalar
where
  M: BulkNorm,
{
  u.bulk_norm()
}

/// ||u||<sub>●</sub>
///
/// Note: Requires either the `std` or `libm` feature to be enabled.
pub trait BulkNorm: BulkNormSquared + Sized {
  /// ||u||<sub>●</sub>
  ///
  /// Note: Requires either the `std` or `libm` feature to be enabled.
  #[cfg(not(feature = "_math"))]
  #[doc(alias = "norm")]
  fn bulk_norm(self) -> Scalar;

  /// ||u||<sub>●</sub>
  ///
  /// Note: Requires either the `std` or `libm` feature to be enabled.
  #[cfg(feature = "_math")]
  #[doc(alias = "norm")]
  #[inline]
  fn bulk_norm(self) -> Scalar {
    use crate::optional::math::Math;
    let norm = Math::sqrt(self.bulk_norm_squared().s);
    Scalar { s: norm }
  }
}

#[cfg(feature = "_math")]
mod norm_impls {
  use super::*;
  impl BulkNorm for Multivector {}
  impl BulkNorm for Vector {}
  impl BulkNorm for Bivector {}
  impl BulkNorm for OddGrade {}
  impl BulkNorm for EvenGrade {}
}

impl BulkNorm for Scalar {
  #[inline]
  fn bulk_norm(self) -> Scalar {
    Scalar { s: self.s.abs() }
  }
}

impl BulkNorm for Trivector {
  #[inline]
  fn bulk_norm(self) -> Scalar {
    Scalar { s: self.e321.abs() }
  }
}

impl BulkNorm for Antiscalar {
  #[inline(always)]
  fn bulk_norm(self) -> Scalar {
    Scalar::ZERO
  }
}

impl BulkNorm for DualNumber {
  #[inline]
  fn bulk_norm(self) -> Scalar {
    Scalar { s: self.s.abs() }
  }
}

impl BulkNormSquared for Multivector {
  #[inline]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar {
      s: self.s * self.s
        + self.e1 * self.e1
        + self.e2 * self.e2
        + self.e3 * self.e3
        + self.e23 * self.e23
        + self.e31 * self.e31
        + self.e12 * self.e12
        + self.e321 * self.e321,
    }
  }
}

impl BulkNormSquared for Scalar {
  #[inline]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar { s: self.s * self.s }
  }
}

impl BulkNormSquared for Vector {
  #[inline]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar {
      s: self.e1 * self.e1 + self.e2 * self.e2 + self.e3 * self.e3,
    }
  }
}

impl BulkNormSquared for Bivector {
  #[inline]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar {
      s: self.e23 * self.e23 + self.e31 * self.e31 + self.e12 * self.e12,
    }
  }
}

impl BulkNormSquared for Trivector {
  #[inline]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar {
      s: self.e321 * self.e321,
    }
  }
}

impl BulkNormSquared for Antiscalar {
  #[inline(always)]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar::ZERO
  }
}

impl BulkNormSquared for DualNumber {
  #[inline]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar { s: self.s * self.s }
  }
}

impl BulkNormSquared for OddGrade {
  #[inline]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar {
      s: self.e1 * self.e1
        + self.e2 * self.e2
        + self.e3 * self.e3
        + self.e321 * self.e321,
    }
  }
}

impl BulkNormSquared for EvenGrade {
  #[inline]
  fn bulk_norm_squared(self) -> Scalar {
    Scalar {
      s: self.s * self.s
        + self.e23 * self.e23
        + self.e31 * self.e31
        + self.e12 * self.e12,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{algebra::operators::*, test_values::*};
  #[cfg(feature = "_math")]
  use crate::{algebra::values::*, helpers::def_for_each};

  #[test]
  fn definition() {
    assert_eq!(
      bulk_norm_squared(MULTIVECTOR_A),
      dot(MULTIVECTOR_A, MULTIVECTOR_A)
    );

    #[cfg(feature = "_math")]
    assert_eq!(
      bulk_norm(MULTIVECTOR_A).s,
      dot(MULTIVECTOR_A, MULTIVECTOR_A).s.sqrt()
    );
  }

  #[cfg(feature = "_math")]
  def_for_each! {
    for variant in [
      multivector_a: MULTIVECTOR_A,
      multivector_b: MULTIVECTOR_B,
      multivector_c: MULTIVECTOR_C,
      scalar_a: grade_0(MULTIVECTOR_A),
      scalar_b: grade_0(MULTIVECTOR_B),
      scalar_c: grade_0(MULTIVECTOR_C),
      vector_a: grade_1(MULTIVECTOR_A),
      vector_b: grade_1(MULTIVECTOR_B),
      vector_c: grade_1(MULTIVECTOR_C),
      bivector_a: grade_2(MULTIVECTOR_A),
      bivector_b: grade_2(MULTIVECTOR_B),
      bivector_c: grade_2(MULTIVECTOR_C),
      trivector_a: grade_3(MULTIVECTOR_A),
      trivector_b: grade_3(MULTIVECTOR_B),
      trivector_c: grade_3(MULTIVECTOR_C),
      antiscalar_a: grade_4(MULTIVECTOR_A),
      antiscalar_b: grade_4(MULTIVECTOR_B),
      antiscalar_c: grade_4(MULTIVECTOR_C),
      dual_number_a: grade_0_4(MULTIVECTOR_A),
      dual_number_b: grade_0_4(MULTIVECTOR_B),
      dual_number_c: grade_0_4(MULTIVECTOR_C),
      odd_grade_a: grade_1_3(MULTIVECTOR_A),
      odd_grade_b: grade_1_3(MULTIVECTOR_B),
      odd_grade_c: grade_1_3(MULTIVECTOR_C),
      even_grade_a: grade_0_2_4(MULTIVECTOR_A),
      even_grade_b: grade_0_2_4(MULTIVECTOR_B),
      even_grade_c: grade_0_2_4(MULTIVECTOR_C),
    ] {
      #[test]
      fn sparse_*() {
        assert_eq!(
          Scalar::from(bulk_norm(variant)),
          bulk_norm(Multivector::from(variant))
        );
      }
    }
  }
}
