use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_binary_operation, return_scalar_zero_binary},
};

/// a ∨ b<sup>★</sup>
#[inline]
pub fn bulk_contraction<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as BulkContraction<Rhs>>::Output
where
  Lhs: BulkContraction<Rhs>,
{
  a.bulk_contraction(b)
}

/// a ∨ b<sup>★</sup>
pub trait BulkContraction<Rhs> {
  type Output;

  /// a ∨ b<sup>★</sup>
  #[doc(alias = "interior product", alias = "product")]
  fn bulk_contraction(self, b: Rhs) -> Self::Output;
}

impl_binary_operation!(BulkContraction::bulk_contraction {
  Multivector, Multivector => Multivector: multivector_bulk_contraction_multivector;
  Multivector, Scalar => Multivector: multivector_bulk_contraction_scalar;
  Multivector, Vector => Multivector: multivector_bulk_contraction_vector;
  Multivector, Bivector => Multivector: multivector_bulk_contraction_bivector;
  Multivector, Trivector => Multivector: multivector_bulk_contraction_trivector;
  Multivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Multivector, DualNumber => Multivector: multivector_bulk_contraction_dual_number;
  Multivector, OddGrade => Multivector: multivector_bulk_contraction_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_bulk_contraction_even_grade;

  Scalar, Multivector => Scalar: scalar_bulk_contraction_multivector;
  Scalar, Scalar => Scalar: scalar_bulk_contraction_scalar;
  Scalar, Vector => Scalar: return_scalar_zero_binary;
  Scalar, Bivector => Scalar: return_scalar_zero_binary;
  Scalar, Trivector => Scalar: return_scalar_zero_binary;
  Scalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Scalar, DualNumber => Scalar: scalar_bulk_contraction_dual_number;
  Scalar, OddGrade => Scalar: return_scalar_zero_binary;
  Scalar, EvenGrade => Scalar: scalar_bulk_contraction_even_grade;

  Vector, Multivector => Multivector: vector_bulk_contraction_multivector;
  Vector, Scalar => Vector: vector_bulk_contraction_scalar;
  Vector, Vector => Scalar: vector_bulk_contraction_vector;
  Vector, Bivector => Scalar: return_scalar_zero_binary;
  Vector, Trivector => Scalar: return_scalar_zero_binary;
  Vector, Antiscalar => Scalar: return_scalar_zero_binary;
  Vector, DualNumber => Vector: vector_bulk_contraction_dual_number;
  Vector, OddGrade => Scalar: vector_bulk_contraction_odd_grade;
  Vector, EvenGrade => Vector: vector_bulk_contraction_even_grade;

  Bivector, Multivector => Multivector: bivector_bulk_contraction_multivector;
  Bivector, Scalar => Bivector: bivector_bulk_contraction_scalar;
  Bivector, Vector => Vector: bivector_bulk_contraction_vector;
  Bivector, Bivector => Scalar: bivector_bulk_contraction_bivector;
  Bivector, Trivector => Scalar: return_scalar_zero_binary;
  Bivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Bivector, DualNumber => Bivector: bivector_bulk_contraction_dual_number;
  Bivector, OddGrade => Vector: bivector_bulk_contraction_odd_grade;
  Bivector, EvenGrade => EvenGrade: bivector_bulk_contraction_even_grade;

  Trivector, Multivector => Multivector: trivector_bulk_contraction_multivector;
  Trivector, Scalar => Trivector: trivector_bulk_contraction_scalar;
  Trivector, Vector => Bivector: trivector_bulk_contraction_vector;
  Trivector, Bivector => Vector: trivector_bulk_contraction_bivector;
  Trivector, Trivector => Scalar: trivector_bulk_contraction_trivector;
  Trivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Trivector, DualNumber => Trivector: trivector_bulk_contraction_dual_number;
  Trivector, OddGrade => EvenGrade: trivector_bulk_contraction_odd_grade;
  Trivector, EvenGrade => OddGrade: trivector_bulk_contraction_even_grade;

  Antiscalar, Multivector => Multivector: antiscalar_bulk_contraction_multivector;
  Antiscalar, Scalar => Antiscalar: antiscalar_bulk_contraction_scalar;
  Antiscalar, Vector => Trivector: antiscalar_bulk_contraction_vector;
  Antiscalar, Bivector => Bivector: antiscalar_bulk_contraction_bivector;
  Antiscalar, Trivector => Vector: antiscalar_bulk_contraction_trivector;
  Antiscalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Antiscalar, DualNumber => Antiscalar: antiscalar_bulk_contraction_dual_number;
  Antiscalar, OddGrade => OddGrade: antiscalar_bulk_contraction_odd_grade;
  Antiscalar, EvenGrade => EvenGrade: antiscalar_bulk_contraction_even_grade;

  DualNumber, Multivector => Multivector: dual_number_bulk_contraction_multivector;
  DualNumber, Scalar => DualNumber: dual_number_bulk_contraction_scalar;
  DualNumber, Vector => Trivector: dual_number_bulk_contraction_vector;
  DualNumber, Bivector => Bivector: dual_number_bulk_contraction_bivector;
  DualNumber, Trivector => Vector: dual_number_bulk_contraction_trivector;
  DualNumber, Antiscalar => Scalar: return_scalar_zero_binary;
  DualNumber, DualNumber => DualNumber: dual_number_bulk_contraction_dual_number;
  DualNumber, OddGrade => OddGrade: dual_number_bulk_contraction_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_bulk_contraction_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_bulk_contraction_multivector;
  OddGrade, Scalar => OddGrade: odd_grade_bulk_contraction_scalar;
  OddGrade, Vector => EvenGrade: odd_grade_bulk_contraction_vector;
  OddGrade, Bivector => Vector: odd_grade_bulk_contraction_bivector;
  OddGrade, Trivector => Scalar: odd_grade_bulk_contraction_trivector;
  OddGrade, Antiscalar => Scalar: return_scalar_zero_binary;
  OddGrade, DualNumber => OddGrade: odd_grade_bulk_contraction_dual_number;
  OddGrade, OddGrade => EvenGrade: odd_grade_bulk_contraction_odd_grade;
  OddGrade, EvenGrade => OddGrade: odd_grade_bulk_contraction_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_bulk_contraction_multivector;
  EvenGrade, Scalar => EvenGrade: even_grade_bulk_contraction_scalar;
  EvenGrade, Vector => OddGrade: even_grade_bulk_contraction_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_bulk_contraction_bivector;
  EvenGrade, Trivector => Vector: even_grade_bulk_contraction_trivector;
  EvenGrade, Antiscalar => Scalar: return_scalar_zero_binary;
  EvenGrade, DualNumber => EvenGrade: even_grade_bulk_contraction_dual_number;
  EvenGrade, OddGrade => OddGrade: even_grade_bulk_contraction_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_bulk_contraction_even_grade;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_bulk_contraction_multivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let s = ls*rs + l1*r1 + l2*r2 + l3*r3
    + l23*r23 + l31*r31 + l12*r12 + l321*r321;

  let e1 = l1*rs + l31*r3 - l12*r2 - l321*r23;
  let e2 = l2*rs + l12*r1 - l23*r3 - l321*r31;
  let e3 = l3*rs + l23*r2 - l31*r1 - l321*r12;
  let e4 = l4*rs + l423*r23 + l431*r31 + l412*r12
    - l41*r1 - l42*r2 - l43*r3 - l1234*r321;

  let e41 = l41*rs + l412*r2 - l431*r3 - l1234*r23;
  let e42 = l42*rs + l423*r3 - l412*r1 - l1234*r31;
  let e43 = l43*rs + l431*r1 - l423*r2 - l1234*r12;
  let e23 = l23*rs - l321*r1;
  let e31 = l31*rs - l321*r2;
  let e12 = l12*rs - l321*r3;

  let e423 = l423*rs + l1234*r1;
  let e431 = l431*rs + l1234*r2;
  let e412 = l412*rs + l1234*r3;
  let e321 = l321*rs;

  let e1234 = l1234*rs;

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
fn multivector_bulk_contraction_scalar(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Scalar { s: rs }: Scalar,
) -> Multivector {

  let s = ls*rs;

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  let e1234 = l1234*rs;

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
fn multivector_bulk_contraction_vector(
  Multivector {
    e1: l1, e2: l2, e3: l3,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
    ..
  }: Multivector,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Multivector {

  let s = l1*r1 + l2*r2 + l3*r3;

  let e1 = l31*r3 - l12*r2;
  let e2 = l12*r1 - l23*r3;
  let e3 = l23*r2 - l31*r1;
  let e4 = -(l41*r1 + l42*r2 + l43*r3);

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  let e423 = l1234*r1;
  let e431 = l1234*r2;
  let e412 = l1234*r3;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_bulk_contraction_bivector(
  Multivector {
    e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
    ..
  }: Multivector,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Multivector {

  let s = l23*r23 + l31*r31 + l12*r12;

  let e1 = -l321*r23;
  let e2 = -l321*r31;
  let e3 = -l321*r12;
  let e4 = l423*r23 + l431*r31 + l412*r12;

  let e41 = -l1234*r23;
  let e42 = -l1234*r31;
  let e43 = -l1234*r12;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_bulk_contraction_trivector(
  Multivector { e321: l321, e1234: l1234, .. }: Multivector,
  Trivector { e321: r321, .. }: Trivector,
) -> Multivector {

  let s = l321*r321;
  let e4 = -l1234*r321;

  Multivector {
    s,
    e4,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_bulk_contraction_dual_number(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  DualNumber { s: rs, .. }: DualNumber,
) -> Multivector {

  let s = ls*rs;

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  let e1234 = l1234*rs;

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
fn multivector_bulk_contraction_odd_grade(
  Multivector {
    e1: l1, e2: l2, e3: l3,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
    ..
  }: Multivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> Multivector {

  let s = l1*r1 + l2*r2 + l3*r3 + l321*r321;

  let e1 = l31*r3 - l12*r2;
  let e2 = l12*r1 - l23*r3;
  let e3 = l23*r2 - l31*r1;
  let e4 = -(l41*r1 + l42*r2 + l43*r3 + l1234*r321);

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  let e423 = l1234*r1;
  let e431 = l1234*r2;
  let e412 = l1234*r3;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_bulk_contraction_even_grade(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> Multivector {

  let s = ls*rs + l23*r23 + l31*r31 + l12*r12;

  let e1 = l1*rs - l321*r23;
  let e2 = l2*rs - l321*r31;
  let e3 = l3*rs - l321*r12;
  let e4 = l4*rs + l423*r23 + l431*r31 + l412*r12;

  let e41 = l41*rs - l1234*r23;
  let e42 = l42*rs - l1234*r31;
  let e43 = l43*rs - l1234*r12;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  let e1234 = l1234*rs;

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
fn scalar_bulk_contraction_multivector(
  Scalar { s: ls }: Scalar,
  Multivector { s: rs, .. }: Multivector,
) -> Scalar {

  let s = ls*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_contraction_scalar(
  Scalar { s: ls }: Scalar,
  Scalar { s: rs }: Scalar,
) -> Scalar {

  let s = ls*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_contraction_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { s: rs, .. }: DualNumber,
) -> Scalar {

  let s = ls*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_contraction_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade { s: rs, .. }: EvenGrade,
) -> Scalar {

  let s = ls*rs;

  Scalar { s }
}

// vector

#[rustfmt::skip]
#[inline]
fn vector_bulk_contraction_multivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4, .. }: Vector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    ..
  }: Multivector,
) -> Multivector {

  let s = l1*r1 + l2*r2 + l3*r3;

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  Multivector {
    s,
    e1, e2, e3, e4,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_contraction_scalar(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4, .. }: Vector,
  Scalar { s: rs }: Scalar,
) -> Vector {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_contraction_vector(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Scalar {

  let s = l1*r1 + l2*r2 + l3*r3;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_contraction_dual_number(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4, .. }: Vector,
  DualNumber { s: rs, .. }: DualNumber,
) -> Vector {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_contraction_odd_grade(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  OddGrade { e1: r1, e2: r2, e3: r3, .. }: OddGrade,
) -> Scalar {

  let s = l1*r1 + l2*r2 + l3*r3;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_contraction_even_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4, .. }: Vector,
  EvenGrade { s: rs, .. }: EvenGrade,
) -> Vector {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  Vector { e1, e2, e3, e4 }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_bulk_contraction_multivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    ..
  }: Multivector,
) -> Multivector {

  let s = l23*r23 + l31*r31 + l12*r12;

  let e1 = l31*r3 - l12*r2;
  let e2 = l12*r1 - l23*r3;
  let e3 = l23*r2 - l31*r1;
  let e4 = -(l41*r1 + l42*r2 + l43*r3);

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_contraction_scalar(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Scalar { s: rs }: Scalar,
) -> Bivector {

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_contraction_vector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Vector {

  let e1 = l31*r3 - l12*r2;
  let e2 = l12*r1 - l23*r3;
  let e3 = l23*r2 - l31*r1;
  let e4 = -(l41*r1 + l42*r2 + l43*r3);

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_contraction_bivector(
  Bivector { e23: l23, e31: l31, e12: l12, .. }: Bivector,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Scalar {

  let s = l23*r23 + l31*r31 + l12*r12;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_contraction_dual_number(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  DualNumber { s: rs, .. }: DualNumber,
) -> Bivector {

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_contraction_odd_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  OddGrade { e1: r1, e2: r2, e3: r3, .. }: OddGrade,
) -> Vector {

  let e1 = l31*r3 - l12*r2;
  let e2 = l12*r1 - l23*r3;
  let e3 = l23*r2 - l31*r1;
  let e4 = -(l41*r1 + l42*r2 + l43*r3);

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_contraction_even_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = l23*r23 + l31*r31 + l12*r12;

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_bulk_contraction_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let s = l321*r321;

  let e1 = -l321*r23;
  let e2 = -l321*r31;
  let e3 = -l321*r12;
  let e4 = l423*r23 + l431*r31 + l412*r12;

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_contraction_scalar(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Scalar { s: rs }: Scalar,
) -> Trivector {

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_contraction_vector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Bivector {

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_contraction_bivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Vector {

  let e1 = -l321*r23;
  let e2 = -l321*r31;
  let e3 = -l321*r12;
  let e4 = l423*r23 + l431*r31 + l412*r12;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_contraction_trivector(
  Trivector { e321: l321, .. }: Trivector,
  Trivector { e321: r321, .. }: Trivector,
) -> Scalar {

  let s = l321*r321;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_contraction_dual_number(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  DualNumber { s: rs, .. }: DualNumber,
) -> Trivector {

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_contraction_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> EvenGrade {

  let s = l321*r321;

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_contraction_even_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> OddGrade {

  let e1 = -l321*r23;
  let e2 = -l321*r31;
  let e3 = -l321*r12;
  let e4 = l423*r23 + l431*r31 + l412*r12;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// antiscalar

#[rustfmt::skip]
#[inline]
fn antiscalar_bulk_contraction_multivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e4 = -l1234*r321;

  let e41 = -l1234*r23;
  let e42 = -l1234*r31;
  let e43 = -l1234*r12;

  let e423 =l1234*r1;
  let e431 =l1234*r2;
  let e412 =l1234*r3;

  let e1234 = l1234*rs;

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
fn antiscalar_bulk_contraction_scalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Scalar { s: rs }: Scalar,
) -> Antiscalar {

  let e1234 = l1234*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_bulk_contraction_vector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Trivector {

  let e423 =l1234*r1;
  let e431 =l1234*r2;
  let e412 =l1234*r3;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_bulk_contraction_bivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Bivector {

  let e41 = -l1234*r23;
  let e42 = -l1234*r31;
  let e43 = -l1234*r12;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_bulk_contraction_trivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Trivector { e321: r321, .. }: Trivector,
) -> Vector {

  let e4 = -l1234*r321;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_bulk_contraction_dual_number(
  Antiscalar { e1234: l1234 }: Antiscalar,
  DualNumber { s: rs, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = l1234*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_bulk_contraction_odd_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e4 = -l1234*r321;

  let e423 =l1234*r1;
  let e431 =l1234*r2;
  let e412 =l1234*r3;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_bulk_contraction_even_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let e41 = -l1234*r23;
  let e42 = -l1234*r31;
  let e43 = -l1234*r12;

  let e1234 = l1234*rs;

  EvenGrade {
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

// dual number

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_contraction_multivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let s = ls*rs;

  let e4 = -l1234*r321;

  let e41 = -l1234*r23;
  let e42 = -l1234*r31;
  let e43 = -l1234*r12;

  let e423 =l1234*r1;
  let e431 =l1234*r2;
  let e412 =l1234*r3;

  let e1234 = l1234*rs;

  Multivector {
    s,
    e4,
    e41, e42, e43,
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_contraction_scalar(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Scalar { s: rs }: Scalar,
) -> DualNumber {

  let s = ls*rs;
  let e1234 = l1234*rs;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_contraction_vector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Trivector {

  let e423 =l1234*r1;
  let e431 =l1234*r2;
  let e412 =l1234*r3;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_contraction_bivector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Bivector {

  let e41 = -l1234*r23;
  let e42 = -l1234*r31;
  let e43 = -l1234*r12;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_contraction_trivector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Trivector { e321: r321, .. }: Trivector,
) -> Vector {

  let e4 = -l1234*r321;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_contraction_dual_number(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  DualNumber { s: rs, .. }: DualNumber,
) -> DualNumber {

  let s = ls*rs;
  let e1234 = l1234*rs;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_contraction_odd_grade(
  DualNumber { e1234: l1234, .. }: DualNumber,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e4 = -l1234*r321;

  let e423 =l1234*r1;
  let e431 =l1234*r2;
  let e412 =l1234*r3;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_contraction_even_grade(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*rs;

  let e41 = -l1234*r23;
  let e42 = -l1234*r31;
  let e43 = -l1234*r12;

  let e1234 = l1234*rs;

  EvenGrade {
    s,
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

// odd grade

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_contraction_multivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let s = l1*r1 + l2*r2 + l3*r3 + l321*r321;

  let e1 = l1*rs - l321*r23;
  let e2 = l2*rs - l321*r31;
  let e3 = l3*rs - l321*r12;
  let e4 = l4*rs + l423*r23 + l431*r31 + l412*r12;

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_contraction_scalar(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Scalar { s: rs }: Scalar,
) -> OddGrade {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_contraction_vector(
  OddGrade {
    e1: l1, e2: l2, e3: l3,
    e423: l423, e431: l431, e412: l412, e321: l321,
    ..
  }: OddGrade,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> EvenGrade {

  let s = l1*r1 + l2*r2 + l3*r3;

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_contraction_bivector(
  OddGrade { e423: l423, e431: l431, e412: l412, e321: l321, .. }: OddGrade,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Vector {

  let e1 = -l321*r23;
  let e2 = -l321*r31;
  let e3 = -l321*r12;
  let e4 = l423*r23 + l431*r31 + l412*r12;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_contraction_trivector(
  OddGrade { e321: l321, .. }: OddGrade,
  Trivector { e321: r321, .. }: Trivector,
) -> Scalar {

  let s = l321*r321;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_contraction_dual_number(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  DualNumber { s: rs, .. }: DualNumber,
) -> OddGrade {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_contraction_odd_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3,
    e423: l423, e431: l431, e412: l412, e321: l321,
    ..
  }: OddGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> EvenGrade {

  let s = l1*r1 + l2*r2 + l3*r3 + l321*r321;

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_contraction_even_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*rs - l321*r23;
  let e2 = l2*rs - l321*r31;
  let e3 = l3*rs - l321*r12;
  let e4 = l4*rs + l423*r23 + l431*r31 + l412*r12;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_contraction_multivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let s = ls*rs + l23*r23 + l31*r31 + l12*r12;

  let e1 = l31*r3 - l12*r2;
  let e2 = l12*r1 - l23*r3;
  let e3 = l23*r2 - l31*r1;
  let e4 = -(l41*r1 + l42*r2 + l43*r3 + l1234*r321);

  let e41 = l41*rs - l1234*r23;
  let e42 = l42*rs - l1234*r31;
  let e43 = l43*rs - l1234*r12;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e423 = l1234*r1;
  let e431 = l1234*r2;
  let e412 = l1234*r3;

  let e1234 = l1234*rs;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_contraction_scalar(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Scalar { s: rs }: Scalar,
) -> EvenGrade {

  let s = ls*rs;

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e1234 = l1234*rs;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_contraction_vector(
  EvenGrade {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
    ..
  }: EvenGrade,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> OddGrade {

  let e1 = l31*r3 - l12*r2;
  let e2 = l12*r1 - l23*r3;
  let e3 = l23*r2 - l31*r1;
  let e4 = -(l41*r1 + l42*r2 + l43*r3);

  let e423 = l1234*r1;
  let e431 = l1234*r2;
  let e412 = l1234*r3;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_contraction_bivector(
  EvenGrade {
    e23: l23, e31: l31, e12: l12,
    e1234: l1234,
    ..
  }: EvenGrade,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> EvenGrade {

  let s = l23*r23 + l31*r31 + l12*r12;

  let e41 = -l1234*r23;
  let e42 = -l1234*r31;
  let e43 = -l1234*r12;

  EvenGrade {
    s,
    e41, e42, e43,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_contraction_trivector(
  EvenGrade { e1234: l1234, .. }: EvenGrade,
  Trivector { e321: r321, .. }: Trivector,
) -> Vector {

  let e4 = -l1234*r321;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_contraction_dual_number(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  DualNumber { s: rs, .. }: DualNumber,
) -> EvenGrade {

  let s = ls*rs;

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e1234 = l1234*rs;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_contraction_odd_grade(
  EvenGrade {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
    ..
  }: EvenGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = l31*r3 - l12*r2;
  let e2 = l12*r1 - l23*r3;
  let e3 = l23*r2 - l31*r1;
  let e4 = - l41*r1 - l42*r2 - l43*r3 - l1234*r321;

  let e423 = l1234*r1;
  let e431 = l1234*r2;
  let e412 = l1234*r3;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_contraction_even_grade(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*rs + l23*r23 + l31*r31 + l12*r12;

  let e41 = l41*rs - l1234*r23;
  let e42 = l42*rs - l1234*r31;
  let e43 = l43*rs - l1234*r12;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e1234 = l1234*rs;

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
      bulk_contraction(MULTIVECTOR_A, MULTIVECTOR_B),
      antiwedge(MULTIVECTOR_A, bulk_dual(MULTIVECTOR_B))
    );
  }

  #[test]
  fn distribution_over_wedge_product() {
    assert_eq!(
      bulk_contraction(MULTIVECTOR_A, wedge(MULTIVECTOR_B, MULTIVECTOR_C)),
      bulk_contraction(
        bulk_contraction(MULTIVECTOR_A, MULTIVECTOR_B),
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
          Multivector::from(bulk_contraction(multivector, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_contraction(scalar, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_contraction(vector, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_contraction(bivector, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_contraction(trivector, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_contraction(antiscalar, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_contraction(dual_number, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_contraction(odd_grade, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_contraction(even_grade, variant)),
          Multivector::from(bulk_contraction(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
