use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{identity, impl_unary_operation, return_scalar_zero_unary},
};

/// u<sub>●</sub>
#[inline]
pub fn bulk<M>(u: M) -> <M as Bulk>::Output
where
  M: Bulk,
{
  u.bulk()
}

/// u<sub>●</sub>
pub trait Bulk {
  type Output;

  /// u<sub>●</sub>
  fn bulk(self) -> Self::Output;
}

impl_unary_operation!(Bulk::bulk {
  Multivector => Multivector: bulk_multivector;
  Scalar => Scalar: identity;
  Vector => Vector: bulk_vector;
  Bivector => Bivector: bulk_bivector;
  Trivector => Trivector: bulk_trivector;
  Antiscalar => Scalar: return_scalar_zero_unary;
  DualNumber => Scalar: bulk_dual_number;
  OddGrade => OddGrade: bulk_odd_grade;
  EvenGrade => EvenGrade: bulk_even_grade;
});

#[rustfmt::skip]
#[inline]
fn bulk_multivector(
  Multivector {
    s,
    e1, e2, e3,
    e23, e31, e12,
    e321,
    ..
  }: Multivector,
) -> Multivector {
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
fn bulk_vector(
  Vector { e1, e2, e3, .. }: Vector,
) -> Vector {
  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bulk_bivector(
  Bivector { e23, e31, e12, .. }: Bivector,
) -> Bivector {
  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bulk_trivector(
  Trivector { e321, .. }: Trivector,
) -> Trivector {
  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bulk_dual_number(
  DualNumber { s, .. }: DualNumber,
) -> Scalar {
  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn bulk_odd_grade(
  OddGrade {
    e1, e2, e3,
    e321,
    ..
  }: OddGrade,
) -> OddGrade {
  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bulk_even_grade(
  EvenGrade {
    s,
    e23, e31, e12,
    ..
  }: EvenGrade,
) -> EvenGrade {
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

  #[test]
  fn bulk_and_weight_sum_to_self() {
    assert_eq!(MULTIVECTOR_A, bulk(MULTIVECTOR_A) + weight(MULTIVECTOR_A));
  }

  #[test]
  fn wedge_decomposition_into_bulk_and_weight() {
    assert_eq!(
      wedge(MULTIVECTOR_A, MULTIVECTOR_B),
      wedge(
        bulk(MULTIVECTOR_A) + weight(MULTIVECTOR_A),
        bulk(MULTIVECTOR_B) + weight(MULTIVECTOR_B)
      )
    );

    assert_eq!(
      bulk(wedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      wedge(bulk(MULTIVECTOR_A), bulk(MULTIVECTOR_B)),
      "Bulk distributes over the Wedge Product"
    );

    assert_eq!(
      weight(wedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      (wedge(bulk(MULTIVECTOR_A), weight(MULTIVECTOR_B))
        + wedge(weight(MULTIVECTOR_A), bulk(MULTIVECTOR_B))),
    );
  }

  #[test]
  fn antiwedge_decomposition_into_bulk_and_weight() {
    assert_eq!(
      antiwedge(MULTIVECTOR_A, MULTIVECTOR_B),
      antiwedge(
        bulk(MULTIVECTOR_A) + weight(MULTIVECTOR_A),
        bulk(MULTIVECTOR_B) + weight(MULTIVECTOR_B)
      )
    );

    assert_eq!(
      bulk(antiwedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      (antiwedge(bulk(MULTIVECTOR_A), weight(MULTIVECTOR_B))
        + antiwedge(weight(MULTIVECTOR_A), bulk(MULTIVECTOR_B))),
    );

    assert_eq!(
      weight(antiwedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      antiwedge(weight(MULTIVECTOR_A), weight(MULTIVECTOR_B)),
      "Weight distributes over the Antiwedge Product"
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
          Multivector::from(bulk(variant)),
          bulk(Multivector::from(variant))
        );
      }
    }
  }
}
