use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_unary_operation, return_scalar_zero_unary},
};

/// u<sub>★</sub>
#[inline]
pub fn left_bulk_dual<M>(u: M) -> <M as LeftBulkDual>::Output
where
  M: LeftBulkDual,
{
  u.left_bulk_dual()
}

/// u<sub>★</sub>
pub trait LeftBulkDual {
  type Output;

  /// u<sub>★</sub>
  fn left_bulk_dual(self) -> Self::Output;
}

impl_unary_operation!(LeftBulkDual::left_bulk_dual {
  Multivector => Multivector: left_bulk_dual_multivector;
  Scalar => Antiscalar: left_bulk_dual_scalar;
  Vector => Trivector: left_bulk_dual_vector;
  Bivector => Bivector: left_bulk_dual_bivector;
  Trivector => Vector: left_bulk_dual_trivector;
  Antiscalar => Scalar: return_scalar_zero_unary;
  DualNumber => Antiscalar: left_bulk_dual_dual_number;
  OddGrade => OddGrade: left_bulk_dual_odd_grade;
  EvenGrade => EvenGrade: left_bulk_dual_even_grade;
});

#[rustfmt::skip]
#[inline]
fn left_bulk_dual_multivector(
  Multivector {
    s: ms,
    e1: m1, e2: m2, e3: m3,
    e23: m23, e31: m31, e12: m12,
    e321: m321,
    ..
  }: Multivector,
) -> Multivector {

  let e4 = m321;

  let e41 = -m23;
  let e42 = -m31;
  let e43 = -m12;

  let e423 = -m1;
  let e431 = -m2;
  let e412 = -m3;

  let e1234 = ms;

  Multivector {
    e4,
    e41, e42, e43,
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn left_bulk_dual_scalar(
  Scalar { s }: Scalar,
) -> Antiscalar {

  let e1234 = s;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn left_bulk_dual_vector(
  Vector { e1: m1, e2: m2, e3: m3, .. }: Vector,
) -> Trivector {

  let e423 = -m1;
  let e431 = -m2;
  let e412 = -m3;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn left_bulk_dual_bivector(
  Bivector { e23: m23, e31: m31, e12: m12, .. }: Bivector,
) -> Bivector {

  let e41 = -m23;
  let e42 = -m31;
  let e43 = -m12;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn left_bulk_dual_trivector(
  Trivector { e321: m321, .. }: Trivector,
) -> Vector {

  let e4 = m321;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn left_bulk_dual_dual_number(
  DualNumber { s: ms, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = ms;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn left_bulk_dual_odd_grade(
  OddGrade {
    e1: m1, e2: m2, e3: m3,
    e321: m321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e4 = m321;

  let e423 = -m1;
  let e431 = -m2;
  let e412 = -m3;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn left_bulk_dual_even_grade(
  EvenGrade {
    s: ms,
    e23: m23, e31: m31, e12: m12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let e41 = -m23;
  let e42 = -m31;
  let e43 = -m12;

  let e1234 = ms;

  EvenGrade {
    e41, e42, e43,
    e1234,
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
          Multivector::from(left_bulk_dual(variant)),
          left_bulk_dual(Multivector::from(variant))
        );
      }
    }
  }
}
