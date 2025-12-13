use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_binary_operation, return_scalar_zero_binary},
};

/// a ∨ b<sup>☆</sup>
#[inline]
pub fn weight_contraction<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as WeightContraction<Rhs>>::Output
where
  Lhs: WeightContraction<Rhs>,
{
  a.weight_contraction(b)
}

/// a ∨ b<sup>☆</sup>
pub trait WeightContraction<Rhs> {
  type Output;

  /// a ∨ b<sup>☆</sup>
  #[doc(alias = "interior product", alias = "product")]
  fn weight_contraction(self, b: Rhs) -> Self::Output;
}

impl_binary_operation!(WeightContraction::weight_contraction {
  Multivector, Multivector => Multivector: multivector_weight_contraction_multivector;
  Multivector, Scalar => Scalar: return_scalar_zero_binary;
  Multivector, Vector => Multivector: multivector_weight_contraction_vector;
  Multivector, Bivector => Multivector: multivector_weight_contraction_bivector;
  Multivector, Trivector => Multivector: multivector_weight_contraction_trivector;
  Multivector, Antiscalar => Scalar: multivector_weight_contraction_antiscalar;
  Multivector, DualNumber => Scalar: multivector_weight_contraction_dual_number;
  Multivector, OddGrade => Multivector: multivector_weight_contraction_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_weight_contraction_even_grade;

  Scalar, Multivector => Scalar: return_scalar_zero_binary;
  Scalar, Scalar => Scalar: return_scalar_zero_binary;
  Scalar, Vector => Scalar: return_scalar_zero_binary;
  Scalar, Bivector => Scalar: return_scalar_zero_binary;
  Scalar, Trivector => Scalar: return_scalar_zero_binary;
  Scalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Scalar, DualNumber => Scalar: return_scalar_zero_binary;
  Scalar, OddGrade => Scalar: return_scalar_zero_binary;
  Scalar, EvenGrade => Scalar: return_scalar_zero_binary;

  Vector, Multivector => Scalar: vector_weight_contraction_multivector;
  Vector, Scalar => Scalar: return_scalar_zero_binary;
  Vector, Vector => Scalar: vector_weight_contraction_vector;
  Vector, Bivector => Scalar: return_scalar_zero_binary;
  Vector, Trivector => Scalar: return_scalar_zero_binary;
  Vector, Antiscalar => Scalar: return_scalar_zero_binary;
  Vector, DualNumber => Scalar: return_scalar_zero_binary;
  Vector, OddGrade => Scalar: vector_weight_contraction_odd_grade;
  Vector, EvenGrade => Scalar: return_scalar_zero_binary;

  Bivector, Multivector => Multivector: bivector_weight_contraction_multivector;
  Bivector, Scalar => Scalar: return_scalar_zero_binary;
  Bivector, Vector => Vector: bivector_weight_contraction_vector;
  Bivector, Bivector => Scalar: bivector_weight_contraction_bivector;
  Bivector, Trivector => Scalar: return_scalar_zero_binary;
  Bivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Bivector, DualNumber => Scalar: return_scalar_zero_binary;
  Bivector, OddGrade => Vector: bivector_weight_contraction_odd_grade;
  Bivector, EvenGrade => Scalar: bivector_weight_contraction_even_grade;

  Trivector, Multivector => Multivector: trivector_weight_contraction_multivector;
  Trivector, Scalar => Scalar: return_scalar_zero_binary;
  Trivector, Vector => Bivector: trivector_weight_contraction_vector;
  Trivector, Bivector => Vector: trivector_weight_contraction_bivector;
  Trivector, Trivector => Scalar: trivector_weight_contraction_trivector;
  Trivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Trivector, DualNumber => Scalar: return_scalar_zero_binary;
  Trivector, OddGrade => EvenGrade: trivector_weight_contraction_odd_grade;
  Trivector, EvenGrade => Vector: trivector_weight_contraction_even_grade;

  Antiscalar, Multivector => Multivector: antiscalar_weight_contraction_multivector;
  Antiscalar, Scalar => Scalar: return_scalar_zero_binary;
  Antiscalar, Vector => Trivector: antiscalar_weight_contraction_vector;
  Antiscalar, Bivector => Bivector: antiscalar_weight_contraction_bivector;
  Antiscalar, Trivector => Vector: antiscalar_weight_contraction_trivector;
  Antiscalar, Antiscalar => Scalar: antiscalar_weight_contraction_antiscalar;
  Antiscalar, DualNumber => Scalar: antiscalar_weight_contraction_dual_number;
  Antiscalar, OddGrade => OddGrade: antiscalar_weight_contraction_odd_grade;
  Antiscalar, EvenGrade => EvenGrade: antiscalar_weight_contraction_even_grade;

  DualNumber, Multivector => Multivector: dual_number_weight_contraction_multivector;
  DualNumber, Scalar => Scalar: return_scalar_zero_binary;
  DualNumber, Vector => Trivector: dual_number_weight_contraction_vector;
  DualNumber, Bivector => Bivector: dual_number_weight_contraction_bivector;
  DualNumber, Trivector => Vector: dual_number_weight_contraction_trivector;
  DualNumber, Antiscalar => Scalar: dual_number_weight_contraction_antiscalar;
  DualNumber, DualNumber => Scalar: dual_number_weight_contraction_dual_number;
  DualNumber, OddGrade => OddGrade: dual_number_weight_contraction_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_weight_contraction_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_weight_contraction_multivector;
  OddGrade, Scalar => Scalar: return_scalar_zero_binary;
  OddGrade, Vector => EvenGrade: odd_grade_weight_contraction_vector;
  OddGrade, Bivector => Vector: odd_grade_weight_contraction_bivector;
  OddGrade, Trivector => Scalar: odd_grade_weight_contraction_trivector;
  OddGrade, Antiscalar => Scalar: return_scalar_zero_binary;
  OddGrade, DualNumber => Scalar: return_scalar_zero_binary;
  OddGrade, OddGrade => EvenGrade: odd_grade_weight_contraction_odd_grade;
  OddGrade, EvenGrade => Vector: odd_grade_weight_contraction_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_weight_contraction_multivector;
  EvenGrade, Scalar => Scalar: return_scalar_zero_binary;
  EvenGrade, Vector => OddGrade: even_grade_weight_contraction_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_weight_contraction_bivector;
  EvenGrade, Trivector => Vector: even_grade_weight_contraction_trivector;
  EvenGrade, Antiscalar => Scalar: even_grade_weight_contraction_antiscalar;
  EvenGrade, DualNumber => Scalar: even_grade_weight_contraction_dual_number;
  EvenGrade, OddGrade => OddGrade: even_grade_weight_contraction_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_weight_contraction_even_grade;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_weight_contraction_multivector(
  Multivector {
    e4: l4,
    e41: l41, e42: l42, e43: l43,
    e423: l423, e431: l431, e412: l412,
    e1234: l1234,
    ..
  }: Multivector,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = l4*r4 + l41*r41 + l42*r42 + l43*r43
    + l423*r423 + l431*r431 + l412*r412 + l1234*r1234;

  let e1 = l41*r4 + l431*r43 - l412*r42 - l1234*r423;
  let e2 = l42*r4 + l412*r41 - l423*r43 - l1234*r431;
  let e3 = l43*r4 + l423*r42 - l431*r41 - l1234*r412;

  let e23 = l423*r4 - l1234*r41;
  let e31 = l431*r4 - l1234*r42;
  let e12 = l412*r4 - l1234*r43;

  let e321 = l1234*r4;

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
fn multivector_weight_contraction_vector(
  Multivector {
    e4: l4,
    e41: l41, e42: l42, e43: l43,
    e423: l423, e431: l431, e412: l412,
    e1234: l1234,
    ..
  }: Multivector,
  Vector { e4: r4, .. }: Vector,
) -> Multivector {

  let s = l4*r4;

  let e1 = l41*r4;
  let e2 = l42*r4;
  let e3 = l43*r4;

  let e23 = l423*r4;
  let e31 = l431*r4;
  let e12 = l412*r4;

  let e321 = l1234*r4;

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
fn multivector_weight_contraction_bivector(
  Multivector {
    e41: l41, e42: l42, e43: l43,
    e423: l423, e431: l431, e412: l412,
    e1234: l1234,
    ..
  }: Multivector,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Multivector {

  let s = l41*r41 + l42*r42 + l43*r43;

  let e1 = l431*r43 - l412*r42;
  let e2 = l412*r41 - l423*r43;
  let e3 = l423*r42 - l431*r41;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  Multivector {
    s,
    e1, e2, e3,
    e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_weight_contraction_trivector(
  Multivector {
    e423: l423, e431: l431, e412: l412,
    e1234: l1234,
    ..
  }: Multivector,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Multivector {

  let s = l423*r423 + l431*r431 + l412*r412;

  let e1 = -l1234*r423;
  let e2 = -l1234*r431;
  let e3 = -l1234*r412;

  Multivector {
    s,
    e1, e2, e3,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_weight_contraction_antiscalar(
  Multivector { e1234: l1234, .. }: Multivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Scalar {

  let s = l1234*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn multivector_weight_contraction_dual_number(
  Multivector { e1234: l1234, .. }: Multivector,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Scalar {

  let s = l1234*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn multivector_weight_contraction_odd_grade(
  Multivector {
    e4: l4,
    e41: l41, e42: l42, e43: l43,
    e423: l423, e431: l431, e412: l412,
    e1234: l1234,
    ..
  }: Multivector,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> Multivector {

  let s = l4*r4 + l423*r423 + l431*r431 + l412*r412;

  let e1 = l41*r4 - l1234*r423;
  let e2 = l42*r4 - l1234*r431;
  let e3 = l43*r4 - l1234*r412;

  let e23 = l423*r4;
  let e31 = l431*r4;
  let e12 = l412*r4;

  let e321 = l1234*r4;

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
fn multivector_weight_contraction_even_grade(
  Multivector {
    e41: l41, e42: l42, e43: l43,
    e423: l423, e431: l431, e412: l412,
    e1234: l1234,
    ..
  }: Multivector,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> Multivector {

  let s = l41*r41 + l42*r42 + l43*r43 + l1234*r1234;

  let e1 = l431*r43 - l412*r42;
  let e2 = l412*r41 - l423*r43;
  let e3 = l423*r42 - l431*r41;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  Multivector {
    s,
    e1, e2, e3,
    e23, e31, e12,
    ..zero()
  }
}

// vector

#[rustfmt::skip]
#[inline]
fn vector_weight_contraction_multivector(
  Vector { e4: l4, .. }: Vector,
  Multivector { e4: r4, .. }: Multivector,
) -> Scalar {

  let s = l4*r4;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_contraction_vector(
  Vector { e4: l4, .. }: Vector,
  Vector { e4: r4, .. }: Vector,
) -> Scalar {

  let s = l4*r4;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_contraction_odd_grade(
  Vector { e4: l4, .. }: Vector,
  OddGrade { e4: r4, .. }: OddGrade,
) -> Scalar {

  let s = l4*r4;

  Scalar { s }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_weight_contraction_multivector(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    ..
  }: Multivector,
) -> Multivector {

  let s = l41*r41 + l42*r42 + l43*r43;

  let e1 = l41*r4;
  let e2 = l42*r4;
  let e3 = l43*r4;

  Multivector {
    s,
    e1, e2, e3,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_contraction_vector(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  Vector { e4: r4, .. }: Vector,
) -> Vector {

  let e1 = l41*r4;
  let e2 = l42*r4;
  let e3 = l43*r4;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_contraction_bivector(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Scalar {

  let s = l41*r41 + l42*r42 + l43*r43;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_contraction_odd_grade(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  OddGrade { e4: r4, .. }: OddGrade,
) -> Vector {

  let e1 = l41*r4;
  let e2 = l42*r4;
  let e3 = l43*r4;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_contraction_even_grade(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  EvenGrade { e41: r41, e42: r42, e43: r43, .. }: EvenGrade,
) -> Scalar {

  let s = l41*r41 + l42*r42 + l43*r43;

  Scalar { s }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_weight_contraction_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    ..
  }: Multivector,
) -> Multivector {

  let s = l423*r423 + l431*r431 + l412*r412;

  let e1 = l431*r43 - l412*r42;
  let e2 = l412*r41 - l423*r43;
  let e3 = l423*r42 - l431*r41;

  let e23 = l423*r4;
  let e31 = l431*r4;
  let e12 = l412*r4;

  Multivector {
    s,
    e1, e2, e3,
    e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_contraction_vector(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  Vector { e4: r4, .. }: Vector,
) -> Bivector {

  let e23 = l423*r4;
  let e31 = l431*r4;
  let e12 = l412*r4;

  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_contraction_bivector(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Vector {

  let e1 = l431*r43 - l412*r42;
  let e2 = l412*r41 - l423*r43;
  let e3 = l423*r42 - l431*r41;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_contraction_trivector(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Scalar {

  let s = l423*r423 + l431*r431 + l412*r412;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_contraction_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> EvenGrade {

  let s = l423*r423 + l431*r431 + l412*r412;

  let e23 = l423*r4;
  let e31 = l431*r4;
  let e12 = l412*r4;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_contraction_even_grade(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  EvenGrade { e41: r41, e42: r42, e43: r43, .. }: EvenGrade,
) -> Vector {

  let e1 = l431*r43 - l412*r42;
  let e2 = l412*r41 - l423*r43;
  let e3 = l423*r42 - l431*r41;

  Vector { e1, e2, e3, ..zero() }
}

// antiscalar

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_contraction_multivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = l1234*r1234;

  let e1 = -l1234*r423;
  let e2 = -l1234*r431;
  let e3 = -l1234*r412;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  let e321 = l1234*r4;

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
fn antiscalar_weight_contraction_vector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Vector { e4: r4, .. }: Vector,
) -> Trivector {

  let e321 = l1234*r4;

  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_contraction_bivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Bivector {

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_contraction_trivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Vector {

  let e1 = -l1234*r423;
  let e2 = -l1234*r431;
  let e3 = -l1234*r412;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_contraction_antiscalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Scalar {

  let s = l1234*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_contraction_dual_number(
  Antiscalar { e1234: l1234 }: Antiscalar,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Scalar {

  let s = l1234*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_contraction_odd_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = -l1234*r423;
  let e2 = -l1234*r431;
  let e3 = -l1234*r412;

  let e321 = l1234*r4;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_contraction_even_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = l1234*r1234;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
  }
}

// dual number

#[rustfmt::skip]
#[inline]
fn dual_number_weight_contraction_multivector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = l1234*r1234;

  let e1 = -l1234*r423;
  let e2 = -l1234*r431;
  let e3 = -l1234*r412;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  let e321 = l1234*r4;

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
fn dual_number_weight_contraction_vector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Vector { e4: r4, .. }: Vector,
) -> Trivector {

  let e321 = l1234*r4;

  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_contraction_bivector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Bivector {

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_contraction_trivector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Vector {

  let e1 = -l1234*r423;
  let e2 = -l1234*r431;
  let e3 = -l1234*r412;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_contraction_antiscalar(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Antiscalar { e1234: r1234, .. }: Antiscalar,
) -> Scalar {

  let s = l1234*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_contraction_dual_number(
  DualNumber { e1234: l1234, .. }: DualNumber,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Scalar {

  let s = l1234*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_contraction_odd_grade(
  DualNumber { e1234: l1234, .. }: DualNumber,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = -l1234*r423;
  let e2 = -l1234*r431;
  let e3 = -l1234*r412;

  let e321 = l1234*r4;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_contraction_even_grade(
  DualNumber { e1234: l1234, .. }: DualNumber,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = l1234*r1234;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
  }
}

// odd grade

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_contraction_multivector(
  OddGrade {
    e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: OddGrade,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    ..
  }: Multivector,
) -> Multivector {

  let s = l4*r4 + l423*r423 + l431*r431 + l412*r412;

  let e1 = l431*r43 - l412*r42;
  let e2 = l412*r41 - l423*r43;
  let e3 = l423*r42 - l431*r41;

  let e23 = l423*r4;
  let e31 = l431*r4;
  let e12 = l412*r4;

  Multivector {
    s,
    e1, e2, e3,
    e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_contraction_vector(
  OddGrade {
    e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: OddGrade,
  Vector { e4: r4, .. }: Vector,
) -> EvenGrade {

  let s = l4*r4;

  let e23 = l423*r4;
  let e31 = l431*r4;
  let e12 = l412*r4;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_contraction_bivector(
  OddGrade { e423: l423, e431: l431, e412: l412, .. }: OddGrade,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Vector {

  let e1 = l431*r43 - l412*r42;
  let e2 = l412*r41 - l423*r43;
  let e3 = l423*r42 - l431*r41;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_contraction_trivector(
  OddGrade { e423: l423, e431: l431, e412: l412, .. }: OddGrade,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Scalar {

  let s = l423*r423 + l431*r431 + l412*r412;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_contraction_odd_grade(
  OddGrade {
    e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: OddGrade,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> EvenGrade {

  let s = l4*r4 + l423*r423 + l431*r431 + l412*r412;

  let e23 = l423*r4;
  let e31 = l431*r4;
  let e12 = l412*r4;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_contraction_even_grade(
  OddGrade { e423: l423, e431: l431, e412: l412, .. }: OddGrade,
  EvenGrade { e41: r41, e42: r42, e43: r43, .. }: EvenGrade,
) -> Vector {

  let e1 = l431*r43 - l412*r42;
  let e2 = l412*r41 - l423*r43;
  let e3 = l423*r42 - l431*r41;

  Vector { e1, e2, e3, ..zero() }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_weight_contraction_multivector(
  EvenGrade {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: EvenGrade,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = l41*r41 + l42*r42 + l43*r43 + l1234*r1234;

  let e1 = l41*r4 - l1234*r423;
  let e2 = l42*r4 - l1234*r431;
  let e3 = l43*r4 - l1234*r412;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  let e321 = l1234*r4;

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
fn even_grade_weight_contraction_vector(
  EvenGrade {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: EvenGrade,
  Vector { e4: r4, .. }: Vector,
) -> OddGrade {

  let e1 = l41*r4;
  let e2 = l42*r4;
  let e3 = l43*r4;

  let e321 = l1234*r4;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_contraction_bivector(
  EvenGrade {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: EvenGrade,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> EvenGrade {

  let s = l41*r41 + l42*r42 + l43*r43;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_contraction_trivector(
  EvenGrade { e1234: l1234, .. }: EvenGrade,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Vector {

  let e1 = -l1234*r423;
  let e2 = -l1234*r431;
  let e3 = -l1234*r412;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_contraction_antiscalar(
  EvenGrade { e1234: l1234, .. }: EvenGrade,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Scalar {

  let s = l1234*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_contraction_dual_number(
  EvenGrade { e1234: l1234, .. }: EvenGrade,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Scalar {

  let s = l1234*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_contraction_odd_grade(
  EvenGrade {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: EvenGrade,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = l41*r4 - l1234*r423;
  let e2 = l42*r4 - l1234*r431;
  let e3 = l43*r4 - l1234*r412;

  let e321 = l1234*r4;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_contraction_even_grade(
  EvenGrade {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: EvenGrade,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = l41*r41 + l42*r42 + l43*r43 + l1234*r1234;

  let e23 = -l1234*r41;
  let e31 = -l1234*r42;
  let e12 = -l1234*r43;

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
  fn definition() {
    assert_eq!(
      weight_contraction(MULTIVECTOR_A, MULTIVECTOR_B),
      antiwedge(MULTIVECTOR_A, weight_dual(MULTIVECTOR_B))
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
      fn sparse_multivector_a_*() {
        let multivector: Multivector = MULTIVECTOR_A;
        assert_eq!(
          Multivector::from(weight_contraction(multivector, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_contraction(scalar, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_contraction(vector, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_contraction(bivector, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_contraction(trivector, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_contraction(antiscalar, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_contraction(dual_number, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_contraction(odd_grade, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_contraction(even_grade, variant)),
          Multivector::from(weight_contraction(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
