use crate::algebra::values::{
  Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade, Scalar,
  Trivector, Vector,
};

/// ũ
#[inline]
pub fn reverse<M>(u: M) -> M
where
  M: Reverse,
{
  u.reverse()
}

/// ũ
pub trait Reverse {
  /// ũ
  fn reverse(self) -> Self;
}

impl Reverse for Multivector {
  #[inline]
  fn reverse(self) -> Self {
    Multivector {
      e41: -self.e41,
      e42: -self.e42,
      e43: -self.e43,
      e23: -self.e23,
      e31: -self.e31,
      e12: -self.e12,
      e423: -self.e423,
      e431: -self.e431,
      e412: -self.e412,
      e321: -self.e321,
      ..self
    }
  }
}

impl Reverse for Scalar {
  #[inline(always)]
  fn reverse(self) -> Self {
    self
  }
}

impl Reverse for Vector {
  #[inline(always)]
  fn reverse(self) -> Self {
    self
  }
}

impl Reverse for Bivector {
  #[inline]
  fn reverse(self) -> Self {
    -self
  }
}

impl Reverse for Trivector {
  #[inline]
  fn reverse(self) -> Self {
    -self
  }
}

impl Reverse for Antiscalar {
  #[inline(always)]
  fn reverse(self) -> Self {
    self
  }
}

impl Reverse for DualNumber {
  #[inline(always)]
  fn reverse(self) -> Self {
    self
  }
}

impl Reverse for OddGrade {
  #[inline]
  fn reverse(self) -> Self {
    OddGrade {
      e423: -self.e423,
      e431: -self.e431,
      e412: -self.e412,
      e321: -self.e321,
      ..self
    }
  }
}

impl Reverse for EvenGrade {
  #[inline]
  fn reverse(self) -> Self {
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
  fn linearity() {
    let s = Scalar { s: 7. };

    assert_eq!(reverse(s * MULTIVECTOR_A), s * reverse(MULTIVECTOR_A));

    assert_eq!(
      reverse(MULTIVECTOR_A + MULTIVECTOR_B),
      reverse(MULTIVECTOR_A) + reverse(MULTIVECTOR_B)
    );
  }

  #[test]
  fn distribution_over_geometric_product() {
    assert_eq!(
      reverse(geometric_product(MULTIVECTOR_A, MULTIVECTOR_B)),
      geometric_product(reverse(MULTIVECTOR_B), reverse(MULTIVECTOR_A))
    );
  }

  #[test]
  fn relationship_to_dot_product() {
    assert_eq!(
      grade_0(geometric_product(MULTIVECTOR_A, reverse(MULTIVECTOR_B))),
      dot(MULTIVECTOR_A, MULTIVECTOR_B)
    );
  }

  #[test]
  fn relationship_to_bulk_dual() {
    assert_eq!(
      bulk_dual(MULTIVECTOR_A),
      geometric_product(reverse(MULTIVECTOR_A), Multivector::E1234)
    );

    assert_eq!(
      left_bulk_dual(MULTIVECTOR_A),
      geometric_product(Multivector::E1234, reverse(MULTIVECTOR_A))
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
          Multivector::from(reverse(variant)),
          reverse(Multivector::from(variant))
        );
      }
    }
  }
}
