use crate::algebra::values::{
  zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
  Scalar, Trivector, Vector,
};

/// u ∨ 𝐞₃₂₁ = u ∨ 𝐞̄₄
#[inline]
pub fn attitude<M>(u: M) -> <M as Attitude>::Output
where
  M: Attitude,
{
  u.attitude()
}

/// u ∨ 𝐞₃₂₁ = u ∨ 𝐞̄₄
pub trait Attitude {
  type Output;

  /// u ∨ 𝐞₃₂₁ = u ∨ 𝐞̄₄
  fn attitude(self) -> Self::Output;
}

impl Attitude for Multivector {
  type Output = Multivector;

  #[inline]
  fn attitude(self) -> Self::Output {
    let s = self.e4;
    let e1 = self.e41;
    let e2 = self.e42;
    let e3 = self.e43;
    let e23 = self.e423;
    let e31 = self.e431;
    let e12 = self.e412;
    let e321 = self.e1234;

    Multivector {
      s,
      e1,
      e2,
      e3,
      e23,
      e31,
      e12,
      e321,
      ..zero()
    }
  }
}

impl Attitude for Scalar {
  type Output = Scalar;

  #[inline(always)]
  fn attitude(self) -> Self::Output {
    Scalar::ZERO
  }
}

impl Attitude for Vector {
  type Output = Scalar;

  #[inline]
  fn attitude(self) -> Self::Output {
    Scalar { s: self.e4 }
  }
}

impl Attitude for Bivector {
  type Output = Vector;

  #[inline]
  fn attitude(self) -> Self::Output {
    Vector {
      e1: self.e41,
      e2: self.e42,
      e3: self.e43,
      ..zero()
    }
  }
}

impl Attitude for Trivector {
  type Output = Bivector;

  #[inline]
  fn attitude(self) -> Self::Output {
    Bivector {
      e23: self.e423,
      e31: self.e431,
      e12: self.e412,
      ..zero()
    }
  }
}

impl Attitude for Antiscalar {
  type Output = Trivector;

  #[inline]
  fn attitude(self) -> Self::Output {
    Trivector {
      e321: self.e1234,
      ..zero()
    }
  }
}

impl Attitude for DualNumber {
  type Output = Trivector;

  #[inline]
  fn attitude(self) -> Self::Output {
    Trivector {
      e321: self.e1234,
      ..zero()
    }
  }
}

impl Attitude for OddGrade {
  type Output = EvenGrade;

  #[inline]
  fn attitude(self) -> Self::Output {
    let s = self.e4;
    let e23 = self.e423;
    let e31 = self.e431;
    let e12 = self.e412;
    EvenGrade {
      s,
      e23,
      e31,
      e12,
      ..zero()
    }
  }
}

impl Attitude for EvenGrade {
  type Output = OddGrade;

  #[inline]
  fn attitude(self) -> Self::Output {
    let e1 = self.e41;
    let e2 = self.e42;
    let e3 = self.e43;
    let e321 = self.e1234;
    OddGrade {
      e1,
      e2,
      e3,
      e321,
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
          Multivector::from(attitude(variant)),
          antiwedge(variant, right_complement(Multivector::E4)),
          "The Attitude of an object is defined as the Antiwedge Product of \
          the object with the e321 unit trivector"
        );
      }
    }
  }
}
