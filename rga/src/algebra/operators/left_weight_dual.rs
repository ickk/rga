use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_unary_operation, return_scalar_zero_unary},
};

/// u<sub>☆</sub>
#[inline]
pub fn left_weight_dual<M>(u: M) -> <M as LeftWeightDual>::Output
where
  M: LeftWeightDual,
{
  u.left_weight_dual()
}

/// u<sub>☆</sub>
pub trait LeftWeightDual {
  type Output;

  /// u<sub>☆</sub>
  fn left_weight_dual(self) -> Self::Output;
}

impl_unary_operation!(LeftWeightDual::left_weight_dual {
  Multivector => Multivector: left_weight_dual_multivector;
  Scalar => Scalar: return_scalar_zero_unary;
  Vector => Trivector: left_weight_dual_vector;
  Bivector => Bivector: left_weight_dual_bivector;
  Trivector => Vector: left_weight_dual_trivector;
  Antiscalar => Scalar: left_weight_dual_antiscalar;
  DualNumber => Scalar: left_weight_dual_dual_number;
  OddGrade => OddGrade: left_weight_dual_odd_grade;
  EvenGrade => EvenGrade: left_weight_dual_even_grade;
});

#[rustfmt::skip]
#[inline]
fn left_weight_dual_multivector(
  Multivector {
    e4: m4,
    e43: m43, e42: m42, e41: m41,
    e423: m423, e431: m431, e412: m412,
    e1234: m1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = m1234;

  let e1 = m423;
  let e2 = m431;
  let e3 = m412;

  let e23 = -m41;
  let e31 = -m42;
  let e12 = -m43;

  let e321 = -m4;

  Multivector {
    s,
    e1, e2, e3,
    e23, e31, e12,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn left_weight_dual_vector(
  Vector { e4: m4, .. }: Vector,
) -> Trivector {

  let e321 = -m4;

  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn left_weight_dual_bivector(
  Bivector { e43: m43, e42: m42, e41: m41, .. }: Bivector,
) -> Bivector {

  let e23 = -m41;
  let e31 = -m42;
  let e12 = -m43;

  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn left_weight_dual_trivector(
  Trivector { e423: m423, e431: m431, e412: m412, .. }: Trivector,
) -> Vector {

  let e1 = m423;
  let e2 = m431;
  let e3 = m412;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn left_weight_dual_antiscalar(
  Antiscalar { e1234: m1234 }: Antiscalar,
) -> Scalar {

  let s = m1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn left_weight_dual_dual_number(
  DualNumber { e1234: m1234, .. }: DualNumber,
) -> Scalar {

  let s = m1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn left_weight_dual_odd_grade(
  OddGrade {
    e4: m4,
    e423: m423, e431: m431, e412: m412,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = m423;
  let e2 = m431;
  let e3 = m412;

  let e321 = -m4;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn left_weight_dual_even_grade(
  EvenGrade {
    e43: m43, e42: m42, e41: m41,
    e1234: m1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = m1234;

  let e23 = -m41;
  let e31 = -m42;
  let e12 = -m43;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
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
          Multivector::from(left_weight_dual(variant)),
          left_weight_dual(Multivector::from(variant))
        );
      }
    }
  }
}
