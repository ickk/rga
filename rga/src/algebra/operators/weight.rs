use crate::algebra::values::{
  zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
  Scalar, Trivector, Vector,
};

/// u<sub>○</sub>
#[inline]
pub fn weight<M>(u: M) -> <M as Weight>::Output
where
  M: Weight,
{
  u.weight()
}

/// u<sub>○</sub>
pub trait Weight {
  type Output;

  /// u<sub>○</sub>
  fn weight(self) -> Self::Output;
}

impl Weight for Multivector {
  type Output = Multivector;

  #[inline]
  fn weight(self) -> Self::Output {
    Multivector {
      e4: self.e4,
      e41: self.e41,
      e42: self.e42,
      e43: self.e43,
      e423: self.e423,
      e431: self.e431,
      e412: self.e412,
      e1234: self.e1234,
      ..zero()
    }
  }
}

impl Weight for Scalar {
  type Output = Antiscalar;

  #[inline(always)]
  fn weight(self) -> Self::Output {
    Antiscalar::ZERO
  }
}

impl Weight for Vector {
  type Output = Vector;

  #[inline]
  fn weight(self) -> Self::Output {
    Vector {
      e4: self.e4,
      ..zero()
    }
  }
}

impl Weight for Bivector {
  type Output = Bivector;

  #[inline]
  fn weight(self) -> Self::Output {
    Bivector {
      e41: self.e41,
      e42: self.e42,
      e43: self.e43,
      ..zero()
    }
  }
}

impl Weight for Trivector {
  type Output = Trivector;

  #[inline]
  fn weight(self) -> Self::Output {
    Trivector {
      e423: self.e423,
      e431: self.e431,
      e412: self.e412,
      ..zero()
    }
  }
}

impl Weight for Antiscalar {
  type Output = Antiscalar;

  #[inline(always)]
  fn weight(self) -> Self::Output {
    self
  }
}

impl Weight for DualNumber {
  type Output = Antiscalar;

  #[inline]
  fn weight(self) -> Self::Output {
    Antiscalar { e1234: self.e1234 }
  }
}

impl Weight for OddGrade {
  type Output = OddGrade;

  #[inline]
  fn weight(self) -> Self::Output {
    OddGrade {
      e4: self.e4,
      e423: self.e423,
      e431: self.e431,
      e412: self.e412,
      ..zero()
    }
  }
}

impl Weight for EvenGrade {
  type Output = EvenGrade;

  #[inline]
  fn weight(self) -> Self::Output {
    EvenGrade {
      e41: self.e41,
      e42: self.e42,
      e43: self.e43,
      e1234: self.e1234,
      ..zero()
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
          Multivector::from(weight(variant)),
          weight(Multivector::from(variant))
        );
      }
    }
  }
}
