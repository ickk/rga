use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_binary_operation, return_scalar_zero_binary},
};

/// a ∧ b<sup>☆</sup>
#[inline]
pub fn weight_expansion<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as WeightExpansion<Rhs>>::Output
where
  Lhs: WeightExpansion<Rhs>,
{
  a.weight_expansion(b)
}

/// a ∧ b<sup>☆</sup>
pub trait WeightExpansion<Rhs> {
  type Output;

  /// a ∧ b<sup>☆</sup>
  #[doc(alias = "interior product", alias = "product")]
  fn weight_expansion(self, b: Rhs) -> Self::Output;
}

impl_binary_operation!(WeightExpansion::weight_expansion {
  Multivector, Multivector => Multivector: multivector_weight_expansion_multivector;
  Multivector, Scalar => Scalar: return_scalar_zero_binary;
  Multivector, Vector => Multivector: multivector_weight_expansion_vector;
  Multivector, Bivector => Multivector: multivector_weight_expansion_bivector;
  Multivector, Trivector => Multivector: multivector_weight_expansion_trivector;
  Multivector, Antiscalar => Multivector: multivector_weight_expansion_antiscalar;
  Multivector, DualNumber => Multivector: multivector_weight_expansion_dual_number;
  Multivector, OddGrade => Multivector: multivector_weight_expansion_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_weight_expansion_even_grade;

  Scalar, Multivector => Multivector: scalar_weight_expansion_multivector;
  Scalar, Scalar => Scalar: return_scalar_zero_binary;
  Scalar, Vector => Trivector: scalar_weight_expansion_vector;
  Scalar, Bivector => Bivector: scalar_weight_expansion_bivector;
  Scalar, Trivector => Vector: scalar_weight_expansion_trivector;
  Scalar, Antiscalar => Scalar: scalar_weight_expansion_antiscalar;
  Scalar, DualNumber => Scalar: scalar_weight_expansion_dual_number;
  Scalar, OddGrade => OddGrade: scalar_weight_expansion_odd_grade;
  Scalar, EvenGrade => EvenGrade: scalar_weight_expansion_even_grade;

  Vector, Multivector => Multivector: vector_weight_expansion_multivector;
  Vector, Scalar => Scalar: return_scalar_zero_binary;
  Vector, Vector => Antiscalar: vector_weight_expansion_vector;
  Vector, Bivector => Trivector: vector_weight_expansion_bivector;
  Vector, Trivector => Bivector: vector_weight_expansion_trivector;
  Vector, Antiscalar => Vector: vector_weight_expansion_antiscalar;
  Vector, DualNumber => Vector: vector_weight_expansion_dual_number;
  Vector, OddGrade => EvenGrade: vector_weight_expansion_odd_grade;
  Vector, EvenGrade => OddGrade: vector_weight_expansion_even_grade;

  Bivector, Multivector => Multivector: bivector_weight_expansion_multivector;
  Bivector, Scalar => Scalar: return_scalar_zero_binary;
  Bivector, Vector => Scalar: return_scalar_zero_binary;
  Bivector, Bivector => Antiscalar: bivector_weight_expansion_bivector;
  Bivector, Trivector => Trivector: bivector_weight_expansion_trivector;
  Bivector, Antiscalar => Bivector: bivector_weight_expansion_antiscalar;
  Bivector, DualNumber => Bivector: bivector_weight_expansion_dual_number;
  Bivector, OddGrade => Trivector: bivector_weight_expansion_odd_grade;
  Bivector, EvenGrade => EvenGrade: bivector_weight_expansion_even_grade;

  Trivector, Multivector => Multivector: trivector_weight_expansion_multivector;
  Trivector, Scalar => Scalar: return_scalar_zero_binary;
  Trivector, Vector => Scalar: return_scalar_zero_binary;
  Trivector, Bivector => Scalar: return_scalar_zero_binary;
  Trivector, Trivector => Antiscalar: trivector_weight_expansion_trivector;
  Trivector, Antiscalar => Trivector: trivector_weight_expansion_antiscalar;
  Trivector, DualNumber => Trivector: trivector_weight_expansion_dual_number;
  Trivector, OddGrade => Antiscalar: trivector_weight_expansion_odd_grade;
  Trivector, EvenGrade => Trivector: trivector_weight_expansion_even_grade;

  Antiscalar, Multivector => Antiscalar: antiscalar_weight_expansion_multivector;
  Antiscalar, Scalar => Scalar: return_scalar_zero_binary;
  Antiscalar, Vector => Scalar: return_scalar_zero_binary;
  Antiscalar, Bivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Trivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Antiscalar => Antiscalar: antiscalar_weight_expansion_antiscalar;
  Antiscalar, DualNumber => Antiscalar: antiscalar_weight_expansion_dual_number;
  Antiscalar, OddGrade => Scalar: return_scalar_zero_binary;
  Antiscalar, EvenGrade => Antiscalar: antiscalar_weight_expansion_even_grade;

  DualNumber, Multivector => Multivector: dual_number_weight_expansion_multivector;
  DualNumber, Scalar => Scalar: return_scalar_zero_binary;
  DualNumber, Vector => Trivector: dual_number_weight_expansion_vector;
  DualNumber, Bivector => Bivector: dual_number_weight_expansion_bivector;
  DualNumber, Trivector => Vector: dual_number_weight_expansion_trivector;
  DualNumber, Antiscalar => DualNumber: dual_number_weight_expansion_antiscalar;
  DualNumber, DualNumber => DualNumber: dual_number_weight_expansion_dual_number;
  DualNumber, OddGrade => OddGrade: dual_number_weight_expansion_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_weight_expansion_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_weight_expansion_multivector;
  OddGrade, Scalar => Scalar: return_scalar_zero_binary;
  OddGrade, Vector => Antiscalar: odd_grade_weight_expansion_vector;
  OddGrade, Bivector => Trivector: odd_grade_weight_expansion_bivector;
  OddGrade, Trivector => EvenGrade: odd_grade_weight_expansion_trivector;
  OddGrade, Antiscalar => OddGrade: odd_grade_weight_expansion_antiscalar;
  OddGrade, DualNumber => OddGrade: odd_grade_weight_expansion_dual_number;
  OddGrade, OddGrade => EvenGrade: odd_grade_weight_expansion_odd_grade;
  OddGrade, EvenGrade => OddGrade: odd_grade_weight_expansion_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_weight_expansion_multivector;
  EvenGrade, Scalar => Scalar: return_scalar_zero_binary;
  EvenGrade, Vector => Trivector: even_grade_weight_expansion_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_weight_expansion_bivector;
  EvenGrade, Trivector => OddGrade: even_grade_weight_expansion_trivector;
  EvenGrade, Antiscalar => EvenGrade: even_grade_weight_expansion_antiscalar;
  EvenGrade, DualNumber => EvenGrade: even_grade_weight_expansion_dual_number;
  EvenGrade, OddGrade => OddGrade: even_grade_weight_expansion_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_weight_expansion_even_grade;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_weight_expansion_multivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = ls*r1234;

  let e1 = -ls*r423 + l1*r1234;
  let e2 = -ls*r431 + l2*r1234;
  let e3 = -ls*r412 + l3*r1234;
  let e4 = l4*r1234;

  let e41 = -l4*r423 + l41*r1234;
  let e42 = -l4*r431 + l42*r1234;
  let e43 = -l4*r412 + l43*r1234;
  let e23 = -ls*r41 - l2*r412 + l3*r431 + l23*r1234;
  let e31 = -ls*r42 - l3*r423 + l1*r412 + l31*r1234;
  let e12 = -ls*r43 - l1*r431 + l2*r423 + l12*r1234;

  let e423 = -l4*r41 - l42*r412 + l43*r431 + l423*r1234;
  let e431 = -l4*r42 - l43*r423 + l41*r412 + l431*r1234;
  let e412 = -l4*r43 - l41*r431 + l42*r423 + l412*r1234;
  let e321 = ls*r4 + l1*r41 + l2*r42 + l3*r43
    + l23*r423 + l31*r431 + l12*r412 + l321*r1234;

  let e1234 = l4*r4 + l41*r41 + l42*r42 + l43*r43
    + l423*r423 + l431*r431 + l412*r412 + l1234*r1234;

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
fn multivector_weight_expansion_vector(
  Multivector {
    s: ls,
    e4: l4,
    ..
  }: Multivector,
  Vector { e4: r4, .. }: Vector,
) -> Multivector {

  let e321 = ls*r4;
  let e1234 = l4*r4;

  Multivector {
    e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_weight_expansion_bivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43,
    ..
  }: Multivector,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Multivector {

  let e23 = -ls*r41;
  let e31 = -ls*r42;
  let e12 = -ls*r43;

  let e423 = -l4*r41;
  let e431 = -l4*r42;
  let e412 = -l4*r43;
  let e321 = l1*r41 + l2*r42 + l3*r43;

  let e1234 = l41*r41 + l42*r42 + l43*r43;

  Multivector {
    e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_weight_expansion_trivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412,
    ..
  }: Multivector,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Multivector {

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = -l2*r412 + l3*r431;
  let e31 = -l3*r423 + l1*r412;
  let e12 = -l1*r431 + l2*r423;

  let e423 = -l42*r412 + l43*r431;
  let e431 = -l43*r423 + l41*r412;
  let e412 = -l41*r431 + l42*r423;
  let e321 = l23*r423 + l31*r431 + l12*r412;

  let e1234 = l423*r423 + l431*r431 + l412*r412;

  Multivector {
    e1, e2, e3,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_weight_expansion_antiscalar(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Multivector {

  let s = ls*r1234;

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  let e1234 = l1234*r1234;

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
fn multivector_weight_expansion_dual_number(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Multivector {

  let s = ls*r1234;

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  let e1234 = l1234*r1234;

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
fn multivector_weight_expansion_odd_grade(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412,
    ..
  }: Multivector,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> Multivector {

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = -l2*r412 + l3*r431;
  let e31 = -l3*r423 + l1*r412;
  let e12 = -l1*r431 + l2*r423;

  let e423 = -l42*r412 + l43*r431;
  let e431 = -l43*r423 + l41*r412;
  let e412 = -l41*r431 + l42*r423;
  let e321 = ls*r4 + l23*r423 + l31*r431 + l12*r412;

  let e1234 = l4*r4 + l423*r423 + l431*r431 + l412*r412;

  Multivector {
    e1, e2, e3,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_weight_expansion_even_grade(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> Multivector {

  let s = ls*r1234;

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = -ls*r41 + l23*r1234;
  let e31 = -ls*r42 + l31*r1234;
  let e12 = -ls*r43 + l12*r1234;

  let e423 = -l4*r41 + l423*r1234;
  let e431 = -l4*r42 + l431*r1234;
  let e412 = -l4*r43 + l412*r1234;
  let e321 = l1*r41 + l2*r42 + l3*r43 + l321*r1234;

  let e1234 = l41*r41 + l42*r42 + l43*r43 + l1234*r1234;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }
}

// scalar

#[rustfmt::skip]
#[inline]
fn scalar_weight_expansion_multivector(
  Scalar { s: ls }: Scalar,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = ls*r1234;

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e23 = -ls*r41;
  let e31 = -ls*r42;
  let e12 = -ls*r43;

  let e321 = ls*r4;

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
fn scalar_weight_expansion_vector(
  Scalar { s: ls }: Scalar,
  Vector { e4: r4, .. }: Vector,
) -> Trivector {

  let e321 = ls*r4;

  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_weight_expansion_bivector(
  Scalar { s: ls }: Scalar,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Bivector {

  let e23 = -ls*r41;
  let e31 = -ls*r42;
  let e12 = -ls*r43;

  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_weight_expansion_trivector(
  Scalar { s: ls }: Scalar,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Vector {

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_weight_expansion_antiscalar(
  Scalar { s: ls }: Scalar,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Scalar {

  let s = ls*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_weight_expansion_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Scalar {

  let s = ls*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_weight_expansion_odd_grade(
  Scalar { s: ls }: Scalar,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e321 = ls*r4;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_weight_expansion_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*r1234;

  let e23 = -ls*r41;
  let e31 = -ls*r42;
  let e12 = -ls*r43;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
  }
}

// vector

#[rustfmt::skip]
#[inline]
fn vector_weight_expansion_multivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = -l2*r412 + l3*r431;
  let e31 = -l3*r423 + l1*r412;
  let e12 = -l1*r431 + l2*r423;

  let e423 = -l4*r41;
  let e431 = -l4*r42;
  let e412 = -l4*r43;
  let e321 = l1*r41 + l2*r42 + l3*r43;

  let e1234 = l4*r4;

  Multivector {
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_expansion_vector(
  Vector { e4: l4, .. }: Vector,
  Vector { e4: r4, .. }: Vector,
) -> Antiscalar {

  let e1234 = l4*r4;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_expansion_bivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Trivector {

  let e423 = -l4*r41;
  let e431 = -l4*r42;
  let e412 = -l4*r43;
  let e321 = l1*r41 + l2*r42 + l3*r43;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_expansion_trivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Bivector {

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = -l2*r412 + l3*r431;
  let e31 = -l3*r423 + l1*r412;
  let e12 = -l1*r431 + l2*r423;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_expansion_antiscalar(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Vector {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_expansion_dual_number(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Vector {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_expansion_odd_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> EvenGrade {

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = -l2*r412 + l3*r431;
  let e31 = -l3*r423 + l1*r412;
  let e12 = -l1*r431 + l2*r423;

  let e1234 = l4*r4;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_weight_expansion_even_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e423 = -l4*r41;
  let e431 = -l4*r42;
  let e412 = -l4*r43;
  let e321 = l1*r41 + l2*r42 + l3*r43;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_weight_expansion_multivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Multivector {
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  let e423 = -l42*r412 + l43*r431;
  let e431 = -l43*r423 + l41*r412;
  let e412 = -l41*r431 + l42*r423;
  let e321 = l23*r423 + l31*r431 + l12*r412;

  let e1234 = l41*r41 + l42*r42 + l43*r43;

  Multivector {
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_expansion_bivector(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Antiscalar {

  let e1234 = l41*r41 + l42*r42 + l43*r43;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_expansion_trivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Trivector {

  let e423 = -l42*r412 + l43*r431;
  let e431 = -l43*r423 + l41*r412;
  let e412 = -l41*r431 + l42*r423;
  let e321 = l23*r423 + l31*r431 + l12*r412;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_expansion_antiscalar(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Bivector {

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_expansion_dual_number(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Bivector {

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_expansion_odd_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  OddGrade { e423: r423, e431: r431, e412: r412, .. }: OddGrade,
) -> Trivector {

  let e423 = -l42*r412 + l43*r431;
  let e431 = -l43*r423 + l41*r412;
  let e412 = -l41*r431 + l42*r423;
  let e321 = l23*r423 + l31*r431 + l12*r412;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn bivector_weight_expansion_even_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  let e1234 = l41*r41 + l42*r42 + l43*r43;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_weight_expansion_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Multivector {
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  let e1234 = l423*r423 + l431*r431 + l412*r412;

  Multivector {
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_expansion_trivector(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Antiscalar {

  let e1234 = l423*r423 + l431*r431 + l412*r412;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_expansion_antiscalar(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Trivector {

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_expansion_dual_number(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Trivector {

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_expansion_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  OddGrade { e423: r423, e431: r431, e412: r412, .. }: OddGrade,
) -> Antiscalar {

  let e1234 = l423*r423 + l431*r431 + l412*r412;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn trivector_weight_expansion_even_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  EvenGrade { e1234: r1234, .. }: EvenGrade,
) -> Trivector {

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  Trivector { e423, e431, e412, e321 }
}

// antiscalar

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_expansion_multivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Multivector { e1234: r1234, .. }: Multivector,
) -> Antiscalar {

  let e1234 = l1234*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_expansion_antiscalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Antiscalar { e1234: r1234, .. }: Antiscalar,
) -> Antiscalar {

  let e1234 = l1234*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_expansion_dual_number(
  Antiscalar { e1234: l1234 }: Antiscalar,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = l1234*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_weight_expansion_even_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  EvenGrade { e1234: r1234, .. }: EvenGrade,
) -> Antiscalar {

  let e1234 = l1234*r1234;

  Antiscalar { e1234 }
}

// dual number

#[rustfmt::skip]
#[inline]
fn dual_number_weight_expansion_multivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = ls*r1234;

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e23 = -ls*r41;
  let e31 = -ls*r42;
  let e12 = -ls*r43;

  let e321 = ls*r4;

  let e1234 = l1234*r1234;

  Multivector {
    s,
    e1, e2, e3,
    e23, e31, e12,
    e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_expansion_vector(
  DualNumber { s: ls, .. }: DualNumber,
  Vector { e4: r4, .. }: Vector,
) -> Trivector {

  let e321 = ls*r4;

  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_expansion_bivector(
  DualNumber { s: ls, .. }: DualNumber,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Bivector {

  let e23 = -ls*r41;
  let e31 = -ls*r42;
  let e12 = -ls*r43;

  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_expansion_trivector(
  DualNumber { s: ls, .. }: DualNumber,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Vector {

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_expansion_antiscalar(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> DualNumber {

  let s = ls*r1234;
  let e1234 = l1234*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_expansion_dual_number(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> DualNumber {

  let s = ls*r1234;
  let e1234 = l1234*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_expansion_odd_grade(
  DualNumber { s: ls, .. }: DualNumber,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e321 = ls*r4;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_weight_expansion_even_grade(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*r1234;

  let e23 = -ls*r41;
  let e31 = -ls*r42;
  let e12 = -ls*r43;

  let e1234 = l1234*r1234;

  EvenGrade {
    s,
    e23, e31, e12,
    e1234,
    ..zero()
  }
}

// odd grade

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_expansion_multivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = -l2*r412 + l3*r431;
  let e31 = -l3*r423 + l1*r412;
  let e12 = -l1*r431 + l2*r423;

  let e423 = -l4*r41 + l423*r1234;
  let e431 = -l4*r42 + l431*r1234;
  let e412 = -l4*r43 + l412*r1234;
  let e321 = l1*r41 + l2*r42 + l3*r43 + l321*r1234;

  let e1234 = l4*r4 + l423*r423 + l431*r431 + l412*r412;

  Multivector {
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_expansion_vector(
  OddGrade { e4: l4, .. }: OddGrade,
  Vector { e4: r4, .. }: Vector,
) -> Antiscalar {

  let e1234 = l4*r4;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_expansion_bivector(
  OddGrade { e1: l1, e2: l2, e3: l3, e4: l4, .. }: OddGrade,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Trivector {

  let e423 = -l4*r41;
  let e431 = -l4*r42;
  let e412 = -l4*r43;
  let e321 = l1*r41 + l2*r42 + l3*r43;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_expansion_trivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: OddGrade,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> EvenGrade {

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = -l2*r412 + l3*r431;
  let e31 = -l3*r423 + l1*r412;
  let e12 = -l1*r431 + l2*r423;

  let e1234 = l423*r423 + l431*r431 + l412*r412;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_expansion_antiscalar(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> OddGrade {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_expansion_dual_number(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> OddGrade {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_expansion_odd_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: OddGrade,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> EvenGrade {

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = -l2*r412 + l3*r431;
  let e31 = -l3*r423 + l1*r412;
  let e12 = -l1*r431 + l2*r423;

  let e1234 = l4*r4 + l423*r423 + l431*r431 + l412*r412;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_weight_expansion_even_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e423 = -l4*r41 + l423*r1234;
  let e431 = -l4*r42 + l431*r1234;
  let e412 = -l4*r43 + l412*r1234;
  let e321 = l1*r41 + l2*r42 + l3*r43 + l321*r1234;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_weight_expansion_multivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = ls*r1234;

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = -ls*r41 + l23*r1234;
  let e31 = -ls*r42 + l31*r1234;
  let e12 = -ls*r43 + l12*r1234;

  let e423 = -l42*r412 + l43*r431;
  let e431 = -l43*r423 + l41*r412;
  let e412 = -l41*r431 + l42*r423;
  let e321 = ls*r4 + l23*r423 + l31*r431 + l12*r412;

  let e1234 = l41*r41 + l42*r42 + l43*r43 + l1234*r1234;

  Multivector {
    s,
    e1, e2, e3,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_expansion_vector(
  EvenGrade { s: ls, .. }: EvenGrade,
  Vector { e4: r4, .. }: Vector,
) -> Trivector {

  let e321 = ls*r4;

  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_expansion_bivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43,
    ..
  }: EvenGrade,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> EvenGrade {

  let e23 = -ls*r41;
  let e31 = -ls*r42;
  let e12 = -ls*r43;

  let e1234 = l41*r41 + l42*r42 + l43*r43;

  EvenGrade {
    e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_expansion_trivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> OddGrade {

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e423 = -l42*r412 + l43*r431;
  let e431 = -l43*r423 + l41*r412;
  let e412 = -l41*r431 + l42*r423;
  let e321 = l23*r423 + l31*r431 + l12*r412;

  OddGrade {
    e1, e2, e3,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_expansion_antiscalar(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> EvenGrade {

  let s = ls*r1234;

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  let e1234 = l1234*r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_expansion_dual_number(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> EvenGrade {

  let s = ls*r1234;

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  let e1234 = l1234*r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_expansion_odd_grade(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = -ls*r423;
  let e2 = -ls*r431;
  let e3 = -ls*r412;

  let e423 = -l42*r412 + l43*r431;
  let e431 = -l43*r423 + l41*r412;
  let e412 = -l41*r431 + l42*r423;
  let e321 = ls*r4 + l23*r423 + l31*r431 + l12*r412;

  OddGrade {
    e1, e2, e3,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_weight_expansion_even_grade(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*r1234;

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = -ls*r41 + l23*r1234;
  let e31 = -ls*r42 + l31*r1234;
  let e12 = -ls*r43 + l12*r1234;

  let e1234 = l41*r41 + l42*r42 + l43*r43 + l1234*r1234;

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
    assert_eq!(
      weight_expansion(MULTIVECTOR_A, MULTIVECTOR_B),
      wedge(MULTIVECTOR_A, weight_dual(MULTIVECTOR_B))
    );
  }

  #[test]
  fn distribution_over_antiwedge_product() {
    assert_eq!(
      weight_expansion(MULTIVECTOR_A, antiwedge(MULTIVECTOR_B, MULTIVECTOR_C)),
      weight_expansion(
        weight_expansion(MULTIVECTOR_A, MULTIVECTOR_B),
        MULTIVECTOR_C
      )
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
          Multivector::from(weight_expansion(multivector, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_expansion(scalar, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_expansion(vector, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_expansion(bivector, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_expansion(trivector, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_expansion(antiscalar, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_expansion(dual_number, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_expansion(odd_grade, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(weight_expansion(even_grade, variant)),
          Multivector::from(weight_expansion(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
