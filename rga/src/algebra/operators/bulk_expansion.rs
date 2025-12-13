use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_binary_operation, return_scalar_zero_binary},
};

/// a ∧ b<sup>★</sup>
#[inline]
pub fn bulk_expansion<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as BulkExpansion<Rhs>>::Output
where
  Lhs: BulkExpansion<Rhs>,
{
  a.bulk_expansion(b)
}

/// a ∧ b<sup>★</sup>
pub trait BulkExpansion<Rhs> {
  type Output;

  /// a ∧ b<sup>★</sup>
  #[doc(alias = "interior product", alias = "product")]
  fn bulk_expansion(self, b: Rhs) -> Self::Output;
}

impl_binary_operation!(BulkExpansion::bulk_expansion {
  Multivector, Multivector => Multivector: multivector_bulk_expansion_multivector;
  Multivector, Scalar => Antiscalar: multivector_bulk_expansion_scalar;
  Multivector, Vector => Multivector: multivector_bulk_expansion_vector;
  Multivector, Bivector => Multivector: multivector_bulk_expansion_bivector;
  Multivector, Trivector => Multivector: multivector_bulk_expansion_trivector;
  Multivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Multivector, DualNumber => Antiscalar: multivector_bulk_expansion_dual_number;
  Multivector, OddGrade => Multivector: multivector_bulk_expansion_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_bulk_expansion_even_grade;

  Scalar, Multivector => Multivector: scalar_bulk_expansion_multivector;
  Scalar, Scalar => Antiscalar: scalar_bulk_expansion_scalar;
  Scalar, Vector => Trivector: scalar_bulk_expansion_vector;
  Scalar, Bivector => Bivector: scalar_bulk_expansion_bivector;
  Scalar, Trivector => Vector: scalar_bulk_expansion_trivector;
  Scalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Scalar, DualNumber => Antiscalar: scalar_bulk_expansion_dual_number;
  Scalar, OddGrade => OddGrade: scalar_bulk_expansion_odd_grade;
  Scalar, EvenGrade => EvenGrade: scalar_bulk_expansion_even_grade;

  Vector, Multivector => Multivector: vector_bulk_expansion_multivector;
  Vector, Scalar => Scalar: return_scalar_zero_binary;
  Vector, Vector => Antiscalar: vector_bulk_expansion_vector;
  Vector, Bivector => Trivector: vector_bulk_expansion_bivector;
  Vector, Trivector => Bivector: vector_bulk_expansion_trivector;
  Vector, Antiscalar => Scalar: return_scalar_zero_binary;
  Vector, DualNumber => Scalar: return_scalar_zero_binary;
  Vector, OddGrade => EvenGrade: vector_bulk_expansion_odd_grade;
  Vector, EvenGrade => Trivector: vector_bulk_expansion_even_grade;

  Bivector, Multivector => Multivector: bivector_bulk_expansion_multivector;
  Bivector, Scalar => Scalar: return_scalar_zero_binary;
  Bivector, Vector => Scalar: return_scalar_zero_binary;
  Bivector, Bivector => Antiscalar: bivector_bulk_expansion_bivector;
  Bivector, Trivector => Trivector: bivector_bulk_expansion_trivector;
  Bivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Bivector, DualNumber => Scalar: return_scalar_zero_binary;
  Bivector, OddGrade => Trivector: bivector_bulk_expansion_odd_grade;
  Bivector, EvenGrade => Antiscalar: bivector_bulk_expansion_even_grade;

  Trivector, Multivector => Antiscalar: trivector_bulk_expansion_multivector;
  Trivector, Scalar => Scalar: return_scalar_zero_binary;
  Trivector, Vector => Scalar: return_scalar_zero_binary;
  Trivector, Bivector => Scalar: return_scalar_zero_binary;
  Trivector, Trivector => Antiscalar: trivector_bulk_expansion_trivector;
  Trivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Trivector, DualNumber => Scalar: return_scalar_zero_binary;
  Trivector, OddGrade => Antiscalar: trivector_bulk_expansion_odd_grade;
  Trivector, EvenGrade => Scalar: return_scalar_zero_binary;

  Antiscalar, Multivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Scalar => Scalar: return_scalar_zero_binary;
  Antiscalar, Vector => Scalar: return_scalar_zero_binary;
  Antiscalar, Bivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Trivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Antiscalar, DualNumber => Scalar: return_scalar_zero_binary;
  Antiscalar, OddGrade => Scalar: return_scalar_zero_binary;
  Antiscalar, EvenGrade => Scalar: return_scalar_zero_binary;

  DualNumber, Multivector => Multivector: dual_number_bulk_expansion_multivector;
  DualNumber, Scalar => Antiscalar: dual_number_bulk_expansion_scalar;
  DualNumber, Vector => Trivector: dual_number_bulk_expansion_vector;
  DualNumber, Bivector => Bivector: dual_number_bulk_expansion_bivector;
  DualNumber, Trivector => Vector: dual_number_bulk_expansion_trivector;
  DualNumber, Antiscalar => Scalar: return_scalar_zero_binary;
  DualNumber, DualNumber => Antiscalar: dual_number_bulk_expansion_dual_number;
  DualNumber, OddGrade => OddGrade: dual_number_bulk_expansion_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_bulk_expansion_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_bulk_expansion_multivector;
  OddGrade, Scalar => Scalar: return_scalar_zero_binary;
  OddGrade, Vector => Antiscalar: odd_grade_bulk_expansion_vector;
  OddGrade, Bivector => Trivector: odd_grade_bulk_expansion_bivector;
  OddGrade, Trivector => EvenGrade: odd_grade_bulk_expansion_trivector;
  OddGrade, Antiscalar => Scalar: return_scalar_zero_binary;
  OddGrade, DualNumber => Scalar: return_scalar_zero_binary;
  OddGrade, OddGrade => EvenGrade: odd_grade_bulk_expansion_odd_grade;
  OddGrade, EvenGrade => Trivector: odd_grade_bulk_expansion_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_bulk_expansion_multivector;
  EvenGrade, Scalar => Antiscalar: even_grade_bulk_expansion_scalar;
  EvenGrade, Vector => Trivector: even_grade_bulk_expansion_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_bulk_expansion_bivector;
  EvenGrade, Trivector => OddGrade: even_grade_bulk_expansion_trivector;
  EvenGrade, Antiscalar => Scalar: return_scalar_zero_binary;
  EvenGrade, DualNumber => Antiscalar: even_grade_bulk_expansion_dual_number;
  EvenGrade, OddGrade => OddGrade: even_grade_bulk_expansion_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_bulk_expansion_even_grade;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_bulk_expansion_multivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3,
    e23: l23, e31: l31, e12: l12,
    e321: l321,
    ..
  }: Multivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e4 = -ls*r321;

  let e41 = -ls*r23 + l1*r321;
  let e42 = -ls*r31 + l2*r321;
  let e43 = -ls*r12 + l3*r321;

  let e423 = ls*r1 + l2*r12 - l3*r31 - l23*r321;
  let e431 = ls*r2 + l3*r23 - l1*r12 - l31*r321;
  let e412 = ls*r3 + l1*r31 - l2*r23 - l12*r321;

  let e1234 = ls*rs + l1*r1 + l2*r2 + l3*r3
    + l23*r23 + l31*r31 + l12*r12 + l321*r321;

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
fn multivector_bulk_expansion_scalar(
  Multivector { s: ls, .. }: Multivector,
  Scalar { s: rs }: Scalar,
) -> Antiscalar {

  let e1234 = ls*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn multivector_bulk_expansion_vector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3,
    ..
  }: Multivector,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Multivector {

  let e423 = ls*r1;
  let e431 = ls*r2;
  let e412 = ls*r3;

  let e1234 = l1*r1 + l2*r2 + l3*r3;

  Multivector {
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_bulk_expansion_bivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3,
    e23: l23, e31: l31, e12: l12,
    ..
  }: Multivector,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Multivector {

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e423 = l2*r12 - l3*r31;
  let e431 = l3*r23 - l1*r12;
  let e412 = l1*r31 - l2*r23;

  let e1234 = l23*r23 + l31*r31 + l12*r12;

  Multivector {
    e41, e42, e43,
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_bulk_expansion_trivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3,
    e23: l23, e31: l31, e12: l12,
    e321: l321,
    ..
  }: Multivector,
  Trivector { e321: r321, .. }: Trivector,
) -> Multivector {

  let e4 = -ls*r321;

  let e41 = l1*r321;
  let e42 = l2*r321;
  let e43 = l3*r321;

  let e423 = -l23*r321;
  let e431 = -l31*r321;
  let e412 = -l12*r321;

  let e1234 = l321*r321;

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
fn multivector_bulk_expansion_dual_number(
  Multivector { s: ls, .. }: Multivector,
  DualNumber { s: rs, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = ls*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn multivector_bulk_expansion_odd_grade(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3,
    e23: l23, e31: l31, e12: l12,
    e321: l321,
    ..
  }: Multivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> Multivector {

  let e4 = -ls*r321;

  let e41 = l1*r321;
  let e42 = l2*r321;
  let e43 = l3*r321;

  let e423 = ls*r1 - l23*r321;
  let e431 = ls*r2 - l31*r321;
  let e412 = ls*r3 - l12*r321;

  let e1234 = l1*r1 + l2*r2 + l3*r3 + l321*r321;

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
fn multivector_bulk_expansion_even_grade(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3,
    e23: l23, e31: l31, e12: l12,
    ..
  }: Multivector,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> Multivector {

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e423 = l2*r12 - l3*r31;
  let e431 = l3*r23 - l1*r12;
  let e412 = l1*r31 - l2*r23;

  let e1234 = ls*rs + l23*r23 + l31*r31 + l12*r12;

  Multivector {
    e41, e42, e43,
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

// scalar

#[rustfmt::skip]
#[inline]
fn scalar_bulk_expansion_multivector(
  Scalar { s: ls }: Scalar,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e4 = -ls*r321;

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e423 = ls*r1;
  let e431 = ls*r2;
  let e412 = ls*r3;

  let e1234 = ls*rs;

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
fn scalar_bulk_expansion_scalar(
  Scalar { s: ls }: Scalar,
  Scalar { s: rs }: Scalar,
) -> Antiscalar {

  let e1234 = ls*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_expansion_vector(
  Scalar { s: ls }: Scalar,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Trivector {

  let e423 = ls*r1;
  let e431 = ls*r2;
  let e412 = ls*r3;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_expansion_bivector(
  Scalar { s: ls }: Scalar,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Bivector {

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_expansion_trivector(
  Scalar { s: ls }: Scalar,
  Trivector { e321: r321, .. }: Trivector,
) -> Vector {

  let e4 = -ls*r321;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_expansion_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { s: rs, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = ls*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_expansion_odd_grade(
  Scalar { s: ls }: Scalar,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e4 = -ls*r321;

  let e423 = ls*r1;
  let e431 = ls*r2;
  let e412 = ls*r3;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_bulk_expansion_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e1234 = ls*rs;

  EvenGrade {
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

// vector

#[rustfmt::skip]
#[inline]
fn vector_bulk_expansion_multivector(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  Multivector {
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e41 = l1*r321;
  let e42 = l2*r321;
  let e43 = l3*r321;

  let e423 = l2*r12 - l3*r31;
  let e431 = l3*r23 - l1*r12;
  let e412 = l1*r31 - l2*r23;

  let e1234 = l1*r1 + l2*r2 + l3*r3;

  Multivector {
    e41, e42, e43,
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_expansion_vector(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Antiscalar {

  let e1234 = l1*r1 + l2*r2 + l3*r3;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_expansion_bivector(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Trivector {

  let e423 = l2*r12 - l3*r31;
  let e431 = l3*r23 - l1*r12;
  let e412 = l1*r31 - l2*r23;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_expansion_trivector(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  Trivector { e321: r321, .. }: Trivector,
) -> Bivector {

  let e41 = l1*r321;
  let e42 = l2*r321;
  let e43 = l3*r321;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_expansion_odd_grade(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> EvenGrade {

  let e41 = l1*r321;
  let e42 = l2*r321;
  let e43 = l3*r321;

  let e1234 = l1*r1 + l2*r2 + l3*r3;

  EvenGrade {
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_bulk_expansion_even_grade(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  EvenGrade { e23: r23, e31: r31, e12: r12, .. }: EvenGrade,
) -> Trivector {

  let e423 = l2*r12 - l3*r31;
  let e431 = l3*r23 - l1*r12;
  let e412 = l1*r31 - l2*r23;

  Trivector { e423, e431, e412, ..zero() }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_bulk_expansion_multivector(
  Bivector { e23: l23, e31: l31, e12: l12, .. }: Bivector,
  Multivector {
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e423 = -l23*r321;
  let e431 = -l31*r321;
  let e412 = -l12*r321;

  let e1234 = l23*r23 + l31*r31 + l12*r12;

  Multivector {
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_expansion_bivector(
  Bivector { e23: l23, e31: l31, e12: l12, .. }: Bivector,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Antiscalar {

  let e1234 = l23*r23 + l31*r31 + l12*r12;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_expansion_trivector(
  Bivector { e23: l23, e31: l31, e12: l12, .. }: Bivector,
  Trivector { e321: r321, .. }: Trivector
) -> Trivector {

  let e423 = -l23*r321;
  let e431 = -l31*r321;
  let e412 = -l12*r321;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_expansion_odd_grade(
  Bivector { e23: l23, e31: l31, e12: l12, .. }: Bivector,
  OddGrade { e321: r321, .. }: OddGrade,
) -> Trivector {

  let e423 = -l23*r321;
  let e431 = -l31*r321;
  let e412 = -l12*r321;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bivector_bulk_expansion_even_grade(
  Bivector { e23: l23, e31: l31, e12: l12, .. }: Bivector,
  EvenGrade { e23: r23, e31: r31, e12: r12, .. }: EvenGrade,
) -> Antiscalar {

  let e1234 = l23*r23 + l31*r31 + l12*r12;

  Antiscalar { e1234 }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_bulk_expansion_multivector(
  Trivector { e321: l321, .. }: Trivector,
  Multivector { e321: r321, .. }: Multivector,
) -> Antiscalar {

  let e1234 = l321*r321;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_expansion_trivector(
  Trivector { e321: l321, .. }: Trivector,
  Trivector { e321: r321, .. }: Trivector,
) -> Antiscalar {

  let e1234 = l321*r321;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn trivector_bulk_expansion_odd_grade(
  Trivector { e321: l321, .. }: Trivector,
  OddGrade { e321: r321, .. }: OddGrade,
) -> Antiscalar {

  let e1234 = l321*r321;

  Antiscalar { e1234 }
}

// dual number

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_expansion_multivector(
  DualNumber { s: ls, .. }: DualNumber,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e4 = -ls*r321;

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e423 = ls*r1;
  let e431 = ls*r2;
  let e412 = ls*r3;

  let e1234 = ls*rs;

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
fn dual_number_bulk_expansion_scalar(
  DualNumber { s: ls, .. }: DualNumber,
  Scalar { s: rs }: Scalar,
) -> Antiscalar {

  let e1234 = ls*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_expansion_vector(
  DualNumber { s: ls, .. }: DualNumber,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Trivector {

  let e423 = ls*r1;
  let e431 = ls*r2;
  let e412 = ls*r3;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_expansion_bivector(
  DualNumber { s: ls, .. }: DualNumber,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Bivector {

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_expansion_trivector(
  DualNumber { s: ls, .. }: DualNumber,
  Trivector { e321: r321, .. }: Trivector,
) -> Vector {

  let e4 = -ls*r321;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_expansion_dual_number(
  DualNumber { s: ls, .. }: DualNumber,
  DualNumber { s: rs, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = ls*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_expansion_odd_grade(
  DualNumber { s: ls, .. }: DualNumber,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e4 = -ls*r321;

  let e423 = ls*r1;
  let e431 = ls*r2;
  let e412 = ls*r3;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_bulk_expansion_even_grade(
  DualNumber { s: ls, .. }: DualNumber,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e1234 = ls*rs;

  EvenGrade {
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

// odd grade

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_expansion_multivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3,
    e321: l321,
    ..
  }: OddGrade,
  Multivector {
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e41 = l1*r321;
  let e42 = l2*r321;
  let e43 = l3*r321;

  let e423 = l2*r12 - l3*r31;
  let e431 = l3*r23 - l1*r12;
  let e412 = l1*r31 - l2*r23;

  let e1234 = l1*r1 + l2*r2 + l3*r3 + l321*r321;

  Multivector {
    e41, e42, e43,
    e423, e431, e412,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_expansion_vector(
  OddGrade { e1: l1, e2: l2, e3: l3, .. }: OddGrade,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Antiscalar {

  let e1234 = l1*r1 + l2*r2 + l3*r3;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_expansion_bivector(
  OddGrade { e1: l1, e2: l2, e3: l3, .. }: OddGrade,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Trivector {

  let e423 = l2*r12 - l3*r31;
  let e431 = l3*r23 - l1*r12;
  let e412 = l1*r31 - l2*r23;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_expansion_trivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3,
    e321: l321,
    ..
  }: OddGrade,
  Trivector { e321: r321, .. }: Trivector,
) -> EvenGrade {

  let e41 = l1*r321;
  let e42 = l2*r321;
  let e43 = l3*r321;

  let e1234 = l321*r321;

  EvenGrade {
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_expansion_odd_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3,
    e321: l321,
    ..
  }: OddGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> EvenGrade {

  let e41 = l1*r321;
  let e42 = l2*r321;
  let e43 = l3*r321;

  let e1234 = l1*r1 + l2*r2 + l3*r3 + l321*r321;

  EvenGrade {
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_bulk_expansion_even_grade(
  OddGrade { e1: l1, e2: l2, e3: l3, .. }: OddGrade,
  EvenGrade { e23: r23, e31: r31, e12: r12, .. }: EvenGrade,
) -> Trivector {

  let e423 = l2*r12 - l3*r31;
  let e431 = l3*r23 - l1*r12;
  let e412 = l1*r31 - l2*r23;

  Trivector { e423, e431, e412, ..zero() }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_expansion_multivector(
  EvenGrade {
    s: ls,
    e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3,
    e23: r23, e31: r31, e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e4 = -ls*r321;

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e423 = ls*r1 - l23*r321;
  let e431 = ls*r2 - l31*r321;
  let e412 = ls*r3 - l12*r321;

  let e1234 = ls*rs + l23*r23 + l31*r31 + l12*r12;

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
fn even_grade_bulk_expansion_scalar(
  EvenGrade { s: ls, .. }: EvenGrade,
  Scalar { s: rs }: Scalar,
) -> Antiscalar {

  let e1234 = ls*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_expansion_vector(
  EvenGrade { s: ls, .. }: EvenGrade,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Trivector {

  let e423 = ls*r1;
  let e431 = ls*r2;
  let e412 = ls*r3;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_expansion_bivector(
  EvenGrade {
    s: ls,
    e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> EvenGrade {

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e1234 = l23*r23 + l31*r31 + l12*r12;

  EvenGrade {
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_expansion_trivector(
  EvenGrade {
    s: ls,
    e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  Trivector { e321: r321, .. }: Trivector,
) -> OddGrade {

  let e4 = -ls*r321;

  let e423 = -l23*r321;
  let e431 = -l31*r321;
  let e412 = -l12*r321;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_expansion_dual_number(
  EvenGrade { s: ls, .. }: EvenGrade,
  DualNumber { s: rs, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = ls*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_expansion_odd_grade(
  EvenGrade {
    s: ls,
    e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e4 = -ls*r321;

  let e423 = ls*r1 - l23*r321;
  let e431 = ls*r2 - l31*r321;
  let e412 = ls*r3 - l12*r321;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_bulk_expansion_even_grade(
  EvenGrade {
    s: ls,
    e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let e41 = -ls*r23;
  let e42 = -ls*r31;
  let e43 = -ls*r12;

  let e1234 = ls*rs + l23*r23 + l31*r31 + l12*r12;

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
      bulk_expansion(MULTIVECTOR_A, MULTIVECTOR_B),
      wedge(MULTIVECTOR_A, bulk_dual(MULTIVECTOR_B))
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
          Multivector::from(bulk_expansion(multivector, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_expansion(scalar, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_expansion(vector, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_expansion(bivector, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_expansion(trivector, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_expansion(antiscalar, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_expansion(dual_number, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_expansion(odd_grade, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bulk_expansion(even_grade, variant)),
          Multivector::from(bulk_expansion(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
