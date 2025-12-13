use crate::algebra::values::{
  Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade, Scalar,
  Trivector, Unit, Vector,
};

/// u ∘ u
#[inline]
pub fn weight_norm_squared<M>(u: M) -> Antiscalar
where
  M: WeightNormSquared,
{
  u.weight_norm_squared()
}

/// ||u||<sub>○</sub>
///
/// Note: Requires either the `std` or `libm` feature to be enabled.
#[inline]
pub fn weight_norm<M>(u: M) -> Antiscalar
where
  M: WeightNorm,
{
  u.weight_norm()
}

/// u ∘ u
pub trait WeightNormSquared {
  /// u ∘ u
  #[doc(alias = "norm")]
  fn weight_norm_squared(self) -> Antiscalar;
}

/// ||u||<sub>○</sub>
///
/// Note: Requires either the `std` or `libm` feature to be enabled for
/// non-[`Unit`] values.
pub trait WeightNorm: WeightNormSquared + Sized {
  /// ||u||<sub>○</sub>
  ///
  /// Note: Requires either the `std` or `libm` feature to be enabled for
  /// non-[`Unit`] values.
  #[cfg(not(feature = "_math"))]
  #[doc(alias = "norm")]
  fn weight_norm(self) -> Antiscalar;

  /// ||u||<sub>○</sub>
  ///
  /// Note: Requires either the `std` or `libm` feature to be enabled for
  /// non-[`Unit`] values.
  #[cfg(feature = "_math")]
  #[doc(alias = "norm")]
  #[inline]
  fn weight_norm(self) -> Antiscalar {
    use crate::optional::math::Math;
    Antiscalar {
      e1234: Math::sqrt(self.weight_norm_squared().e1234),
    }
  }
}

#[cfg(feature = "_math")]
mod norm_impls {
  use super::*;
  impl WeightNorm for Multivector {}
  impl WeightNorm for Bivector {}
  impl WeightNorm for Trivector {}
  impl WeightNorm for OddGrade {}
  impl WeightNorm for EvenGrade {}
}

impl WeightNorm for Scalar {
  #[inline(always)]
  fn weight_norm(self) -> Antiscalar {
    Antiscalar::ZERO
  }
}

impl WeightNorm for Vector {
  #[inline]
  fn weight_norm(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e4.abs(),
    }
  }
}

impl WeightNorm for Antiscalar {
  #[inline]
  fn weight_norm(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e1234.abs(),
    }
  }
}

impl WeightNorm for DualNumber {
  #[inline]
  fn weight_norm(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e1234.abs(),
    }
  }
}

impl<T> WeightNorm for Unit<T>
where
  T: Copy + WeightNormSquared,
{
  /// Unconditionally return unit antiscalar 𝟙 for any [`Unit`] value
  ///
  /// `Unit` values are assumed to have a weight norm of 𝟙, which avoids a
  /// sqrt.
  #[inline(always)]
  fn weight_norm(self) -> Antiscalar {
    Antiscalar::E1234
  }
}

impl<T> WeightNormSquared for Unit<T>
where
  T: Copy + WeightNormSquared,
{
  /// Unconditionally return unit antiscalar 𝟙 for any [`Unit`] value
  #[inline(always)]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar::E1234
  }
}

impl WeightNormSquared for Multivector {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e4 * self.e4
        + self.e41 * self.e41
        + self.e42 * self.e42
        + self.e43 * self.e43
        + self.e423 * self.e423
        + self.e431 * self.e431
        + self.e412 * self.e412
        + self.e1234 * self.e1234,
    }
  }
}

impl WeightNormSquared for Scalar {
  #[inline(always)]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar::ZERO
  }
}

impl WeightNormSquared for Vector {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e4 * self.e4,
    }
  }
}

impl WeightNormSquared for Bivector {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e41 * self.e41 + self.e42 * self.e42 + self.e43 * self.e43,
    }
  }
}

impl WeightNormSquared for Trivector {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e423 * self.e423
        + self.e431 * self.e431
        + self.e412 * self.e412,
    }
  }
}

impl WeightNormSquared for Antiscalar {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e1234 * self.e1234,
    }
  }
}

impl WeightNormSquared for DualNumber {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e1234 * self.e1234,
    }
  }
}

impl WeightNormSquared for OddGrade {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e4 * self.e4
        + self.e423 * self.e423
        + self.e431 * self.e431
        + self.e412 * self.e412,
    }
  }
}

impl WeightNormSquared for EvenGrade {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    Antiscalar {
      e1234: self.e41 * self.e41
        + self.e42 * self.e42
        + self.e43 * self.e43
        + self.e1234 * self.e1234,
    }
  }
}

#[cfg(test)]
mod tests {
  #[cfg(feature = "_math")]
  use crate::{
    algebra::{operators::*, values::*},
    helpers::def_for_each,
    test_values::*,
  };

  #[cfg(feature = "_math")]
  #[test]
  fn definition() {
    assert_eq!(
      weight_norm(MULTIVECTOR_A).e1234,
      antidot(MULTIVECTOR_A, MULTIVECTOR_A).e1234.sqrt()
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
          Antiscalar::from(weight_norm(variant)),
          weight_norm(Multivector::from(variant))
        );
      }
    }
  }
}
