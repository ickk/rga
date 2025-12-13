use crate::algebra::values::{
  Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade, Scalar,
  Trivector, Vector,
};

/// ṵ
#[inline]
pub fn antireverse<M>(u: M) -> M
where
  M: Antireverse,
{
  u.antireverse()
}

/// ṵ
pub trait Antireverse {
  /// ṵ
  fn antireverse(self) -> Self;
}

impl Antireverse for Multivector {
  #[inline]
  fn antireverse(self) -> Self {
    Multivector {
      e1: -self.e1,
      e2: -self.e2,
      e3: -self.e3,
      e4: -self.e4,
      e41: -self.e41,
      e42: -self.e42,
      e43: -self.e43,
      e23: -self.e23,
      e31: -self.e31,
      e12: -self.e12,
      ..self
    }
  }
}

impl Antireverse for Scalar {
  #[inline(always)]
  fn antireverse(self) -> Self {
    self
  }
}

impl Antireverse for Vector {
  #[inline]
  fn antireverse(self) -> Self {
    -self
  }
}

impl Antireverse for Bivector {
  #[inline]
  fn antireverse(self) -> Self {
    -self
  }
}

impl Antireverse for Trivector {
  #[inline(always)]
  fn antireverse(self) -> Self {
    self
  }
}

impl Antireverse for Antiscalar {
  #[inline(always)]
  fn antireverse(self) -> Self {
    self
  }
}

impl Antireverse for DualNumber {
  #[inline(always)]
  fn antireverse(self) -> Self {
    self
  }
}

impl Antireverse for OddGrade {
  #[inline]
  fn antireverse(self) -> Self {
    OddGrade {
      e1: -self.e1,
      e2: -self.e2,
      e3: -self.e3,
      e4: -self.e4,
      ..self
    }
  }
}

impl Antireverse for EvenGrade {
  #[inline(always)]
  fn antireverse(self) -> Self {
    EvenGrade {
      e41: -self.e41,
      e42: -self.e42,
      e43: -self.e43,
      e23: -self.e23,
      e31: -self.e31,
      e12: -self.e12,
      ..self
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    algebra::{operators::*, values::*},
    helpers::def_for_each,
    test_values::*,
  };

  #[test]
  fn de_morgans_laws() {
    assert_eq!(
      antireverse(MULTIVECTOR_A),
      left_complement(reverse(right_complement(MULTIVECTOR_A)))
    );

    assert_eq!(
      antireverse(MULTIVECTOR_A),
      right_complement(reverse(left_complement(MULTIVECTOR_A)))
    )
  }

  #[test]
  fn relationship_to_antidot_product() {
    assert_eq!(
      grade_4(geometric_antiproduct(
        MULTIVECTOR_A,
        antireverse(MULTIVECTOR_B)
      )),
      antidot(MULTIVECTOR_A, MULTIVECTOR_B)
    );
  }

  #[test]
  fn relationship_to_weight_dual() {
    assert_eq!(
      weight_dual(MULTIVECTOR_A),
      geometric_antiproduct(
        antireverse(MULTIVECTOR_A),
        Multivector { s: 1., ..zero() }
      )
    );

    assert_eq!(
      left_weight_dual(MULTIVECTOR_A),
      geometric_antiproduct(
        Multivector { s: 1., ..zero() },
        antireverse(MULTIVECTOR_A)
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
        assert_eq!(
          Multivector::from(antireverse(variant)),
          antireverse(Multivector::from(variant))
        );
      }
    }
  }
}
