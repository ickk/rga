use crate::{
  algebra::values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::impl_unary_operation,
};

/// ū
#[inline]
pub fn right_complement<M>(u: M) -> <M as RightComplement>::Output
where
  M: RightComplement,
{
  u.right_complement()
}

/// ū
pub trait RightComplement {
  type Output;

  /// ū
  fn right_complement(self) -> Self::Output;
}

impl_unary_operation!(RightComplement::right_complement {
  Multivector => Multivector: right_complement_multivector;
  Scalar => Antiscalar: right_complement_scalar;
  Vector => Trivector: right_complement_vector;
  Bivector => Bivector: right_complement_bivector;
  Trivector => Vector: right_complement_trivector;
  Antiscalar => Scalar: right_complement_antiscalar;
  DualNumber => DualNumber: right_complement_dual_number;
  OddGrade => OddGrade: right_complement_odd_grade;
  EvenGrade => EvenGrade: right_complement_even_grade;
});

#[rustfmt::skip]
#[inline]
fn right_complement_multivector(
  Multivector {
    s: ms,
    e1: m1, e2: m2, e3: m3, e4: m4,
    e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
    e423: m423, e431: m431, e412: m412, e321: m321,
    e1234: m1234,
  }: Multivector,
) -> Multivector {

  let s = m1234;

  let e1 = -m423;
  let e2 = -m431;
  let e3 = -m412;
  let e4 = -m321;

  let e41 = -m23;
  let e42 = -m31;
  let e43 = -m12;
  let e23 = -m41;
  let e31 = -m42;
  let e12 = -m43;

  let e423 = m1;
  let e431 = m2;
  let e412 = m3;
  let e321 = m4;

  let e1234 = ms;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn right_complement_scalar(
  Scalar { s: ms }: Scalar
) -> Antiscalar {
  let e1234 = ms;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn right_complement_vector(
  Vector { e1: m1, e2: m2, e3: m3, e4: m4 }: Vector,
) -> Trivector {

  let e423 = m1;
  let e431 = m2;
  let e412 = m3;
  let e321 = m4;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn right_complement_bivector(
  Bivector {
    e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
  }: Bivector,
) -> Bivector {

  let e41 = -m23;
  let e42 = -m31;
  let e43 = -m12;
  let e23 = -m41;
  let e31 = -m42;
  let e12 = -m43;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn right_complement_trivector(
  Trivector { e423: m423, e431: m431, e412: m412, e321: m321 }: Trivector,
) -> Vector {

  let e1 = -m423;
  let e2 = -m431;
  let e3 = -m412;
  let e4 = -m321;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn right_complement_antiscalar(
  Antiscalar { e1234: m1234 }: Antiscalar,
) -> Scalar {

  let s = m1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn right_complement_dual_number(
  DualNumber { s: ms, e1234: m1234 }: DualNumber,
) -> DualNumber {

  let s = m1234;
  let e1234 = ms;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn right_complement_odd_grade(
  OddGrade {
    e1: m1, e2: m2, e3: m3, e4: m4,
    e423: m423, e431: m431, e412: m412, e321: m321,
  }: OddGrade,
) -> OddGrade {

  let e1 = -m423;
  let e2 = -m431;
  let e3 = -m412;
  let e4 = -m321;

  let e423 = m1;
  let e431 = m2;
  let e412 = m3;
  let e321 = m4;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn right_complement_even_grade(
  EvenGrade {
    s: ms,
    e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
    e1234: m1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = m1234;

  let e41 = -m23;
  let e42 = -m31;
  let e43 = -m12;
  let e23 = -m41;
  let e31 = -m42;
  let e12 = -m43;

  let e1234 = ms;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
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
    let s = Multivector { s: 1., ..zero() };
    let e1 = Multivector { e1: 1., ..zero() };
    let e2 = Multivector { e2: 1., ..zero() };
    let e3 = Multivector { e3: 1., ..zero() };
    let e4 = Multivector { e4: 1., ..zero() };
    let e41 = Multivector { e41: 1., ..zero() };
    let e42 = Multivector { e42: 1., ..zero() };
    let e43 = Multivector { e43: 1., ..zero() };
    let e23 = Multivector { e23: 1., ..zero() };
    let e31 = Multivector { e31: 1., ..zero() };
    let e12 = Multivector { e12: 1., ..zero() };
    let e423 = Multivector { e423: 1., ..zero() };
    let e431 = Multivector { e431: 1., ..zero() };
    let e412 = Multivector { e412: 1., ..zero() };
    let e321 = Multivector { e321: 1., ..zero() };
    let e1234 = Multivector {
      e1234: 1.,
      ..zero()
    };

    assert_eq!(wedge(s, right_complement(s)), e1234);
    assert_eq!(wedge(e1, right_complement(e1)), e1234);
    assert_eq!(wedge(e2, right_complement(e2)), e1234);
    assert_eq!(wedge(e3, right_complement(e3)), e1234);
    assert_eq!(wedge(e4, right_complement(e4)), e1234);
    assert_eq!(wedge(e41, right_complement(e41)), e1234);
    assert_eq!(wedge(e42, right_complement(e42)), e1234);
    assert_eq!(wedge(e43, right_complement(e43)), e1234);
    assert_eq!(wedge(e23, right_complement(e23)), e1234);
    assert_eq!(wedge(e31, right_complement(e31)), e1234);
    assert_eq!(wedge(e12, right_complement(e12)), e1234);
    assert_eq!(wedge(e423, right_complement(e423)), e1234);
    assert_eq!(wedge(e431, right_complement(e431)), e1234);
    assert_eq!(wedge(e412, right_complement(e412)), e1234);
    assert_eq!(wedge(e321, right_complement(e321)), e1234);
    assert_eq!(wedge(e1234, right_complement(e1234)), e1234);
  }

  #[test]
  fn inverse() {
    assert_eq!(
      MULTIVECTOR_A,
      left_complement(right_complement(MULTIVECTOR_A),),
      "The left complement & right complement are inverses of each other"
    );
    assert_eq!(
      MULTIVECTOR_A,
      right_complement(left_complement(MULTIVECTOR_A)),
      "The left complement & right complement are inverses of each other"
    );
  }

  #[test]
  fn linearity() {
    assert_eq!(
      right_complement(SCALAR_A * MULTIVECTOR_A),
      SCALAR_A * right_complement(MULTIVECTOR_A),
      "The right complement is a linear operation"
    );
    assert_eq!(
      right_complement(MULTIVECTOR_A + MULTIVECTOR_B),
      right_complement(MULTIVECTOR_A) + right_complement(MULTIVECTOR_B),
      "The right complement is a linear operation"
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
          Multivector::from(right_complement(variant)),
          right_complement(Multivector::from(variant))
        );
      }
    }
  }
}
