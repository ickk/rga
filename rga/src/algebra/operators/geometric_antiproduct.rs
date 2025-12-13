use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_binary_operation, return_scalar_zero_binary},
};

/// a ⟇ b
#[inline]
pub fn geometric_antiproduct<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as GeometricAntiproduct<Rhs>>::Output
where
  Lhs: GeometricAntiproduct<Rhs>,
{
  a.geometric_antiproduct(b)
}

/// a ⟇ b
pub trait GeometricAntiproduct<Rhs> {
  type Output;

  /// a ⟇ b
  #[doc(alias = "product")]
  fn geometric_antiproduct(self, b: Rhs) -> Self::Output;
}

impl_binary_operation!(GeometricAntiproduct::geometric_antiproduct {
  Multivector, Multivector => Multivector: multivector_antigeometric_multivector;
  Multivector, Scalar => Multivector: multivector_antigeometric_scalar;
  Multivector, Vector => Multivector: multivector_antigeometric_vector;
  Multivector, Bivector => Multivector: multivector_antigeometric_bivector;
  Multivector, Trivector => Multivector: multivector_antigeometric_trivector;
  Multivector, Antiscalar => Multivector: multivector_antigeometric_antiscalar;
  Multivector, DualNumber => Multivector: multivector_antigeometric_dual_number;
  Multivector, OddGrade => Multivector: multivector_antigeometric_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_antigeometric_even_grade;

  Scalar, Multivector => Multivector: scalar_antigeometric_multivector;
  Scalar, Scalar => Scalar: return_scalar_zero_binary;
  Scalar, Vector => Trivector: scalar_antigeometric_vector;
  Scalar, Bivector => Bivector: scalar_antigeometric_bivector;
  Scalar, Trivector => Vector: scalar_antigeometric_trivector;
  Scalar, Antiscalar => Scalar: scalar_antigeometric_antiscalar;
  Scalar, DualNumber => Scalar: scalar_antigeometric_dual_number;
  Scalar, OddGrade => OddGrade: scalar_antigeometric_odd_grade;
  Scalar, EvenGrade => Multivector: scalar_antigeometric_even_grade;

  Vector, Multivector => Multivector: vector_antigeometric_multivector;
  Vector, Scalar => Trivector: vector_antigeometric_scalar;
  Vector, Vector => EvenGrade: vector_antigeometric_vector;
  Vector, Bivector => OddGrade: vector_antigeometric_bivector;
  Vector, Trivector => EvenGrade: vector_antigeometric_trivector;
  Vector, Antiscalar => Vector: vector_antigeometric_antiscalar;
  Vector, DualNumber => OddGrade: vector_antigeometric_dual_number;
  Vector, OddGrade => EvenGrade: vector_antigeometric_odd_grade;
  Vector, EvenGrade => OddGrade: vector_antigeometric_even_grade;

  Bivector, Multivector => Multivector: bivector_antigeometric_multivector;
  Bivector, Scalar => Bivector: bivector_antigeometric_scalar;
  Bivector, Vector => OddGrade: bivector_antigeometric_vector;
  Bivector, Bivector => EvenGrade: bivector_antigeometric_bivector;
  Bivector, Trivector => OddGrade: bivector_antigeometric_trivector;
  Bivector, Antiscalar => Bivector: bivector_antigeometric_antiscalar;
  Bivector, DualNumber => Bivector: bivector_antigeometric_dual_number;
  Bivector, OddGrade => OddGrade: bivector_antigeometric_odd_grade;
  Bivector, EvenGrade => EvenGrade: bivector_antigeometric_even_grade;

  Trivector, Multivector => Multivector: trivector_antigeometric_multivector;
  Trivector, Scalar => Vector: trivector_antigeometric_scalar;
  Trivector, Vector => EvenGrade: trivector_antigeometric_vector;
  Trivector, Bivector => OddGrade: trivector_antigeometric_bivector;
  Trivector, Trivector => EvenGrade: trivector_antigeometric_trivector;
  Trivector, Antiscalar => Trivector: trivector_antigeometric_antiscalar;
  Trivector, DualNumber => OddGrade: trivector_antigeometric_dual_number;
  Trivector, OddGrade => EvenGrade: trivector_antigeometric_odd_grade;
  Trivector, EvenGrade => OddGrade: trivector_antigeometric_even_grade;

  Antiscalar, Multivector => Multivector: antiscalar_antigeometric_multivector;
  Antiscalar, Scalar => Scalar: antiscalar_antigeometric_scalar;
  Antiscalar, Vector => Vector: antiscalar_antigeometric_vector;
  Antiscalar, Bivector => Bivector: antiscalar_antigeometric_bivector;
  Antiscalar, Trivector => Trivector: antiscalar_antigeometric_trivector;
  Antiscalar, Antiscalar => Antiscalar: antiscalar_antigeometric_antiscalar;
  Antiscalar, DualNumber => DualNumber: antiscalar_antigeometric_dual_number;
  Antiscalar, OddGrade => OddGrade: antiscalar_antigeometric_odd_grade;
  Antiscalar, EvenGrade => EvenGrade: antiscalar_antigeometric_even_grade;

  DualNumber, Multivector => Multivector: dual_number_antigeometric_multivector;
  DualNumber, Scalar => Scalar: dual_number_antigeometric_scalar;
  DualNumber, Vector => OddGrade: dual_number_antigeometric_vector;
  DualNumber, Bivector => Bivector: dual_number_antigeometric_bivector;
  DualNumber, Trivector => OddGrade: dual_number_antigeometric_trivector;
  DualNumber, Antiscalar => DualNumber: dual_number_antigeometric_antiscalar;
  DualNumber, DualNumber => DualNumber: dual_number_antigeometric_dual_number;
  DualNumber, OddGrade => OddGrade: dual_number_antigeometric_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_antigeometric_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_antigeometric_multivector;
  OddGrade, Scalar => OddGrade: odd_grade_antigeometric_scalar;
  OddGrade, Vector => EvenGrade: odd_grade_antigeometric_vector;
  OddGrade, Bivector => OddGrade: odd_grade_antigeometric_bivector;
  OddGrade, Trivector => EvenGrade: odd_grade_antigeometric_trivector;
  OddGrade, Antiscalar => OddGrade: odd_grade_antigeometric_antiscalar;
  OddGrade, DualNumber => OddGrade: odd_grade_antigeometric_dual_number;
  OddGrade, OddGrade => EvenGrade: odd_grade_antigeometric_odd_grade;
  OddGrade, EvenGrade => OddGrade: odd_grade_antigeometric_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_antigeometric_multivector;
  EvenGrade, Scalar => EvenGrade: even_grade_antigeometric_scalar;
  EvenGrade, Vector => OddGrade: even_grade_antigeometric_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_antigeometric_bivector;
  EvenGrade, Trivector => OddGrade: even_grade_antigeometric_trivector;
  EvenGrade, Antiscalar => EvenGrade: even_grade_antigeometric_antiscalar;
  EvenGrade, DualNumber => EvenGrade: even_grade_antigeometric_dual_number;
  EvenGrade, OddGrade => OddGrade: even_grade_antigeometric_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_antigeometric_even_grade;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_antigeometric_multivector(
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

  let s = (l1*r423 - l423*r1) + (l2*r431 - l431*r2)
    + (l3*r412 - l412*r3) + (l4*r321 - l321*r4)
    - (l41*r23 + l23*r41) - (l42*r31 + l31*r42)
    - (l43*r12 + l12*r43) + (ls*r1234 + l1234*rs);

  let e1 = (l1*r1234 + l1234*r1) + (l41*r321 + l321*r41)
    + (l31*r412 + l412*r31) - (l12*r431 + l431*r12)
    + (ls*r423 - l423*rs) + (l2*r43 - l43*r2)
    + (l42*r3 - l3*r42) + (l23*r4 - l4*r23);
  let e2 = (l2*r1234 + l1234*r2) + (l42*r321 + l321*r42)
    + (l12*r423 + l423*r12) - (l23*r412 + l412*r23)
    + (ls*r431 - l431*rs) + (l3*r41 - l41*r3)
    + (l43*r1 - l1*r43) + (l31*r4 - l4*r31);
  let e3 = (l3*r1234 + l1234*r3) + (l43*r321 + l321*r43)
    + (l23*r431 + l431*r23) - (l31*r423 + l423*r31)
    + (ls*r412 - l412*rs) + (l1*r42 - l42*r1)
    + (l41*r2 - l2*r41) + (l12*r4 - l4*r12);
  let e4 = (l4*r1234 + l1234*r4) - (l41*r423 + l423*r41)
    - (l42*r431 + l431*r42) - (l43*r412 + l412*r43);

  let e41 = (l41*r1234 + l1234*r41) - (l4*r423 + l423*r4)
    + (l42*r43 - l43*r42) + (l412*r431 - l431*r412);
  let e42 = (l42*r1234 + l1234*r42) - (l4*r431 + l431*r4)
    + (l43*r41 - l41*r43) + (l423*r412 - l412*r423);
  let e43 = (l43*r1234 + l1234*r43) - (l4*r412 + l412*r4)
    + (l41*r42 - l42*r41) + (l431*r423 - l423*r431);
  let e23 = (l23*r1234 + l1234*r23) + (ls*r41 + l41*rs)
    + (l3*r431 + l431*r3) - (l2*r412 + l412*r2)
    + (l4*r1 - l1*r4) + (l31*r43 - l43*r31)
    + (l42*r12 - l12*r42) + (l423*r321 - l321*r423);
  let e31 = (l31*r1234 + l1234*r31) + (ls*r42 + l42*rs)
    + (l1*r412 + l412*r1) - (l3*r423 + l423*r3)
    + (l4*r2 - l2*r4) + (l12*r41 - l41*r12)
    + (l43*r23 - l23*r43) + (l431*r321 - l321*r431);
  let e12 = (l12*r1234 + l1234*r12) + (ls*r43 + l43*rs)
    + (l2*r423 + l423*r2) - (l1*r431 + l431*r1)
    + (l4*r3 - l3*r4) + (l23*r42 - l42*r23)
    + (l41*r31 - l31*r41) + (l412*r321 - l321*r412);

  let e423 = (l423*r1234 + l1234*r423) + (l4*r41 + l41*r4)
    + (l431*r43 - l43*r431) + (l42*r412 - l412*r42);
  let e431 = (l431*r1234 + l1234*r431) + (l4*r42 + l42*r4)
    + (l412*r41 - l41*r412) + (l43*r423 - l423*r43);
  let e412 = (l412*r1234 + l1234*r412) + (l4*r43 + l43*r4)
    + (l423*r42 - l42*r423) + (l41*r431 - l431*r41);
  let e321 = (l321*r1234 + l1234*r321) - (l1*r41 + l41*r1)
    - (l2*r42 + l42*r2) - (l3*r43 + l43*r3)
    + (ls*r4 - l4*rs) + (l423*r23 - l23*r423)
    + (l431*r31 - l31*r431) + (l412*r12 - l12*r412);

  let e1234 = - l4*r4 - l41*r41 - l42*r42 - l43*r43
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
fn multivector_antigeometric_scalar(
  Multivector {
    e4: l4,
    e41: l41, e42: l42, e43: l43,
    e423: l423, e431: l431, e412: l412,
    e1234: l1234,
    ..
  }: Multivector,
  Scalar { s: rs }: Scalar,
) -> Multivector {

  let s = l1234*rs;

  let e1 = -l423*rs;
  let e2 = -l431*rs;
  let e3 = -l412*rs;

  let e23 = l41*rs;
  let e31 = l42*rs;
  let e12 = l43*rs;

  let e321 = -l4*rs;

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
fn multivector_antigeometric_vector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Multivector {

  let s = -l423*r1 - l431*r2 - l412*r3 - l321*r4;

  let e1 = l1234*r1 - l43*r2 + l42*r3 + l23*r4;
  let e2 = l1234*r2 - l41*r3 + l43*r1 + l31*r4;
  let e3 = l1234*r3 - l42*r1 + l41*r2 + l12*r4;
  let e4 = l1234*r4;

  let e41 = - l423*r4;
  let e42 = - l431*r4;
  let e43 = - l412*r4;
  let e23 = l431*r3 - l412*r2 + (l4*r1 - l1*r4);
  let e31 = l412*r1 - l423*r3 + (l4*r2 - l2*r4);
  let e12 = l423*r2 - l431*r1 + (l4*r3 - l3*r4);

  let e423 = l41*r4;
  let e431 = l42*r4;
  let e412 = l43*r4;
  let e321 = -l41*r1 - l42*r2 - l43*r3 + ls*r4;

  let e1234 = -l4*r4;

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
fn multivector_antigeometric_bivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Multivector {

  let s = -(l43*r12 + l12*r43) - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e1 = l321*r41 + l412*r31 - l431*r12 + l2*r43 - l3*r42 - l4*r23;
  let e2 = l321*r42 + l423*r12 - l412*r23 + l3*r41 - l1*r43 - l4*r31;
  let e3 = l321*r43 + l431*r23 - l423*r31 + l1*r42 - l2*r41 - l4*r12;
  let e4 = -l423*r41 - l431*r42 - l412*r43;

  let e41 = l1234*r41 + (l42*r43 - l43*r42);
  let e42 = l1234*r42 + (l43*r41 - l41*r43);
  let e43 = l1234*r43 + (l41*r42 - l42*r41);
  let e23 = l1234*r23 + ls*r41 + (l31*r43 - l43*r31) + (l42*r12 - l12*r42);
  let e31 = l1234*r31 + ls*r42 + (l12*r41 - l41*r12) + (l43*r23 - l23*r43);
  let e12 = l1234*r12 + ls*r43 + (l23*r42 - l42*r23) + (l41*r31 - l31*r41);

  let e423 = l4*r41 + l431*r43 - l412*r42;
  let e431 = l4*r42 + l412*r41 - l423*r43;
  let e412 = l4*r43 + l423*r42 - l431*r41;
  let e321 = - l1*r41 - l2*r42 - l3*r43 + l423*r23 + l431*r31 + l412*r12;

  let e1234 = - l41*r41 - l42*r42 - l43*r43;

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
fn multivector_antigeometric_trivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Multivector {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  let e1 = l41*r321 + l31*r412 - l12*r431 + ls*r423;
  let e2 = l42*r321 + l12*r423 - l23*r412 + ls*r431;
  let e3 = l43*r321 + l23*r431 - l31*r423 + ls*r412;
  let e4 = -l41*r423 - l42*r431 - l43*r412;

  let e41 = -l4*r423 + (l412*r431 - l431*r412);
  let e42 = -l4*r431 + (l423*r412 - l412*r423);
  let e43 = -l4*r412 + (l431*r423 - l423*r431);
  let e23 = l3*r431 - l2*r412 + (l423*r321 - l321*r423);
  let e31 = l1*r412 - l3*r423 + (l431*r321 - l321*r431);
  let e12 = l2*r423 - l1*r431 + (l412*r321 - l321*r412);

  let e423 = l1234*r423 - l43*r431 + l42*r412;
  let e431 = l1234*r431 - l41*r412 + l43*r423;
  let e412 = l1234*r412 - l42*r423 + l41*r431;
  let e321 = l1234*r321 - l23*r423 - l31*r431 - l12*r412;

  let e1234 = l423*r423 + l431*r431 + l412*r412;

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
fn multivector_antigeometric_antiscalar(
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
fn multivector_antigeometric_dual_number(
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

  let e1 = l1*r1234 - l423*rs;
  let e2 = l2*r1234 - l431*rs;
  let e3 = l3*r1234 - l412*rs;
  let e4 = l4*r1234;

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234 + l41*rs;
  let e31 = l31*r1234 + l42*rs;
  let e12 = l12*r1234 + l43*rs;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234 - l4*rs;

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
fn multivector_antigeometric_odd_grade(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> Multivector {

  let s = (l1*r423 - l423*r1) + (l2*r431 - l431*r2)
    + (l3*r412 - l412*r3) + (l4*r321 - l321*r4);

  let e1 = l1234*r1 + l41*r321 + l31*r412 - l12*r431
    + ls*r423 - l43*r2 + l42*r3 + l23*r4;
  let e2 = l1234*r2 + l42*r321 + l12*r423 - l23*r412
    + ls*r431 - l41*r3 + l43*r1 + l31*r4;
  let e3 = l1234*r3 + l43*r321 + l23*r431 - l31*r423
    + ls*r412 - l42*r1 + l41*r2 + l12*r4;
  let e4 = l1234*r4 - l41*r423 - l42*r431 - l43*r412;

  let e41 = -(l4*r423 + l423*r4) + (l412*r431 - l431*r412);
  let e42 = -(l4*r431 + l431*r4) + (l423*r412 - l412*r423);
  let e43 = -(l4*r412 + l412*r4) + (l431*r423 - l423*r431);
  let e23 = (l3*r431 + l431*r3) - (l2*r412 + l412*r2)
    + (l4*r1 - l1*r4) + (l423*r321 - l321*r423);
  let e31 = (l1*r412 + l412*r1) - (l3*r423 + l423*r3)
    + (l4*r2 - l2*r4) + (l431*r321 - l321*r431);
  let e12 = (l2*r423 + l423*r2) - (l1*r431 + l431*r1)
    + (l4*r3 - l3*r4) + (l412*r321 - l321*r412);

  let e423 = l1234*r423 + l41*r4 - l43*r431 + l42*r412;
  let e431 = l1234*r431 + l42*r4 - l41*r412 + l43*r423;
  let e412 = l1234*r412 + l43*r4 - l42*r423 + l41*r431;
  let e321 = l1234*r321 - l41*r1 - l42*r2 - l43*r3
    + ls*r4 - l23*r423 - l31*r431 - l12*r412;

  let e1234 = - l4*r4 + l423*r423 + l431*r431 + l412*r412;

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
fn multivector_antigeometric_even_grade(
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

  let e1 = l1*r1234 + l321*r41 + l412*r31 - l431*r12
    - l423*rs + l2*r43 - l3*r42 - l4*r23;
  let e2 = l2*r1234 + l321*r42 + l423*r12 - l412*r23
    - l431*rs + l3*r41 - l1*r43 - l4*r31;
  let e3 = l3*r1234 + l321*r43 + l431*r23 - l423*r31
    - l412*rs + l1*r42 - l2*r41 - l4*r12;
  let e4 = l4*r1234 - l423*r41 - l431*r42 - l412*r43;

  let e41 = (l41*r1234 + l1234*r41) + (l42*r43 - l43*r42);
  let e42 = (l42*r1234 + l1234*r42) + (l43*r41 - l41*r43);
  let e43 = (l43*r1234 + l1234*r43) + (l41*r42 - l42*r41);
  let e23 = (l23*r1234 + l1234*r23) + (ls*r41 + l41*rs)
    + (l31*r43 - l43*r31) + (l42*r12 - l12*r42);
  let e31 = (l31*r1234 + l1234*r31) + (ls*r42 + l42*rs)
    + (l12*r41 - l41*r12) + (l43*r23 - l23*r43);
  let e12 = (l12*r1234 + l1234*r12) + (ls*r43 + l43*rs)
    + (l23*r42 - l42*r23) + (l41*r31 - l31*r41);

  let e423 = l423*r1234 + l4*r41 + l431*r43 - l412*r42;
  let e431 = l431*r1234 + l4*r42 + l412*r41 - l423*r43;
  let e412 = l412*r1234 + l4*r43 + l423*r42 - l431*r41;
  let e321 = l321*r1234 - l1*r41 - l2*r42 - l3*r43
    - l4*rs + l423*r23 + l431*r31 + l412*r12;

  let e1234 = -l41*r41 - l42*r42 - l43*r43 + l1234*r1234;

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
fn scalar_antigeometric_multivector(
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

  let e1 = ls*r423;
  let e2 = ls*r431;
  let e3 = ls*r412;

  let e23 = ls*r41;
  let e31 = ls*r42;
  let e12 = ls*r43;

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
fn scalar_antigeometric_vector(
  Scalar { s: ls }: Scalar,
  Vector { e4: r4, .. }: Vector,
) -> Trivector {

  let e321 = ls*r4;

  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_antigeometric_bivector(
  Scalar { s: ls }: Scalar,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Bivector {

  let e23 = ls*r41;
  let e31 = ls*r42;
  let e12 = ls*r43;

  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_antigeometric_trivector(
  Scalar { s: ls }: Scalar,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Vector {

  let e1 = ls*r423;
  let e2 = ls*r431;
  let e3 = ls*r412;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn scalar_antigeometric_antiscalar(
  Scalar { s: ls }: Scalar,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Scalar {

  let s = ls*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_antigeometric_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Scalar {

  let s = ls*r1234;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_antigeometric_odd_grade(
  Scalar { s: ls }: Scalar,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> OddGrade {

  let e1 = ls*r423;
  let e2 = ls*r431;
  let e3 = ls*r412;

  let e321 = ls*r4;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_antigeometric_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> Multivector {

  let s = ls*r1234;

  let e23 = ls*r41;
  let e31 = ls*r42;
  let e12 = ls*r43;

  Multivector {
    s,
    e23, e31, e12,
    ..zero()
  }
}

// vector

#[rustfmt::skip]
#[inline]
fn vector_antigeometric_multivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  let e1 = l1*r1234 + l2*r43 - l3*r42 - l4*r23;
  let e2 = l2*r1234 + l3*r41 - l1*r43 - l4*r31;
  let e3 = l3*r1234 + l1*r42 - l2*r41 - l4*r12;
  let e4 = l4*r1234;

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = l3*r431 - l2*r412 + (l4*r1 - l1*r4);
  let e31 = l1*r412 - l3*r423 + (l4*r2 - l2*r4);
  let e12 = l2*r423 - l1*r431 + (l4*r3 - l3*r4);

  let e423 = l4*r41;
  let e431 = l4*r42;
  let e412 = l4*r43;
  let e321 = -l1*r41 - l2*r42 - l3*r43 - l4*rs;

  let e1234 = -l4*r4;

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
fn vector_antigeometric_scalar(
  Vector { e4: l4, .. }: Vector,
  Scalar { s: rs }: Scalar,
) -> Trivector {

  let e321 = -l4*rs;

  Trivector { e321, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn vector_antigeometric_vector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> EvenGrade {

  let e23 = l4*r1 - l1*r4;
  let e31 = l4*r2 - l2*r4;
  let e12 = l4*r3 - l3*r4;

  let e1234 = -l4*r4;

  EvenGrade {
    e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_antigeometric_bivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> OddGrade {

  let e1 = l2*r43 - l3*r42 - l4*r23;
  let e2 = l3*r41 - l1*r43 - l4*r31;
  let e3 = l1*r42 - l2*r41 - l4*r12;

  let e423 = l4*r41;
  let e431 = l4*r42;
  let e412 = l4*r43;
  let e321 = -l1*r41 - l2*r42 - l3*r43;

  OddGrade {
    e1, e2, e3,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_antigeometric_trivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> EvenGrade {

  let s = l1*r423
        + l2*r431
        + l3*r412
        + l4*r321;

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = l3*r431 - l2*r412;
  let e31 = l1*r412 - l3*r423;
  let e12 = l2*r423 - l1*r431;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_antigeometric_antiscalar(
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
fn vector_antigeometric_dual_number(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> OddGrade {

  let e1 = l1*r1234;
  let e2 = l2*r1234;
  let e3 = l3*r1234;
  let e4 = l4*r1234;

  let e321 = -l4*rs;

  OddGrade {
    e1, e2, e3, e4,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_antigeometric_odd_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> EvenGrade {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  let e41 = -l4*r423;
  let e42 = -l4*r431;
  let e43 = -l4*r412;
  let e23 = l3*r431 - l2*r412 + (l4*r1 - l1*r4);
  let e31 = l1*r412 - l3*r423 + (l4*r2 - l2*r4);
  let e12 = l2*r423 - l1*r431 + (l4*r3 - l3*r4);

  let e1234 = -l4*r4;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_antigeometric_even_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*r1234 + l2*r43 - l3*r42 - l4*r23;
  let e2 = l2*r1234 + l3*r41 - l1*r43 - l4*r31;
  let e3 = l3*r1234 + l1*r42 - l2*r41 - l4*r12;
  let e4 = l4*r1234;

  let e423 = l4*r41;
  let e431 = l4*r42;
  let e412 = l4*r43;
  let e321 = -l1*r41 - l2*r42 - l3*r43 - l4*rs;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_antigeometric_multivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = -(l43*r12 + l12*r43) - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e1 = l41*r321 + l31*r412 - l12*r431 - l43*r2 + l42*r3 + l23*r4;
  let e2 = l42*r321 + l12*r423 - l23*r412 - l41*r3 + l43*r1 + l31*r4;
  let e3 = l43*r321 + l23*r431 - l31*r423 - l42*r1 + l41*r2 + l12*r4;
  let e4 = -l41*r423 - l42*r431 - l43*r412;

  let e41 = l41*r1234 + (l42*r43 - l43*r42);
  let e42 = l42*r1234 + (l43*r41 - l41*r43);
  let e43 = l43*r1234 + (l41*r42 - l42*r41);
  let e23 = l23*r1234 + l41*rs + (l31*r43 - l43*r31) + (l42*r12 - l12*r42);
  let e31 = l31*r1234 + l42*rs + (l12*r41 - l41*r12) + (l43*r23 - l23*r43);
  let e12 = l12*r1234 + l43*rs + (l23*r42 - l42*r23) + (l41*r31 - l31*r41);

  let e423 = l41*r4 - l43*r431 + l42*r412;
  let e431 = l42*r4 - l41*r412 + l43*r423;
  let e412 = l43*r4 - l42*r423 + l41*r431;
  let e321 = -l41*r1 - l42*r2 - l43*r3 - l23*r423 - l31*r431 - l12*r412;

  let e1234 = -l41*r41 - l42*r42 - l43*r43;

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
fn bivector_antigeometric_scalar(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  Scalar { s: rs }: Scalar,
) -> Bivector {

  let e23 = l41*rs;
  let e31 = l42*rs;
  let e12 = l43*rs;

  Bivector { e23, e31, e12, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bivector_antigeometric_vector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> OddGrade {

  let e1 = -l43*r2 + l42*r3 + l23*r4;
  let e2 = -l41*r3 + l43*r1 + l31*r4;
  let e3 = -l42*r1 + l41*r2 + l12*r4;

  let e423 = l41*r4;
  let e431 = l42*r4;
  let e412 = l43*r4;
  let e321 = -l41*r1 - l42*r2 - l43*r3;

  OddGrade {
    e1, e2, e3,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_antigeometric_bivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> EvenGrade {

  let s = -(l43*r12 + l12*r43) - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e41 = l42*r43 - l43*r42;
  let e42 = l43*r41 - l41*r43;
  let e43 = l41*r42 - l42*r41;
  let e23 = (l31*r43 - l43*r31) + (l42*r12 - l12*r42);
  let e31 = (l12*r41 - l41*r12) + (l43*r23 - l23*r43);
  let e12 = (l23*r42 - l42*r23) + (l41*r31 - l31*r41);

  let e1234 = -l41*r41 - l42*r42 - l43*r43;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_antigeometric_trivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> OddGrade {

  let e1 = l41*r321 + l31*r412 - l12*r431;
  let e2 = l42*r321 + l12*r423 - l23*r412;
  let e3 = l43*r321 + l23*r431 - l31*r423;
  let e4 = -l41*r423 - l42*r431 - l43*r412;

  let e423 = -l43*r431 + l42*r412;
  let e431 = -l41*r412 + l43*r423;
  let e412 = -l42*r423 + l41*r431;
  let e321 = -l23*r423 - l31*r431 - l12*r412;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_antigeometric_antiscalar(
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
fn bivector_antigeometric_dual_number(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> Bivector {

  let e41 = l41*r1234;
  let e42 = l42*r1234;
  let e43 = l43*r1234;
  let e23 = l23*r1234 + l41*rs;
  let e31 = l31*r1234 + l42*rs;
  let e12 = l12*r1234 + l43*rs;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn bivector_antigeometric_odd_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l41*r321 + l31*r412 - l12*r431 - l43*r2 + l42*r3 + l23*r4;
  let e2 = l42*r321 + l12*r423 - l23*r412 - l41*r3 + l43*r1 + l31*r4;
  let e3 = l43*r321 + l23*r431 - l31*r423 - l42*r1 + l41*r2 + l12*r4;
  let e4 = -l41*r423 - l42*r431 - l43*r412;

  let e423 = l41*r4 - l43*r431 + l42*r412;
  let e431 = l42*r4 - l41*r412 + l43*r423;
  let e412 = l43*r4 - l42*r423 + l41*r431;
  let e321 = -l41*r1 - l42*r2 - l43*r3 - l23*r423 - l31*r431 - l12*r412;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_antigeometric_even_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = -(l43*r12 + l12*r43) - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e41 = l41*r1234 + (l42*r43 - l43*r42);
  let e42 = l42*r1234 + (l43*r41 - l41*r43);
  let e43 = l43*r1234 + (l41*r42 - l42*r41);
  let e23 = l23*r1234 + l41*rs + (l31*r43 - l43*r31) + (l42*r12 - l12*r42);
  let e31 = l31*r1234 + l42*rs + (l12*r41 - l41*r12) + (l43*r23 - l23*r43);
  let e12 = l12*r1234 + l43*rs + (l23*r42 - l42*r23) + (l41*r31 - l31*r41);

  let e1234 = -l41*r41 - l42*r42 - l43*r43;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_antigeometric_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = -l423*r1 - l431*r2 - l412*r3 - l321*r4;

  let e1 = l321*r41 + l412*r31 - l431*r12 - l423*rs;
  let e2 = l321*r42 + l423*r12 - l412*r23 - l431*rs;
  let e3 = l321*r43 + l431*r23 - l423*r31 - l412*rs;
  let e4 = -l423*r41 - l431*r42 - l412*r43;

  let e41 = -l423*r4 + (l412*r431 - l431*r412);
  let e42 = -l431*r4 + (l423*r412 - l412*r423);
  let e43 = -l412*r4 + (l431*r423 - l423*r431);
  let e23 = l431*r3 - l412*r2 + (l423*r321 - l321*r423);
  let e31 = l412*r1 - l423*r3 + (l431*r321 - l321*r431);
  let e12 = l423*r2 - l431*r1 + (l412*r321 - l321*r412);

  let e423 = l423*r1234 + l431*r43 - l412*r42;
  let e431 = l431*r1234 + l412*r41 - l423*r43;
  let e412 = l412*r1234 + l423*r42 - l431*r41;
  let e321 = l321*r1234 + l423*r23 + l431*r31 + l412*r12;

  let e1234 = l423*r423 + l431*r431 + l412*r412;

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
fn trivector_antigeometric_scalar(
  Trivector { e423: l423, e431: l431, e412: l412, ..  }: Trivector,
  Scalar { s: rs }: Scalar,
) -> Vector {

  let e1 = -l423*rs;
  let e2 = -l431*rs;
  let e3 = -l412*rs;

  Vector { e1, e2, e3, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn trivector_antigeometric_vector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> EvenGrade {

  let s = -l423*r1
         - l431*r2
         - l412*r3
         - l321*r4;

  let e41 = -l423*r4;
  let e42 = -l431*r4;
  let e43 = -l412*r4;
  let e23 = l431*r3
          - l412*r2;
  let e31 = l412*r1
          - l423*r3;
  let e12 = l423*r2
          - l431*r1;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_antigeometric_bivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> OddGrade {

  let e1 = l321*r41 + l412*r31 - l431*r12;
  let e2 = l321*r42 + l423*r12 - l412*r23;
  let e3 = l321*r43 + l431*r23 - l423*r31;
  let e4 = -l423*r41 - l431*r42 - l412*r43;

  let e423 = l431*r43 - l412*r42;
  let e431 = l412*r41 - l423*r43;
  let e412 = l423*r42 - l431*r41;
  let e321 = l423*r23 + l431*r31 + l412*r12;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_antigeometric_trivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> EvenGrade {

  let e41 = l412*r431 - l431*r412;
  let e42 = l423*r412 - l412*r423;
  let e43 = l431*r423 - l423*r431;
  let e23 = l423*r321 - l321*r423;
  let e31 = l431*r321 - l321*r431;
  let e12 = l412*r321 - l321*r412;

  let e1234 = l423*r423 + l431*r431 + l412*r412;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_antigeometric_antiscalar(
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
fn trivector_antigeometric_dual_number(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> OddGrade {

  let e1 = -l423*rs;
  let e2 = -l431*rs;
  let e3 = -l412*rs;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234;

  OddGrade {
    e1, e2, e3,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_antigeometric_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> EvenGrade {

  let s = -l423*r1 - l431*r2 - l412*r3 - l321*r4;

  let e41 = -l423*r4 + (l412*r431 - l431*r412);
  let e42 = -l431*r4 + (l423*r412 - l412*r423);
  let e43 = -l412*r4 + (l431*r423 - l423*r431);
  let e23 = l431*r3 - l412*r2 + (l423*r321 - l321*r423);
  let e31 = l412*r1 - l423*r3 + (l431*r321 - l321*r431);
  let e12 = l423*r2 - l431*r1 + (l412*r321 - l321*r412);

  let e1234 = l423*r423 + l431*r431 + l412*r412;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_antigeometric_even_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> OddGrade {

  let e1 = l321*r41 + l412*r31 - l431*r12 - l423*rs;
  let e2 = l321*r42 + l423*r12 - l412*r23 - l431*rs;
  let e3 = l321*r43 + l431*r23 - l423*r31 - l412*rs;
  let e4 = -l423*r41 - l431*r42 - l412*r43;

  let e423 = l423*r1234 + l431*r43 - l412*r42;
  let e431 = l431*r1234 + l412*r41 - l423*r43;
  let e412 = l412*r1234 + l423*r42 - l431*r41;
  let e321 = l321*r1234 + l423*r23 + l431*r31 + l412*r12;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// antiscalar

#[rustfmt::skip]
#[inline]
fn antiscalar_antigeometric_multivector(
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
fn antiscalar_antigeometric_scalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Scalar { s: rs }: Scalar,
) -> Scalar {

  let s = l1234*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antigeometric_vector(
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
fn antiscalar_antigeometric_bivector(
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
fn antiscalar_antigeometric_trivector(
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
fn antiscalar_antigeometric_antiscalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Antiscalar {

  let e1234 = l1234*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antigeometric_dual_number(
  Antiscalar { e1234: l1234 }: Antiscalar,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let s = l1234*rs;
  let e1234 = l1234*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antigeometric_odd_grade(
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
fn antiscalar_antigeometric_even_grade(
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

// dual number

#[rustfmt::skip]
#[inline]
fn dual_number_antigeometric_multivector(
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

  let e1 = l1234*r1 + ls*r423;
  let e2 = l1234*r2 + ls*r431;
  let e3 = l1234*r3 + ls*r412;
  let e4 = l1234*r4;

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23 + ls*r41;
  let e31 = l1234*r31 + ls*r42;
  let e12 = l1234*r12 + ls*r43;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321 + ls*r4;

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
fn dual_number_antigeometric_scalar(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Scalar { s: rs }: Scalar,
) -> Scalar {

  let s = l1234*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antigeometric_vector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> OddGrade {

  let e1 = l1234*r1;
  let e2 = l1234*r2;
  let e3 = l1234*r3;
  let e4 = l1234*r4;

  let e321 = ls*r4;

  OddGrade {
    e1, e2, e3, e4,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antigeometric_bivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Bivector {

  let e41 = l1234*r41;
  let e42 = l1234*r42;
  let e43 = l1234*r43;
  let e23 = l1234*r23 + ls*r41;
  let e31 = l1234*r31 + ls*r42;
  let e12 = l1234*r12 + ls*r43;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antigeometric_trivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> OddGrade {

  let e1 = ls*r423;
  let e2 = ls*r431;
  let e3 = ls*r412;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321;

  OddGrade {
    e1, e2, e3,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antigeometric_antiscalar(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> DualNumber {

  let s = ls*r1234;
  let e1234 = l1234*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antigeometric_dual_number(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let s = ls*r1234 + l1234*rs;
  let e1234 = l1234*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antigeometric_odd_grade(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l1234*r1 + ls*r423;
  let e2 = l1234*r2 + ls*r431;
  let e3 = l1234*r3 + ls*r412;
  let e4 = l1234*r4;

  let e423 = l1234*r423;
  let e431 = l1234*r431;
  let e412 = l1234*r412;
  let e321 = l1234*r321 + ls*r4;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antigeometric_even_grade(
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
  let e23 = l1234*r23 + ls*r41;
  let e31 = l1234*r31 + ls*r42;
  let e12 = l1234*r12 + ls*r43;

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
fn odd_grade_antigeometric_multivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = (l1*r423 - l423*r1) + (l2*r431 - l431*r2)
    + (l3*r412 - l412*r3) + (l4*r321 - l321*r4);

  let e1 = l1*r1234 + l321*r41 + l412*r31 - l431*r12
    - l423*rs + l2*r43 - l3*r42 - l4*r23;
  let e2 = l2*r1234 + l321*r42 + l423*r12 - l412*r23
    - l431*rs + l3*r41 - l1*r43 - l4*r31;
  let e3 = l3*r1234 + l321*r43 + l431*r23 - l423*r31
    - l412*rs + l1*r42 - l2*r41 - l4*r12;
  let e4 = l4*r1234 - l423*r41 - l431*r42 - l412*r43;

  let e41 = -(l4*r423 + l423*r4) + (l412*r431 - l431*r412);
  let e42 = -(l4*r431 + l431*r4) + (l423*r412 - l412*r423);
  let e43 = -(l4*r412 + l412*r4) + (l431*r423 - l423*r431);
  let e23 = (l3*r431 + l431*r3) - (l2*r412 + l412*r2)
    + (l4*r1 - l1*r4) + (l423*r321 - l321*r423);
  let e31 = (l1*r412 + l412*r1) - (l3*r423 + l423*r3)
    + (l4*r2 - l2*r4) + (l431*r321 - l321*r431);
  let e12 = (l2*r423 + l423*r2) - (l1*r431 + l431*r1)
    + (l4*r3 - l3*r4) + (l412*r321 - l321*r412);

  let e423 = l423*r1234 + l4*r41 + l431*r43 - l412*r42;
  let e431 = l431*r1234 + l4*r42 + l412*r41 - l423*r43;
  let e412 = l412*r1234 + l4*r43 + l423*r42 - l431*r41;
  let e321 = l321*r1234 - l1*r41 - l2*r42 - l3*r43
    - l4*rs + l423*r23 + l431*r31 + l412*r12;

  let e1234 = - l4*r4 + l423*r423 + l431*r431 + l412*r412;

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
fn odd_grade_antigeometric_scalar(
  OddGrade {
    e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: OddGrade,
  Scalar { s: rs }: Scalar,
) -> OddGrade {

  let e1 = -l423*rs;
  let e2 = -l431*rs;
  let e3 = -l412*rs;

  let e321 = -l4*rs;

  OddGrade {
    e1, e2, e3,
    e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antigeometric_vector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> EvenGrade {

  let s = -l423*r1 - l431*r2 - l412*r3 - l321*r4;

  let e41 = -l423*r4;
  let e42 = -l431*r4;
  let e43 = -l412*r4;
  let e23 = l431*r3 - l412*r2 + (l4*r1 - l1*r4);
  let e31 = l412*r1 - l423*r3 + (l4*r2 - l2*r4);
  let e12 = l423*r2 - l431*r1 + (l4*r3 - l3*r4);

  let e1234 = -l4*r4;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antigeometric_bivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> OddGrade {

  let e1 = l321*r41 + l412*r31 - l431*r12 + l2*r43 - l3*r42 - l4*r23;
  let e2 = l321*r42 + l423*r12 - l412*r23 + l3*r41 - l1*r43 - l4*r31;
  let e3 = l321*r43 + l431*r23 - l423*r31 + l1*r42 - l2*r41 - l4*r12;
  let e4 = -l423*r41 - l431*r42 - l412*r43;

  let e423 = l4*r41 + l431*r43 - l412*r42;
  let e431 = l4*r42 + l412*r41 - l423*r43;
  let e412 = l4*r43 + l423*r42 - l431*r41;
  let e321 = -l1*r41 - l2*r42 - l3*r43 + l423*r23 + l431*r31 + l412*r12;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antigeometric_trivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> EvenGrade {

  let s = l1*r423 + l2*r431 + l3*r412 + l4*r321;

  let e41 = -l4*r423 + (l412*r431 - l431*r412);
  let e42 = -l4*r431 + (l423*r412 - l412*r423);
  let e43 = -l4*r412 + (l431*r423 - l423*r431);
  let e23 = l3*r431 - l2*r412 + (l423*r321 - l321*r423);
  let e31 = l1*r412 - l3*r423 + (l431*r321 - l321*r431);
  let e12 = l2*r423 - l1*r431 + (l412*r321 - l321*r412);

  let e1234 = l423*r423 + l431*r431 + l412*r412;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antigeometric_antiscalar(
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
fn odd_grade_antigeometric_dual_number(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> OddGrade {

  let e1 = l1*r1234 - l423*rs;
  let e2 = l2*r1234 - l431*rs;
  let e3 = l3*r1234 - l412*rs;
  let e4 = l4*r1234;

  let e423 = l423*r1234;
  let e431 = l431*r1234;
  let e412 = l412*r1234;
  let e321 = l321*r1234 - l4*rs;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antigeometric_odd_grade(
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

  let e41 = -(l4*r423 + l423*r4) + (l412*r431 - l431*r412);
  let e42 = -(l4*r431 + l431*r4) + (l423*r412 - l412*r423);
  let e43 = -(l4*r412 + l412*r4) + (l431*r423 - l423*r431);
  let e23 = (l3*r431 + l431*r3) - (l2*r412 + l412*r2)
    + (l4*r1 - l1*r4) + (l423*r321 - l321*r423);
  let e31 = (l1*r412 + l412*r1) - (l3*r423 + l423*r3)
    + (l4*r2 - l2*r4) + (l431*r321 - l321*r431);
  let e12 = (l2*r423 + l423*r2) - (l1*r431 + l431*r1)
    + (l4*r3 - l3*r4) + (l412*r321 - l321*r412);

  let e1234 = - l4*r4 + l423*r423 + l431*r431 + l412*r412;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antigeometric_even_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*r1234 + l321*r41 + l412*r31 - l431*r12
    - l423*rs + l2*r43 - l3*r42 - l4*r23;
  let e2 = l2*r1234 + l321*r42 + l423*r12 - l412*r23
    - l431*rs + l3*r41 - l1*r43 - l4*r31;
  let e3 = l3*r1234 + l321*r43 + l431*r23 - l423*r31
    - l412*rs + l1*r42 - l2*r41 - l4*r12;
  let e4 = l4*r1234 - l423*r41 - l431*r42 - l412*r43;

  let e423 = l423*r1234 + l4*r41 + l431*r43 - l412*r42;
  let e431 = l431*r1234 + l4*r42 + l412*r41 - l423*r43;
  let e412 = l412*r1234 + l4*r43 + l423*r42 - l431*r41;
  let e321 = l321*r1234 - l1*r41 - l2*r42 - l3*r43
    - l4*rs + l423*r23 + l431*r31 + l412*r12;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_antigeometric_multivector(
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

  let e1 = l1234*r1 + l41*r321 + l31*r412 - l12*r431
    + ls*r423 - l43*r2 + l42*r3 + l23*r4;
  let e2 = l1234*r2 + l42*r321 + l12*r423 - l23*r412
    + ls*r431 - l41*r3 + l43*r1 + l31*r4;
  let e3 = l1234*r3 + l43*r321 + l23*r431 - l31*r423
    + ls*r412 - l42*r1 + l41*r2 + l12*r4;
  let e4 = l1234*r4 - l41*r423 - l42*r431 - l43*r412;

  let e41 = (l41*r1234 + l1234*r41) + (l42*r43 - l43*r42);
  let e42 = (l42*r1234 + l1234*r42) + (l43*r41 - l41*r43);
  let e43 = (l43*r1234 + l1234*r43) + (l41*r42 - l42*r41);
  let e23 = (l23*r1234 + l1234*r23) + (ls*r41 + l41*rs)
    + (l31*r43 - l43*r31) + (l42*r12 - l12*r42);
  let e31 = (l31*r1234 + l1234*r31) + (ls*r42 + l42*rs)
    + (l12*r41 - l41*r12) + (l43*r23 - l23*r43);
  let e12 = (l12*r1234 + l1234*r12) + (ls*r43 + l43*rs)
    + (l23*r42 - l42*r23) + (l41*r31 - l31*r41);

  let e423 = l1234*r423 + l41*r4 - l43*r431 + l42*r412;
  let e431 = l1234*r431 + l42*r4 - l41*r412 + l43*r423;
  let e412 = l1234*r412 + l43*r4 - l42*r423 + l41*r431;
  let e321 = l1234*r321 - l41*r1 - l42*r2 - l43*r3
    + ls*r4 - l23*r423 - l31*r431 - l12*r412;

  let e1234 = - l41*r41 - l42*r42 - l43*r43 + l1234*r1234;

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
fn even_grade_antigeometric_scalar(
  EvenGrade {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: EvenGrade,
  Scalar { s: rs }: Scalar,
) -> EvenGrade {

  let s =l1234*rs;

  let e23 =l41*rs;
  let e31 =l42*rs;
  let e12 =l43*rs;

  EvenGrade {
    s,
    e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antigeometric_vector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> OddGrade {

  let e1 = l1234*r1 - l43*r2 + l42*r3 + l23*r4;
  let e2 = l1234*r2 - l41*r3 + l43*r1 + l31*r4;
  let e3 = l1234*r3 - l42*r1 + l41*r2 + l12*r4;
  let e4 = l1234*r4;

  let e423 = l41*r4;
  let e431 = l42*r4;
  let e412 = l43*r4;
  let e321 = -l41*r1 - l42*r2 - l43*r3 + ls*r4;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antigeometric_bivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> EvenGrade {

  let s = -(l43*r12 + l12*r43) - (l41*r23 + l23*r41) - (l31*r42 + l42*r31);

  let e41 = l1234*r41 + (l42*r43 - l43*r42);
  let e42 = l1234*r42 + (l43*r41 - l41*r43);
  let e43 = l1234*r43 + (l41*r42 - l42*r41);
  let e23 = l1234*r23 + ls*r41 + (l31*r43 - l43*r31) + (l42*r12 - l12*r42);
  let e31 = l1234*r31 + ls*r42 + (l12*r41 - l41*r12) + (l43*r23 - l23*r43);
  let e12 = l1234*r12 + ls*r43 + (l23*r42 - l42*r23) + (l41*r31 - l31*r41);

  let e1234 = - l41*r41 - l42*r42 - l43*r43;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antigeometric_trivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> OddGrade {

  let e1 = l41*r321 + l31*r412 - l12*r431 + ls*r423;
  let e2 = l42*r321 + l12*r423 - l23*r412 + ls*r431;
  let e3 = l43*r321 + l23*r431 - l31*r423 + ls*r412;
  let e4 = -l41*r423 - l42*r431 - l43*r412;

  let e423 = l1234*r423 - l43*r431 + l42*r412;
  let e431 = l1234*r431 - l41*r412 + l43*r423;
  let e412 = l1234*r412 - l42*r423 + l41*r431;
  let e321 = l1234*r321 - l23*r423 - l31*r431 - l12*r412;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antigeometric_antiscalar(
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
fn even_grade_antigeometric_dual_number(
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
  let e23 = l23*r1234 + l41*rs;
  let e31 = l31*r1234 + l42*rs;
  let e12 = l12*r1234 + l43*rs;

  let e1234 = l1234*r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antigeometric_odd_grade(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l1234*r1 + l41*r321 + l31*r412 - l12*r431
    + ls*r423 - l43*r2 + l42*r3 + l23*r4;
  let e2 = l1234*r2 + l42*r321 + l12*r423 - l23*r412
    + ls*r431 - l41*r3 + l43*r1 + l31*r4;
  let e3 = l1234*r3 + l43*r321 + l23*r431 - l31*r423
    + ls*r412 - l42*r1 + l41*r2 + l12*r4;
  let e4 = l1234*r4 - l41*r423 - l42*r431 - l43*r412;

  let e423 = l1234*r423 + l41*r4 - l43*r431 + l42*r412;
  let e431 = l1234*r431 + l42*r4 - l41*r412 + l43*r423;
  let e412 = l1234*r412 + l43*r4 - l42*r423 + l41*r431;
  let e321 = l1234*r321 - l41*r1 - l42*r2 - l43*r3
    + ls*r4 - l23*r423 - l31*r431 - l12*r412;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antigeometric_even_grade(
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

  let e41 = (l41*r1234 + l1234*r41) + (l42*r43 - l43*r42);
  let e42 = (l42*r1234 + l1234*r42) + (l43*r41 - l41*r43);
  let e43 = (l43*r1234 + l1234*r43) + (l41*r42 - l42*r41);
  let e23 = (l23*r1234 + l1234*r23) + (ls*r41 + l41*rs)
    + (l31*r43 - l43*r31) + (l42*r12 - l12*r42);
  let e31 = (l31*r1234 + l1234*r31) + (ls*r42 + l42*rs)
    + (l12*r41 - l41*r12) + (l43*r23 - l23*r43);
  let e12 = (l12*r1234 + l1234*r12) + (ls*r43 + l43*rs)
    + (l23*r42 - l42*r23) + (l41*r31 - l31*r41);

  let e1234 = - l41*r41 - l42*r42 - l43*r43 + l1234*r1234;

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
  fn de_morgans_laws() {
    assert_eq!(
      geometric_antiproduct(MULTIVECTOR_A, MULTIVECTOR_B),
      right_complement(geometric_product(
        left_complement(MULTIVECTOR_A),
        left_complement(MULTIVECTOR_B)
      ))
    );

    assert_eq!(
      geometric_antiproduct(MULTIVECTOR_A, MULTIVECTOR_B),
      left_complement(geometric_product(
        right_complement(MULTIVECTOR_A),
        right_complement(MULTIVECTOR_B)
      ))
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
          Multivector::from(geometric_antiproduct(multivector, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_antiproduct(scalar, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_antiproduct(vector, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_antiproduct(bivector, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_antiproduct(trivector, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_antiproduct(antiscalar, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_antiproduct(dual_number, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_antiproduct(odd_grade, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_antiproduct(even_grade, variant)),
          Multivector::from(geometric_antiproduct(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
