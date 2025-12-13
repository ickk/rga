use crate::algebra::values::{
  zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
  Scalar, Trivector, Vector,
};
#[cfg(feature = "approx")]
use crate::F;

pub trait IsGeometric {
  type Residual;

  /// (*u*⟑*ũ* - *u*•*u*)/2
  ///
  /// The geometric-residual ought to be zero for "geometric objects", i.e.
  /// values constructed strictly from the wedge-products of vectors. This
  /// includes all compositions of reflections - thus all valid motors &
  /// flectors.
  ///
  /// When *u*⟑*ũ* - *u*•*u* = 0, then *u*⁻¹ = *ũ*/(*u*•*u*). That is when the
  /// geometric-residual is zero, the inverse of u is equivalent to the reverse
  /// of *u* up to a scaling factor. Further, if *u* is also unitized then
  /// *u*⁻¹ = *ũ*.
  fn geometric_residual(&self) -> Self::Residual;

  #[cfg(feature = "approx")]
  #[inline]
  fn is_geometric_ulps(&self, epsilon: F, max_ulps: u32) -> bool
  where
    Self::Residual: ::approx::UlpsEq<Scalar, Epsilon = F>,
  {
    ::approx::UlpsEq::ulps_eq(
      &self.geometric_residual(),
      &Scalar::ZERO,
      epsilon,
      max_ulps,
    )
  }

  #[cfg(feature = "approx")]
  #[inline]
  fn is_geometric_relative(&self, epsilon: F, max_relative: F) -> bool
  where
    Self::Residual: ::approx::RelativeEq<Scalar, Epsilon = F>,
  {
    ::approx::RelativeEq::relative_eq(
      &self.geometric_residual(),
      &Scalar::ZERO,
      epsilon,
      max_relative,
    )
  }

  #[cfg(feature = "approx")]
  #[inline]
  fn is_geometric_abs(&self, epsilon: F) -> bool
  where
    Self::Residual: ::approx::AbsDiffEq<Scalar, Epsilon = F>,
  {
    ::approx::AbsDiffEq::abs_diff_eq(
      &self.geometric_residual(),
      &Scalar::ZERO,
      epsilon,
    )
  }
}

impl IsGeometric for Multivector {
  type Residual = Multivector;

  #[rustfmt::skip]
  #[inline]
  fn geometric_residual(&self) -> Self::Residual {
    let &Multivector {
      s: ms,
      e1: m1, e2: m2, e3: m3, e4: m4,
      e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
      e423: m423, e431: m431, e412: m412, e321: m321,
      e1234: m1234,
    } = self;

    let e1 = - m23*m321 - m31*m3 + m12*m2 + ms*m1;
    let e2 =   m23*m3 - m31*m321 - m12*m1 + ms*m2;
    let e3 = - m23*m2 + m31*m1 - m12*m321 + ms*m3;
    let e4 =   m23*m423 + m31*m431 + m12*m412 + m1234*m321
      + m1*m41 + m2*m42 + m3*m43 + m4*ms;
    let e1234 = m23*m41 + m31*m42 + m12*m43 + ms*m1234
      - (m1*m423 + m2*m431 + m3*m412 + m4*m321);

    Multivector {
      e1, e2, e3, e4,
      e1234,
      ..zero()
    }
  }
}

impl IsGeometric for Scalar {
  type Residual = Scalar;

  #[inline(always)]
  fn geometric_residual(&self) -> Self::Residual {
    Scalar::ZERO
  }
}

impl IsGeometric for Vector {
  type Residual = Scalar;

  #[inline(always)]
  fn geometric_residual(&self) -> Self::Residual {
    Scalar::ZERO
  }
}

impl IsGeometric for Bivector {
  type Residual = Antiscalar;

  #[inline]
  fn geometric_residual(&self) -> Self::Residual {
    Antiscalar {
      e1234: self.e23 * self.e41 + self.e31 * self.e42 + self.e12 * self.e43,
    }
  }
}

impl IsGeometric for Trivector {
  type Residual = Scalar;

  #[inline(always)]
  fn geometric_residual(&self) -> Self::Residual {
    Scalar::ZERO
  }
}

impl IsGeometric for Antiscalar {
  type Residual = Scalar;

  #[inline(always)]
  fn geometric_residual(&self) -> Self::Residual {
    Scalar::ZERO
  }
}

impl IsGeometric for DualNumber {
  type Residual = Antiscalar;

  #[inline]
  fn geometric_residual(&self) -> Self::Residual {
    Antiscalar {
      e1234: self.s * self.e1234,
    }
  }
}

impl IsGeometric for OddGrade {
  type Residual = Antiscalar;

  #[inline]
  fn geometric_residual(&self) -> Self::Residual {
    Antiscalar {
      e1234: -(self.e1 * self.e423
        + self.e2 * self.e431
        + self.e3 * self.e412
        + self.e4 * self.e321),
    }
  }
}

impl IsGeometric for EvenGrade {
  type Residual = Antiscalar;

  #[inline]
  fn geometric_residual(&self) -> Self::Residual {
    Antiscalar {
      e1234: self.e23 * self.e41
        + self.e31 * self.e42
        + self.e12 * self.e43
        + self.s * self.e1234,
    }
  }
}

#[cfg(test)]
mod tests {
  use {
    crate::{
      algebra::{operators::*, values::*},
      helpers::def_for_each,
      test_values::*,
    },
    ::approx::assert_ulps_eq,
  };

  #[test]
  fn definition() {
    let multivector = MULTIVECTOR_A;
    assert_ulps_eq!(
      dbg!(multivector.geometric_residual()),
      dbg!(
        (geometric_product(multivector, reverse(multivector))
          - bulk_norm_squared(multivector))
          * Scalar(0.5)
      )
    );
  }

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
        assert_ulps_eq!(
          Multivector::from(variant.geometric_residual()),
          Multivector::from(variant).geometric_residual(),
        )
      }
    }
  }
}
