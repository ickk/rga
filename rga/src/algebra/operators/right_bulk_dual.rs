use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_unary_operation, return_scalar_zero_unary},
};

/// u<sup>★</sup>
///
/// This is the [`RightBulkDual`].
#[inline]
pub fn bulk_dual<M>(u: M) -> <M as RightBulkDual>::Output
where
  M: RightBulkDual,
{
  u.bulk_dual()
}

/// u<sup>★</sup>
///
/// The *right bulk dual* is also known as the *Hodge dual*.
pub trait RightBulkDual {
  type Output;

  /// u<sup>★</sup>
  ///
  /// The *right bulk dual* is also known as the *Hodge dual*.
  #[doc(
    alias = "right bulk dual",
    alias = "metric dual",
    alias = "Hodge dual"
  )]
  fn bulk_dual(self) -> Self::Output;
}

impl_unary_operation!(RightBulkDual::bulk_dual {
  Multivector => Multivector: right_bulk_dual_multivector;
  Scalar => Antiscalar: right_bulk_dual_scalar;
  Vector => Trivector: right_bulk_dual_vector;
  Bivector => Bivector: right_bulk_dual_bivector;
  Trivector => Vector: right_bulk_dual_trivector;
  Antiscalar => Scalar: return_scalar_zero_unary;
  DualNumber => Antiscalar: right_bulk_dual_dual_number;
  OddGrade => OddGrade: right_bulk_dual_odd_grade;
  EvenGrade => EvenGrade: right_bulk_dual_even_grade;
});

#[rustfmt::skip]
#[inline]
fn right_bulk_dual_multivector(
  Multivector {
    s: ms,
    e1: m1, e2: m2, e3: m3,
    e23: m23, e31: m31, e12: m12,
    e321: m321,
    ..
  }: Multivector,
) -> Multivector {

  let e4 = -m321;

  let e41 = -m23;
  let e42 = -m31;
  let e43 = -m12;

  let e423 = m1;
  let e431 = m2;
  let e412 = m3;

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
fn right_bulk_dual_scalar(
  Scalar { s: ms }: Scalar,
) -> Antiscalar {

  let e1234 = ms;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn right_bulk_dual_vector(
  Vector { e1: m1, e2: m2, e3: m3, .. }: Vector,
) -> Trivector {

  let e423 = m1;
  let e431 = m2;
  let e412 = m3;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn right_bulk_dual_bivector(
  Bivector { e23: m23, e31: m31, e12: m12, .. }: Bivector,
) -> Bivector {

  let e41 = -m23;
  let e42 = -m31;
  let e43 = -m12;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn right_bulk_dual_trivector(
  Trivector { e321: m321, .. }: Trivector,
) -> Vector {

  let e4 = -m321;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn right_bulk_dual_dual_number(
  DualNumber { s: ms, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = ms;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn right_bulk_dual_odd_grade(
  OddGrade {
    e1: m1, e2: m2, e3: m3,
    e321: m321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e4 = -m321;

  let e423 = m1;
  let e431 = m2;
  let e412 = m3;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn right_bulk_dual_even_grade(
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

  #[test]
  fn definition() {
    assert_eq!(
      bulk_dual(MULTIVECTOR_A),
      right_complement(bulk(MULTIVECTOR_A)),
      "The Right Bulk Dual of an object is defined as the Right Complement of \
      the Bulk of an object"
    );
  }

  #[test]
  fn distribution_over_wedge_product() {
    assert_eq!(
      bulk_dual(wedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      right_complement(bulk(wedge(MULTIVECTOR_A, MULTIVECTOR_B)))
    );

    assert_eq!(
      bulk_dual(wedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      antiwedge(bulk_dual(MULTIVECTOR_A), bulk_dual(MULTIVECTOR_B)),
    );
  }

  #[test]
  fn identities() {
    assert_eq!(
      bulk_dual(MULTIVECTOR_A),
      geometric_product(reverse(MULTIVECTOR_A), Antiscalar::E1234),
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
          Multivector::from(bulk_dual(variant)),
          bulk_dual(Multivector::from(variant))
        );
      }
    }
  }
}
