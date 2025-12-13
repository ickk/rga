use crate::algebra::values::{
  Antiscalar, Bivector, DualNumber, EvenGrade, Matrix4, Multivector, OddGrade,
  Scalar, Trivector, Vector,
};

// multiplication with scalar values

pub use ::core::ops::Mul;

macro_rules! impl_mul_bidi {
  ($($lhs:ty, $rhs:ty => $output:ty: $op_fn:ident;)*) => {
    $(
      impl Mul<$rhs> for $lhs {
        type Output = $output;

        #[inline]
        fn mul(self, rhs: $rhs) -> Self::Output {
          $op_fn(self, rhs)
        }
      }

      impl Mul<$lhs> for $rhs {
        type Output = $output;

        #[inline]
        fn mul(self, rhs: $lhs) -> Self::Output {
          $op_fn(rhs, self)
        }
      }
    )*
  };
}

impl Mul<Scalar> for Scalar {
  type Output = Scalar;

  #[inline]
  fn mul(self, Scalar { s: rs }: Scalar) -> Scalar {
    Scalar { s: self.s * rs }
  }
}

impl_mul_bidi! {
  Scalar, Multivector => Multivector: scalar_mul_multivector;
  Scalar, Vector => Vector: scalar_mul_vector;
  Scalar, Bivector => Bivector: scalar_mul_bivector;
  Scalar, Trivector => Trivector: scalar_mul_trivector;
  Scalar, Antiscalar => Antiscalar: scalar_mul_antiscalar;
  Scalar, DualNumber => DualNumber: scalar_mul_dual_number;
  Scalar, OddGrade => OddGrade: scalar_mul_odd_grade;
  Scalar, EvenGrade => EvenGrade: scalar_mul_even_grade;
  Scalar, Matrix4 => Matrix4: scalar_mul_matrix4;
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_multivector(
  Scalar { s: ls }: Scalar,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {
  let s = ls*rs;

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  let e1234 = ls*r1234;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_vector(
  Scalar { s: ls }: Scalar,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Vector {
  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_bivector(
  Scalar { s: ls }: Scalar,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Bivector {
  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_trivector(
  Scalar { s: ls }: Scalar,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Trivector {
  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_antiscalar(
  Scalar { s: ls }: Scalar,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Antiscalar {
  let e1234 = ls*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {
  let s = ls*rs;
  let e1234 = ls*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_odd_grade(
  Scalar { s: ls }: Scalar,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {
  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {
  let s = ls*rs;

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e1234 = ls*r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_mul_matrix4(
  Scalar { s: ls }: Scalar,
  Matrix4 {
    m11: b11, m12: b12, m13: b13, m14: b14,
    m21: b21, m22: b22, m23: b23, m24: b24,
    m31: b31, m32: b32, m33: b33, m34: b34,
    m41: b41, m42: b42, m43: b43, m44: b44,
  }: Matrix4,
) -> Matrix4 {
  let m11 = ls*b11;
  let m21 = ls*b21;
  let m31 = ls*b31;
  let m41 = ls*b41;

  let m12 = ls*b12;
  let m22 = ls*b22;
  let m32 = ls*b32;
  let m42 = ls*b42;

  let m13 = ls*b13;
  let m23 = ls*b23;
  let m33 = ls*b33;
  let m43 = ls*b43;

  let m14 = ls*b14;
  let m24 = ls*b24;
  let m34 = ls*b34;
  let m44 = ls*b44;

  Matrix4 {
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44,
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
      fn sparse_scalar_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(scalar * variant),
          scalar * Multivector::from(variant)
        );
        assert_eq!(
          Multivector::from(variant * scalar),
          Multivector::from(variant) * scalar
        );
      }
    }
  }
}
