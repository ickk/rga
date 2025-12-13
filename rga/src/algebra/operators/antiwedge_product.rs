use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_binary_operation, return_scalar_zero_binary},
};

/// a ∨ b
///
/// The exterior antiproduct.
#[inline]
pub fn antiwedge<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as AntiwedgeProduct<Rhs>>::Output
where
  Lhs: AntiwedgeProduct<Rhs>,
{
  a.antiwedge(b)
}

/// a ∨ b
///
/// The exterior antiproduct.
pub trait AntiwedgeProduct<Rhs> {
  type Output;

  /// a ∨ b
  ///
  /// The exterior antiproduct.
  #[doc(
    alias = "exterior antiproduct",
    alias = "regressive product",
    alias = "product"
  )]
  fn antiwedge(self, b: Rhs) -> Self::Output;
}

impl_binary_operation!(AntiwedgeProduct::antiwedge {
  Multivector, Multivector => Multivector: multivector_antiwedge_multivector;
  Multivector, Scalar => Scalar: multivector_antiwedge_scalar;
  Multivector, Vector => Multivector: multivector_antiwedge_vector;
  Multivector, Bivector => Multivector: multivector_antiwedge_bivector;
  Multivector, Trivector => Multivector: multivector_antiwedge_trivector;
  Multivector, Antiscalar => Multivector: multivector_antiwedge_antiscalar;
  Multivector, DualNumber => Multivector: multivector_antiwedge_dual_number;
  Multivector, OddGrade => Multivector: multivector_antiwedge_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_antiwedge_even_grade;

  Scalar, Multivector => Scalar: scalar_antiwedge_multivector;
  Scalar, Scalar => Scalar: return_scalar_zero_binary;
  Scalar, Vector => Scalar: return_scalar_zero_binary;
  Scalar, Bivector => Scalar: return_scalar_zero_binary;
  Scalar, Trivector => Scalar: return_scalar_zero_binary;
  Scalar, Antiscalar => Scalar: scalar_antiwedge_antiscalar;
  Scalar, DualNumber => Scalar: scalar_antiwedge_dual_number;
  Scalar, OddGrade => Scalar: return_scalar_zero_binary;
  Scalar, EvenGrade => Scalar: scalar_antiwedge_even_grade;

  Vector, Multivector => Multivector: vector_antiwedge_multivector;
  Vector, Scalar => Scalar: return_scalar_zero_binary;
  Vector, Vector => Scalar: return_scalar_zero_binary;
  Vector, Bivector => Scalar: return_scalar_zero_binary;
  Vector, Trivector => Scalar: vector_antiwedge_trivector;
  Vector, Antiscalar => Vector: vector_antiwedge_antiscalar;
  Vector, DualNumber => Vector: vector_antiwedge_dual_number;
  Vector, OddGrade => Scalar: vector_antiwedge_odd_grade;
  Vector, EvenGrade => Vector: vector_antiwedge_even_grade;

  Bivector, Multivector => Multivector: bivector_antiwedge_multivector;
  Bivector, Scalar => Scalar: return_scalar_zero_binary;
  Bivector, Vector => Scalar: return_scalar_zero_binary;
  Bivector, Bivector => Scalar: bivector_antiwedge_bivector;
  Bivector, Trivector => Vector: bivector_antiwedge_trivector;
  Bivector, Antiscalar => Bivector: bivector_antiwedge_antiscalar;
  Bivector, DualNumber => Bivector: bivector_antiwedge_dual_number;
  Bivector, OddGrade => Vector: bivector_antiwedge_odd_grade;
  Bivector, EvenGrade => EvenGrade: bivector_antiwedge_even_grade;

  Trivector, Multivector => Multivector: trivector_antiwedge_multivector;
  Trivector, Scalar => Scalar: return_scalar_zero_binary;
  Trivector, Vector => Scalar: trivector_antiwedge_vector;
  Trivector, Bivector => Vector: trivector_antiwedge_bivector;
  Trivector, Trivector => Bivector: trivector_antiwedge_trivector;
  Trivector, Antiscalar => Trivector: trivector_antiwedge_antiscalar;
  Trivector, DualNumber => Trivector: trivector_antiwedge_dual_number;
  Trivector, OddGrade => EvenGrade: trivector_antiwedge_odd_grade;
  Trivector, EvenGrade => OddGrade: trivector_antiwedge_even_grade;

  Antiscalar, Multivector => Multivector: antiscalar_antiwedge_multivector;
  Antiscalar, Scalar => Scalar: antiscalar_antiwedge_scalar;
  Antiscalar, Vector => Vector: antiscalar_antiwedge_vector;
  Antiscalar, Bivector => Bivector: antiscalar_antiwedge_bivector;
  Antiscalar, Trivector => Trivector: antiscalar_antiwedge_trivector;
  Antiscalar, Antiscalar => Antiscalar: antiscalar_antiwedge_antiscalar;
  Antiscalar, DualNumber => DualNumber: antiscalar_antiwedge_dual_number;
  Antiscalar, OddGrade => OddGrade: antiscalar_antiwedge_odd_grade;
  Antiscalar, EvenGrade => EvenGrade: antiscalar_antiwedge_even_grade;

  DualNumber, Multivector => Multivector: dual_number_antiwedge_multivector;
  DualNumber, Scalar => Scalar: dual_number_antiwedge_scalar;
  DualNumber, Vector => Vector: dual_number_antiwedge_vector;
  DualNumber, Bivector => Bivector: dual_number_antiwedge_bivector;
  DualNumber, Trivector => Trivector: dual_number_antiwedge_trivector;
  DualNumber, Antiscalar => DualNumber: dual_number_antiwedge_antiscalar;
  DualNumber, DualNumber => DualNumber: dual_number_antiwedge_dual_number;
  DualNumber, OddGrade => OddGrade: dual_number_antiwedge_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_antiwedge_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_antiwedge_multivector;
  OddGrade, Scalar => Scalar: return_scalar_zero_binary;
  OddGrade, Vector => Scalar: odd_grade_antiwedge_vector;
  OddGrade, Bivector => Vector: odd_grade_antiwedge_bivector;
  OddGrade, Trivector => EvenGrade: odd_grade_antiwedge_trivector;
  OddGrade, Antiscalar => OddGrade: odd_grade_antiwedge_antiscalar;
  OddGrade, DualNumber => OddGrade: odd_grade_antiwedge_dual_number;
  OddGrade, OddGrade => EvenGrade: odd_grade_antiwedge_odd_grade;
  OddGrade, EvenGrade => OddGrade: odd_grade_antiwedge_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_antiwedge_multivector;
  EvenGrade, Scalar => Scalar: even_grade_antiwedge_scalar;
  EvenGrade, Vector => Vector: even_grade_antiwedge_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_antiwedge_bivector;
  EvenGrade, Trivector => OddGrade: even_grade_antiwedge_trivector;
  EvenGrade, Antiscalar => EvenGrade: even_grade_antiwedge_antiscalar;
  EvenGrade, DualNumber => EvenGrade: even_grade_antiwedge_dual_number;
  EvenGrade, OddGrade => OddGrade: even_grade_antiwedge_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_antiwedge_even_grade;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_antiwedge_multivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = (ls*r1234 + l1234*rs) + (l1*r423 - l423*r1)
    + (l2*r431 - l431*r2) + (l3*r412 - l412*r3)
    + (l4*r321 - l321*r4) - (l43*r12 + l12*r43)
    - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e1 = (l1*r1234 + l1234*r1) + (l41*r321 + l321*r41)
    + (l31*r412 + l412*r31) - (l12*r431 + l431*r12);
  let e2 = (l2*r1234 + l1234*r2) + (l42*r321 + l321*r42)
    + (l12*r423 + l423*r12) - (l23*r412 + l412*r23);
  let e3 = (l3*r1234 + l1234*r3) + (l43*r321 + l321*r43)
    + (l23*r431 + l431*r23) - (l31*r423 + l423*r31);
  let e4 = (l4*r1234 + l1234*r4) - (l41*r423 + l423*r41)
    - (l42*r431 + l431*r42) - (l43*r412 + l412*r43);

  let e41 = (l41*r1234 + l1234*r41) + (l412*r431 - l431*r412);
  let e42 = (l42*r1234 + l1234*r42) + (l423*r412 - l412*r423);
  let e43 = (l43*r1234 + l1234*r43) + (l431*r423 - l423*r431);
  let e23 = (l23*r1234 + l1234*r23) + (l423*r321 - l321*r423);
  let e31 = (l31*r1234 + l1234*r31) + (l431*r321 - l321*r431);
  let e12 = (l12*r1234 + l1234*r12) + (l412*r321 - l321*r412);

  let e423 = l423*r1234 + l1234*r423;
  let e431 = l431*r1234 + l1234*r431;
  let e412 = l412*r1234 + l1234*r412;
  let e321 = l321*r1234 + l1234*r321;

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
fn multivector_antiwedge_scalar(
  Multivector { e1234: l1234, .. }: Multivector,
  Scalar { s: rs, .. }: Scalar,
) -> Scalar {

  let s = l1234*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn multivector_antiwedge_vector(
  Multivector {
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
    ..
  }: Multivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Multivector {

  let s = -(l423*r1 + l431*r2 + l412*r3 + l321*r4);

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  Multivector {
    s,
    e1, e2, e3, e4,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_antiwedge_bivector(
  Multivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
    ..
  }: Multivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Multivector {

  let s = -(l43*r12 + l12*r43 + l41*r23 + l23*r41 + l31*r42 + l42*r31);

  let e1 = l321*r41 + l412*r31 - l431*r12;
  let e2 = l321*r42 + l423*r12 - l412*r23;
  let e3 = l321*r43 + l431*r23 - l423*r31;
  let e4 = -(l423*r41 + l431*r42 + l412*r43);

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23;
  let e31 = l1234*r31;
  let e12 = l1234*r12;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_antiwedge_trivector(
  Multivector {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
    ..
  }: Multivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Multivector {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  let e1 = l41*r321 + l31*r412 - l12*r431;
  let e2 = l42*r321 + l12*r423 - l23*r412;
  let e3 = l43*r321 + l23*r431 - l31*r423;
  let e4 = -(l41*r423 + l42*r431 + l43*r412);

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

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
fn multivector_antiwedge_antiscalar(
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
fn multivector_antiwedge_dual_number(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> Multivector {

  let s = ls*r1234 + l1234*rs;

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
fn multivector_antiwedge_odd_grade(
  Multivector {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
    ..
  }: Multivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> Multivector {

  let s = (l1*r423 - l423*r1) + (l2*r431 - l431*r2)
    + (l3*r412 - l412*r3) + (l4*r321 - l321*r4);

  let e1 = l1234*r1 + l41*r321 + l31*r412 - l12*r431;
  let e2 = l1234*r2 + l42*r321 + l12*r423 - l23*r412;
  let e3 = l1234*r3 + l43*r321 + l23*r431 - l31*r423;
  let e4 = l1234*r4 - l41*r423 - l42*r431 - l43*r412;

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

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
fn multivector_antiwedge_even_grade(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> Multivector {

  let s = (ls*r1234 + l1234*rs) - (l43*r12 + l12*r43)
    - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e1 = l1*r1234 + l321*r41 + l412*r31 - l431*r12;
  let e2 = l2*r1234 + l321*r42 + l423*r12 - l412*r23;
  let e3 = l3*r1234 + l321*r43 + l431*r23 - l423*r31;
  let e4 = l4*r1234 - l423*r41 - l431*r42 - l412*r43;

  let e41 = l41*r1234 + l1234*r41;
  let e42 = l42*r1234 + l1234*r42;
  let e43 = l43*r1234 + l1234*r43;
  let e23 = l23*r1234 + l1234*r23;
  let e31 = l31*r1234 + l1234*r31;
  let e12 = l12*r1234 + l1234*r12;

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

// scalar

#[rustfmt::skip]
#[inline]
fn scalar_antiwedge_multivector(
  Scalar { s: ls }: Scalar,
  Multivector { e1234: r1234, .. }: Multivector,
) -> Scalar {

  let s = ls*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_antiwedge_antiscalar(
  Scalar { s: ls }: Scalar,
  Antiscalar { e1234: r1234, .. }: Antiscalar,
) -> Scalar {

  let s = ls*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_antiwedge_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Scalar {

  let s = ls*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_antiwedge_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade { e1234: r1234, .. }: EvenGrade,
) -> Scalar {

  let s = ls*r1234;

  Scalar { s }
}

// vector

#[rustfmt::skip]
#[inline]
fn vector_antiwedge_multivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Multivector {
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  Multivector {
    s,
    e1, e2, e3, e4,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_antiwedge_trivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Scalar {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_antiwedge_antiscalar(
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
fn vector_antiwedge_dual_number(
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
fn vector_antiwedge_odd_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  OddGrade {
    e423: r423, e431: r431, e412: r412, e321: r321,
    ..
  }: OddGrade,
) -> Scalar {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn vector_antiwedge_even_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  EvenGrade { e1234: r1234, .. }: EvenGrade,
) -> Vector {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  Vector { e1, e2, e3, e4 }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_antiwedge_multivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Multivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = -(l43*r12 + l12*r43 + l41*r23 + l23*r41 + l31*r42 + l42*r31);

  let e1 = l41*r321 + l31*r412 - l12*r431;
  let e2 = l42*r321 + l12*r423 - l23*r412;
  let e3 = l43*r321 + l23*r431 - l31*r423;
  let e4 = -(l41*r423 + l42*r431 + l43*r412);

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_antiwedge_bivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Scalar {

  let s = -(l43*r12 + l12*r43 + l41*r23 + l23*r41 + l31*r42 + l42*r31);

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn bivector_antiwedge_trivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Vector {

  let e1 = l41*r321 + l31*r412 - l12*r431;
  let e2 = l42*r321 + l12*r423 - l23*r412;
  let e3 = l43*r321 + l23*r431 - l31*r423;
  let e4 = -(l41*r423 + l42*r431 + l43*r412);

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn bivector_antiwedge_antiscalar(
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
fn bivector_antiwedge_dual_number(
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
fn bivector_antiwedge_odd_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  OddGrade { e423: r423, e431: r431, e412: r412, e321: r321, .. }: OddGrade,
) -> Vector {

  let e1 = l41*r321 + l31*r412 - l12*r431;
  let e2 = l42*r321 + l12*r423 - l23*r412;
  let e3 = l43*r321 + l23*r431 - l31*r423;
  let e4 = -(l41*r423 + l42*r431 + l43*r412);

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn bivector_antiwedge_even_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  EvenGrade {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let s = -(l43*r12 + l12*r43 + l41*r23 + l23*r41 + l31*r42 + l42*r31);

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234;
  let e31 = l31*r1234;
  let e12 = l12*r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_antiwedge_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Multivector {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = -(l423*r1 + l431*r2 + l412*r3 + l321*r4);

  let e1 = l321*r41 + l412*r31 - l431*r12;
  let e2 = l321*r42 + l423*r12 - l412*r23;
  let e3 = l321*r43 + l431*r23 - l423*r31;
  let e4 = -(l423*r41 + l431*r42 + l412*r43);

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

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
fn trivector_antiwedge_vector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Scalar {

  let s = -(l423*r1 + l431*r2 + l412*r3 + l321*r4);

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn trivector_antiwedge_bivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Vector {

  let e1 = l321*r41 + l412*r31 - l431*r12;
  let e2 = l321*r42 + l423*r12 - l412*r23;
  let e3 = l321*r43 + l431*r23 - l423*r31;
  let e4 = -(l423*r41 + l431*r42 + l412*r43);

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn trivector_antiwedge_trivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Bivector {

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn trivector_antiwedge_antiscalar(
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
fn trivector_antiwedge_dual_number(
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
fn trivector_antiwedge_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> EvenGrade {

  let s = -(l423*r1 + l431*r2 + l412*r3 + l321*r4);

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_antiwedge_even_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  EvenGrade {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> OddGrade {

  let e1 = l321*r41 + l412*r31 - l431*r12;
  let e2 = l321*r42 + l423*r12 - l412*r23;
  let e3 = l321*r43 + l431*r23 - l423*r31;
  let e4 = -(l423*r41 + l431*r42 + l412*r43);

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// antiscalar

#[rustfmt::skip]
#[inline]
fn antiscalar_antiwedge_multivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = l1234*rs;

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23;
  let e31 = l1234*r31;
  let e12 = l1234*r12;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

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
fn antiscalar_antiwedge_scalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Scalar { s: rs }: Scalar,
) -> Scalar {

  let s = l1234*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antiwedge_vector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Vector {

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antiwedge_bivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Bivector {

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23;
  let e31 = l1234*r31;
  let e12 = l1234*r12;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antiwedge_trivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Trivector {

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antiwedge_antiscalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Antiscalar {

  let e1234 = l1234*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antiwedge_dual_number(
  Antiscalar { e1234: l1234 }: Antiscalar,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let s = l1234*rs;
  let e1234 = l1234*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antiwedge_odd_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antiwedge_even_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = l1234*rs;

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23;
  let e31 = l1234*r31;
  let e12 = l1234*r12;

  let e1234 = l1234*r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

// multivector

#[rustfmt::skip]
#[inline]
fn dual_number_antiwedge_multivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = ls*r1234 + l1234*rs;

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23;
  let e31 = l1234*r31;
  let e12 = l1234*r12;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

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
fn dual_number_antiwedge_scalar(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Scalar { s: rs }: Scalar,
) -> Scalar {

  let s = l1234*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antiwedge_vector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Vector {

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antiwedge_bivector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Bivector {

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23;
  let e31 = l1234*r31;
  let e12 = l1234*r12;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antiwedge_trivector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Trivector {

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antiwedge_antiscalar(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> DualNumber {

  let s = ls*r1234;
  let e1234 = l1234*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antiwedge_dual_number(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let s = ls*r1234 + l1234*rs;
  let e1234 = l1234*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antiwedge_odd_grade(
  DualNumber { e1234: l1234, .. }: DualNumber,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antiwedge_even_grade(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*r1234 + l1234*rs;

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23;
  let e31 = l1234*r31;
  let e12 = l1234*r12;


  let e1234 = l1234*r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

// odd grade

#[rustfmt::skip]
#[inline]
fn odd_grade_antiwedge_multivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Multivector {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
    ..
  }: Multivector,
) -> Multivector {

  let s = (l1*r423 - l423*r1) + (l2*r431 - l431*r2)
    + (l3*r412 - l412*r3) + (l4*r321 - l321*r4);

  let e1 = l1*r1234 + l321*r41 + l412*r31 - l431*r12;
  let e2 = l2*r1234 + l321*r42 + l423*r12 - l412*r23;
  let e3 = l3*r1234 + l321*r43 + l431*r23 - l423*r31;
  let e4 = l4*r1234 - l423*r41 - l431*r42 - l412*r43;

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

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
fn odd_grade_antiwedge_vector(
  OddGrade { e423: l423, e431: l431, e412: l412, e321: l321, .. }: OddGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Scalar {

  let s = -(l423*r1 + l431*r2 + l412*r3 + l321*r4);

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antiwedge_bivector(
  OddGrade { e423: l423, e431: l431, e412: l412, e321: l321, .. }: OddGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Vector {

  let e1 = l321*r41 + l412*r31 - l431*r12;
  let e2 = l321*r42 + l423*r12 - l412*r23;
  let e3 = l321*r43 + l431*r23 - l423*r31;
  let e4 = -(l423*r41 + l431*r42 + l412*r43);

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antiwedge_trivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> EvenGrade {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antiwedge_antiscalar(
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
fn odd_grade_antiwedge_dual_number(
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
fn odd_grade_antiwedge_odd_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> EvenGrade {

  let s = (l1*r423 - l423*r1) + (l2*r431 - l431*r2)
    + (l3*r412 - l412*r3) + (l4*r321 - l321*r4);

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antiwedge_even_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  EvenGrade {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*r1234 + l321*r41 + l412*r31 - l431*r12;
  let e2 = l2*r1234 + l321*r42 + l423*r12 - l412*r23;
  let e3 = l3*r1234 + l321*r43 + l431*r23 - l423*r31;
  let e4 = l4*r1234 - l423*r41 - l431*r42 - l412*r43;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_antiwedge_multivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = (ls*r1234 + l1234*rs) - (l43*r12 + l12*r43)
    - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e1 = l1234*r1 + l41*r321 + l31*r412 - l12*r431;
  let e2 = l1234*r2 + l42*r321 + l12*r423 - l23*r412;
  let e3 = l1234*r3 + l43*r321 + l23*r431 - l31*r423;
  let e4 = l1234*r4 - l41*r423 - l42*r431 - l43*r412;

  let e41 = l41*r1234 + l1234*r41;
  let e42 = l42*r1234 + l1234*r42;
  let e43 = l43*r1234 + l1234*r43;
  let e23 = l23*r1234 + l1234*r23;
  let e31 = l31*r1234 + l1234*r31;
  let e12 = l12*r1234 + l1234*r12;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

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
fn even_grade_antiwedge_scalar(
  EvenGrade { e1234: l1234, .. }: EvenGrade,
  Scalar { s: rs }: Scalar,
) -> Scalar {

  let s = l1234*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antiwedge_vector(
  EvenGrade { e1234: l1234, .. }: EvenGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Vector {

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antiwedge_bivector(
  EvenGrade {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
    ..
  }: EvenGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> EvenGrade {

  let s = -((l43*r12 + l12*r43) + (l41*r23 + l23*r41) + (l31*r42 + l42*r31));

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23;
  let e31 = l1234*r31;
  let e12 = l1234*r12;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antiwedge_trivector(
  EvenGrade {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
    ..
  }: EvenGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> OddGrade {

  let e1 = l41*r321 + l31*r412 - l12*r431;
  let e2 = l42*r321 + l12*r423 - l23*r412;
  let e3 = l43*r321 + l23*r431 - l31*r423;
  let e4 = -(l41*r423 + l42*r431 + l43*r412);

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antiwedge_antiscalar(
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
fn even_grade_antiwedge_dual_number(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> EvenGrade {

  let s = ls*r1234 + l1234*rs;

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
fn even_grade_antiwedge_odd_grade(
  EvenGrade {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
    ..
  }: EvenGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l1234*r1 + l41*r321 + l31*r412 - l12*r431;
  let e2 = l1234*r2 + l42*r321 + l12*r423 - l23*r412;
  let e3 = l1234*r3 + l43*r321 + l23*r431 - l31*r423;
  let e4 = l1234*r4 - l41*r423 - l42*r431 - l43*r412;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antiwedge_even_grade(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = (ls*r1234 + l1234*rs) - (l43*r12 + l12*r43)
    - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e41 = l41*r1234 + l1234*r41;
  let e42 = l42*r1234 + l1234*r42;
  let e43 = l43*r1234 + l1234*r43;
  let e23 = l23*r1234 + l1234*r23;
  let e31 = l31*r1234 + l1234*r31;
  let e12 = l12*r1234 + l1234*r12;

  let e1234 = l1234*r1234;

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
      antiwedge(MULTIVECTOR_A, MULTIVECTOR_B),
      left_complement(wedge(
        right_complement(MULTIVECTOR_A),
        right_complement(MULTIVECTOR_B)
      )),
      "The Antiwedge Product is defined as the Left Complement of the Wedge \
      Product of the Right Complement of its arguments"
    );
    assert_eq!(
      antiwedge(MULTIVECTOR_A, MULTIVECTOR_B),
      right_complement(wedge(
        left_complement(MULTIVECTOR_A),
        left_complement(MULTIVECTOR_B)
      )),
      "The Antiwedge Product is defined as the Right Complement of the Wedge \
      Product of the Left Complement of its arguments"
    );
  }

  #[test]
  fn de_morgans_laws() {
    assert_eq!(
      right_complement(antiwedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      wedge(
        right_complement(MULTIVECTOR_A),
        right_complement(MULTIVECTOR_B)
      ),
      "The Wedge Product & Antiwedge Product follow De Morgan's Laws, using \
      the Right Complement"
    );

    assert_eq!(
      right_complement(wedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      antiwedge(
        right_complement(MULTIVECTOR_A),
        right_complement(MULTIVECTOR_B)
      ),
      "The Wedge Product & Antiwedge Product follow De Morgan's Laws, using \
      the Right Complement"
    );

    assert_eq!(
      left_complement(antiwedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      wedge(
        left_complement(MULTIVECTOR_A),
        left_complement(MULTIVECTOR_B)
      ),
      "The Wedge Product & Antiwedge Product follow De Morgan's Laws, using \
      the Left Complement"
    );

    assert_eq!(
      left_complement(wedge(MULTIVECTOR_A, MULTIVECTOR_B)),
      antiwedge(
        left_complement(MULTIVECTOR_A),
        left_complement(MULTIVECTOR_B)
      ),
      "The Wedge Product & Antiwedge Product follow De Morgan's Laws, using \
      the Left Complement"
    );
  }

  #[rustfmt::skip]
  const A: Multivector = Multivector {
    e423: 2., e431: 3., e412: 5., e321: 7.,
    ..Multivector::ZERO
  };
  #[rustfmt::skip]
  const B: Multivector = Multivector {
    e423: -11., e431: -13., e412: -17., e321: -19.,
    ..Multivector::ZERO
  };
  #[rustfmt::skip]
  const C: Multivector = Multivector {
    e423: -11., e431: -13., e412: -17., e321: -19.,
    ..Multivector::ZERO
  };

  #[test]
  fn antivector_self() {
    assert_eq!(
      wedge(A, A),
      Multivector::zero(),
      "The Antiwedge Product of any Antivector with itself should be zero"
    );
  }

  #[test]
  fn antivector_anticommutivity() {
    assert_eq!(
      antiwedge(A, B),
      -antiwedge(B, A),
      "The Antiwedge Product of two Antivectors anticommutes"
    );
  }

  #[test]
  fn antivector_associativity() {
    assert_eq!(
      wedge(A, wedge(B, C)),
      wedge(wedge(A, B), C),
      "The Antiwedge Product of two Antivectors is associative"
    );
  }

  #[test]
  fn antivector_distributivity() {
    assert_eq!(
      wedge(A, B + C),
      wedge(A, B) + wedge(A, C),
      "The Antiwedge Product of two Antivectors distributes over addition"
    );

    assert_eq!(
      wedge(A + B, C),
      wedge(A, C) + wedge(B, C),
      "The Antiwedge Product of two Antivectors distributes over addition"
    )
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
          Multivector::from(antiwedge(multivector, variant)),
          Multivector::from(antiwedge(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiwedge(scalar, variant)),
          Multivector::from(antiwedge(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiwedge(vector, variant)),
          Multivector::from(antiwedge(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiwedge(bivector, variant)),
          Multivector::from(antiwedge(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiwedge(trivector, variant)),
          Multivector::from(antiwedge(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiwedge(antiscalar, variant)),
          Multivector::from(antiwedge(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiwedge(dual_number, variant)),
          Multivector::from(antiwedge(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiwedge(odd_grade, variant)),
          Multivector::from(antiwedge(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiwedge(even_grade, variant)),
          Multivector::from(antiwedge(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
