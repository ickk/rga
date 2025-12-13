use {
  crate::{
    algebra::values::{
      zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector,
      OddGrade, Scalar, Trivector, Vector,
    },
    helpers::{impl_binary_operation, return_scalar_zero_binary},
  },
  ::core::ops::Mul,
};

/// a ⟑ b
#[inline]
pub fn geometric_product<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <Lhs as GeometricProduct<Rhs>>::Output
where
  Lhs: GeometricProduct<Rhs>,
{
  a.geometric_product(b)
}

/// a ⟑ b
pub trait GeometricProduct<Rhs> {
  type Output;

  /// a ⟑ b
  #[doc(alias = "product")]
  fn geometric_product(self, b: Rhs) -> Self::Output;
}

impl_binary_operation!(GeometricProduct::geometric_product {
  Multivector, Multivector => Multivector: multivector_geometric_multivector;
  Multivector, Scalar => Multivector: Multivector::mul;
  Multivector, Vector => Multivector: multivector_geometric_vector;
  Multivector, Bivector => Multivector: multivector_geometric_bivector;
  Multivector, Trivector => Multivector: multivector_geometric_trivector;
  Multivector, Antiscalar => Multivector: multivector_geometric_antiscalar;
  Multivector, DualNumber => Multivector: multivector_geometric_dual_number;
  Multivector, OddGrade => Multivector: multivector_geometric_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_geometric_even_grade;

  Scalar, Multivector => Multivector: Scalar::mul;
  Scalar, Scalar => Scalar: Scalar::mul;
  Scalar, Vector => Vector: Scalar::mul;
  Scalar, Bivector => Bivector: Scalar::mul;
  Scalar, Trivector => Trivector: Scalar::mul;
  Scalar, Antiscalar => Antiscalar: Scalar::mul;
  Scalar, DualNumber => DualNumber: Scalar::mul;
  Scalar, OddGrade => OddGrade: Scalar::mul;
  Scalar, EvenGrade => EvenGrade: Scalar::mul;

  Vector, Multivector => Multivector: vector_geometric_multivector;
  Vector, Scalar => Vector: Vector::mul;
  Vector, Vector => EvenGrade: vector_geometric_vector;
  Vector, Bivector => OddGrade: vector_geometric_bivector;
  Vector, Trivector => EvenGrade: vector_geometric_trivector;
  Vector, Antiscalar => Trivector: vector_geometric_antiscalar;
  Vector, DualNumber => OddGrade: vector_geometric_dual_number;
  Vector, OddGrade => EvenGrade: vector_geometric_odd_grade;
  Vector, EvenGrade => OddGrade: vector_geometric_even_grade;

  Bivector, Multivector => Multivector: bivector_geometric_multivector;
  Bivector, Scalar => Bivector: Bivector::mul;
  Bivector, Vector => OddGrade: bivector_geometric_vector;
  Bivector, Bivector => EvenGrade: bivector_geometric_bivector;
  Bivector, Trivector => OddGrade: bivector_geometric_trivector;
  Bivector, Antiscalar => Bivector: bivector_geometric_antiscalar;
  Bivector, DualNumber => Bivector: bivector_geometric_dual_number;
  Bivector, OddGrade => OddGrade: bivector_geometric_odd_grade;
  Bivector, EvenGrade => EvenGrade: bivector_geometric_even_grade;

  Trivector, Multivector => Multivector: trivector_geometric_multivector;
  Trivector, Scalar => Trivector: Trivector::mul;
  Trivector, Vector => EvenGrade: trivector_geometric_vector;
  Trivector, Bivector => OddGrade: trivector_geometric_bivector;
  Trivector, Trivector => EvenGrade: trivector_geometric_trivector;
  Trivector, Antiscalar => Vector: trivector_geometric_antiscalar;
  Trivector, DualNumber => OddGrade: trivector_geometric_dual_number;
  Trivector, OddGrade => EvenGrade: trivector_geometric_odd_grade;
  Trivector, EvenGrade => OddGrade: trivector_geometric_even_grade;

  Antiscalar, Multivector => Multivector: antiscalar_geometric_multivector;
  Antiscalar, Scalar => Antiscalar: Antiscalar::mul;
  Antiscalar, Vector => Trivector: antiscalar_geometric_vector;
  Antiscalar, Bivector => Bivector: antiscalar_geometric_bivector;
  Antiscalar, Trivector => Vector: antiscalar_geometric_trivector;
  Antiscalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Antiscalar, DualNumber => Antiscalar: antiscalar_geometric_dual_number;
  Antiscalar, OddGrade => OddGrade: antiscalar_geometric_odd_grade;
  Antiscalar, EvenGrade => EvenGrade: antiscalar_geometric_even_grade;

  DualNumber, Multivector => Multivector: dual_number_geometric_multivector;
  DualNumber, Scalar => DualNumber: DualNumber::mul;
  DualNumber, Vector => OddGrade: dual_number_geometric_vector;
  DualNumber, Bivector => Bivector: dual_number_geometric_bivector;
  DualNumber, Trivector => OddGrade: dual_number_geometric_trivector;
  DualNumber, Antiscalar => Antiscalar: dual_number_geometric_antiscalar;
  DualNumber, DualNumber => DualNumber: dual_number_geometric_dual_number;
  DualNumber, OddGrade => OddGrade: dual_number_geometric_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_geometric_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_geometric_multivector;
  OddGrade, Scalar => OddGrade: OddGrade::mul;
  OddGrade, Vector => EvenGrade: odd_grade_geometric_vector;
  OddGrade, Bivector => OddGrade: odd_grade_geometric_bivector;
  OddGrade, Trivector => EvenGrade: odd_grade_geometric_trivector;
  OddGrade, Antiscalar => OddGrade: odd_grade_geometric_antiscalar;
  OddGrade, DualNumber => OddGrade: odd_grade_geometric_dual_number;
  OddGrade, OddGrade => EvenGrade: odd_grade_geometric_odd_grade;
  OddGrade, EvenGrade => OddGrade: odd_grade_geometric_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_geometric_multivector;
  EvenGrade, Scalar => EvenGrade: EvenGrade::mul;
  EvenGrade, Vector => OddGrade: even_grade_geometric_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_geometric_bivector;
  EvenGrade, Trivector => OddGrade: even_grade_geometric_trivector;
  EvenGrade, Antiscalar => EvenGrade: even_grade_geometric_antiscalar;
  EvenGrade, DualNumber => EvenGrade: even_grade_geometric_dual_number;
  EvenGrade, OddGrade => OddGrade: even_grade_geometric_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_geometric_even_grade;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_geometric_multivector(
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

  let s = ls*rs + l1*r1 + l2*r2 + l3*r3
    - l23*r23 - l31*r31 - l12*r12 - l321*r321;

  let e1 = (l1*rs + ls*r1) - (l2*r12 - l12*r2)
    + (l3*r31 - l31*r3) + (l321*r23 + l23*r321);
  let e2 = (l2*rs + ls*r2) - (l3*r23 - l23*r3)
    + (l1*r12 - l12*r1) + (l321*r31 + l31*r321);
  let e3 = (l3*rs + ls*r3) - (l1*r31 - l31*r1)
    + (l2*r23 - l23*r2) + (l321*r12 + l12*r321);
  let e4 = (l4*rs + ls*r4) - (l1*r41 - l41*r1)
    + (l42*r2 - l2*r42) + (l43*r3 - l3*r43)
    - (l1234*r321 - l321*r1234) - (l23*r423 + l423*r23)
    - (l31*r431 + l431*r31) - (l12*r412 + l412*r12);

  let e41 = (ls*r41 + l41*rs) + (l4*r1 - l1*r4)
    + (l2*r412 + l412*r2) - (l3*r431 + l431*r3)
    + (l12*r42 - l42*r12) + (l43*r31 - l31*r43)
    + (l23*r1234 + l1234*r23) - (l321*r423 - l423*r321);
  let e42 = (ls*r42 + l42*rs) + (l4*r2 - l2*r4)
    + (l3*r423 + l423*r3) - (l1*r412 + l412*r1)
    + (l41*r12 - l12*r41) + (l23*r43 - l43*r23)
    + (l31*r1234 + l1234*r31) - (l321*r431 - l431*r321);
  let e43 = (ls*r43 + l43*rs) + (l4*r3 - l3*r4)
    + (l1*r431 + l431*r1) - (l2*r423 + l423*r2)
    + (l31*r41 - l41*r31) + (l42*r23 - l23*r42)
    + (l12*r1234 + l1234*r12) - (l321*r412 - l412*r321);
  let e23 = (ls*r23 + l23*rs) + (l2*r3 - l3*r2)
    - (l1*r321 + l321*r1) + (l12*r31 - l31*r12);
  let e31 = (ls*r31 + l31*rs) + (l3*r1 - l1*r3)
    - (l2*r321 + l321*r2) + (l23*r12 - l12*r23);
  let e12 = (ls*r12 + l12*rs) + (l1*r2 - l2*r1)
    - (l3*r321 + l321*r3) + (l31*r23 - l23*r31);

  let e423 = (ls*r423 + l423*rs) + (l4*r23 + l23*r4)
    + (l3*r42 + l42*r3) - (l2*r43 + l43*r2)
    + (l1*r1234 - l1234*r1) + (l321*r41 - l41*r321)
      + (l412*r31 - l31*r412) - (l431*r12 - l12*r431);
  let e431 = (ls*r431 + l431*rs) + (l4*r31 + l31*r4)
    + (l1*r43 + l43*r1) - (l3*r41 + l41*r3)
    + (l2*r1234 - l1234*r2) + (l321*r42 - l42*r321)
      + (l423*r12 - l12*r423) - (l412*r23 - l23*r412);
  let e412 = (ls*r412 + l412*rs) + (l4*r12 + l12*r4)
    + (l2*r41 + l41*r2) - (l1*r42 + l42*r1)
    + (l3*r1234 - l1234*r3) + (l321*r43 - l43*r321)
      + (l431*r23 - l23*r431) - (l423*r31 - l31*r423);
  let e321 = (ls*r321 + l321*rs) - (l1*r23 + l23*r1)
    - (l3*r12 + l12*r3) - (l2*r31 + l31*r2);

  let e1234 = (l1*r423 - l423*r1) + (l2*r431 - l431*r2)
    + (l3*r412 - l412*r3) + (l4*r321 - l321*r4)
    - (l41*r23 + l23*r41) - (l42*r31 + l31*r42)
    - (l43*r12 + l12*r43) + (ls*r1234 + l1234*rs);

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
fn multivector_geometric_vector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Multivector {

  let s = l1*r1 + l2*r2 + l3*r3;

  let e1 = ls*r1 - l31*r3 + l12*r2;
  let e2 = ls*r2 - l12*r1 + l23*r3;
  let e3 = ls*r3 - l23*r2 + l31*r1;
  let e4 = ls*r4 + l41*r1 + l42*r2 + l43*r3;

  let e41 = (l4*r1 - l1*r4) + l412*r2 - l431*r3;
  let e42 = (l4*r2 - l2*r4) + l423*r3 - l412*r1;
  let e43 = (l4*r3 - l3*r4) + l431*r1 - l423*r2;
  let e23 = (l2*r3 - l3*r2) - l321*r1;
  let e31 = (l3*r1 - l1*r3) - l321*r2;
  let e12 = (l1*r2 - l2*r1) - l321*r3;

  let e423 = l23*r4 + l42*r3 - l43*r2 - l1234*r1;
  let e431 = l31*r4 + l43*r1 - l41*r3 - l1234*r2;
  let e412 = l12*r4 + l41*r2 - l42*r1 - l1234*r3;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

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
fn multivector_geometric_bivector(
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

  let s = -l23*r23 - l31*r31 - l12*r12;

  let e1 = -l2*r12 + l3*r31 + l321*r23;
  let e2 = -l3*r23 + l1*r12 + l321*r31;
  let e3 = -l1*r31 + l2*r23 + l321*r12;
  let e4 = -l1*r41 - l2*r42 - l3*r43 - l423*r23 - l431*r31 - l412*r12;

  let e41 = ls*r41 + (l12*r42 - l42*r12) + (l43*r31 - l31*r43) + l1234*r23;
  let e42 = ls*r42 + (l41*r12 - l12*r41) + (l23*r43 - l43*r23) + l1234*r31;
  let e43 = ls*r43 + (l31*r41 - l41*r31) + (l42*r23 - l23*r42) + l1234*r12;
  let e23 = ls*r23 + (l12*r31 - l31*r12);
  let e31 = ls*r31 + (l23*r12 - l12*r23);
  let e12 = ls*r12 + (l31*r23 - l23*r31);

  let e423 = l4*r23 + l3*r42 - l2*r43 + l321*r41 + l412*r31 - l431*r12;
  let e431 = l4*r31 + l1*r43 - l3*r41 + l321*r42 + l423*r12 - l412*r23;
  let e412 = l4*r12 + l2*r41 - l1*r42 + l321*r43 + l431*r23 - l423*r31;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

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
fn multivector_geometric_trivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Multivector {

  let s = -l321*r321;

  let e1 = l23*r321;
  let e2 = l31*r321;
  let e3 = l12*r321;
  let e4 = -l1234*r321 - l23*r423 - l31*r431 - l12*r412;

  let e41 = l2*r412 - l3*r431 - (l321*r423 - l423*r321);
  let e42 = l3*r423 - l1*r412 - (l321*r431 - l431*r321);
  let e43 = l1*r431 - l2*r423 - (l321*r412 - l412*r321);
  let e23 = -l1*r321;
  let e31 = -l2*r321;
  let e12 = -l3*r321;

  let e423 = ls*r423 - l41*r321 - l31*r412 + l12*r431;
  let e431 = ls*r431 - l42*r321 - l12*r423 + l23*r412;
  let e412 = ls*r412 - l43*r321 - l23*r431 + l31*r423;
  let e321 = ls*r321;

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

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
fn multivector_geometric_antiscalar(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3,
    e23: l23, e31: l31, e12: l12,
    e321: l321,
    ..
  }: Multivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Multivector {

  let e4 = l321*r1234;

  let e41 = l23*r1234;
  let e42 = l31*r1234;
  let e43 = l12*r1234;

  let e423 = l1*r1234;
  let e431 = l2*r1234;
  let e412 = l3*r1234;

  let e1234 = ls*r1234;

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
fn multivector_geometric_dual_number(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234: l1234,
  }: Multivector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> Multivector {

  let s = ls*rs;

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs + l321*r1234;

  let e41 = l41*rs + l23*r1234;
  let e42 = l42*rs + l31*r1234;
  let e43 = l43*rs + l12*r1234;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e423 = l423*rs + l1*r1234;
  let e431 = l431*rs + l2*r1234;
  let e412 = l412*rs + l3*r1234;
  let e321 = l321*rs;

  let e1234 = ls*r1234 + l1234*rs;

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
fn multivector_geometric_odd_grade(
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

  let s = l1*r1 + l2*r2 + l3*r3 - l321*r321;

  let e1 = ls*r1 + l12*r2 - l31*r3 + l23*r321;
  let e2 = ls*r2 + l23*r3 - l12*r1 + l31*r321;
  let e3 = ls*r3 + l31*r1 - l23*r2 + l12*r321;
  let e4 = ls*r4 + l41*r1 + l42*r2 + l43*r3
    - l1234*r321 - l23*r423 - l31*r431 - l12*r412;

  let e41 = (l4*r1 - l1*r4) + (l2*r412 + l412*r2)
    - (l3*r431 + l431*r3) - (l321*r423 - l423*r321);
  let e42 = (l4*r2 - l2*r4) + (l3*r423 + l423*r3)
    - (l1*r412 + l412*r1) - (l321*r431 - l431*r321);
  let e43 = (l4*r3 - l3*r4) + (l1*r431 + l431*r1)
    - (l2*r423 + l423*r2) - (l321*r412 - l412*r321);
  let e23 = (l2*r3 - l3*r2) - (l1*r321 + l321*r1);
  let e31 = (l3*r1 - l1*r3) - (l2*r321 + l321*r2);
  let e12 = (l1*r2 - l2*r1) - (l3*r321 + l321*r3);

  let e423 = ls*r423 + l23*r4 + l42*r3 - l43*r2
    - l1234*r1 - l41*r321 - l31*r412 + l12*r431;
  let e431 = ls*r431 + l31*r4 + l43*r1 - l41*r3
    - l1234*r2 - l42*r321 - l12*r423 + l23*r412;
  let e412 = ls*r412 + l12*r4 + l41*r2 - l42*r1
    - l1234*r3 - l43*r321 - l23*r431 + l31*r423;
  let e321 = ls*r321 - l23*r1 - l12*r3 - l31*r2;

  let e1234 = (l4*r321 - l321*r4) + (l1*r423 - l423*r1)
    + (l2*r431 - l431*r2) + (l3*r412 - l412*r3);

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
fn multivector_geometric_even_grade(
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

  let s = ls*rs - l23*r23 - l31*r31 - l12*r12;

  let e1 = l1*rs - l2*r12 + l3*r31 + l321*r23;
  let e2 = l2*rs - l3*r23 + l1*r12 + l321*r31;
  let e3 = l3*rs - l1*r31 + l2*r23 + l321*r12;
  let e4 = l4*rs - l1*r41 - l2*r42 - l3*r43
    + l321*r1234 - l423*r23 - l431*r31 - l412*r12;

  let e41 = (ls*r41 + l41*rs) + (l12*r42 - l42*r12)
    + (l43*r31 - l31*r43) + (l23*r1234 + l1234*r23);
  let e42 = (ls*r42 + l42*rs) + (l41*r12 - l12*r41)
    + (l23*r43 - l43*r23) + (l31*r1234 + l1234*r31);
  let e43 = (ls*r43 + l43*rs) + (l31*r41 - l41*r31)
    + (l42*r23 - l23*r42) + (l12*r1234 + l1234*r12);
  let e23 = (ls*r23 + l23*rs) + (l12*r31 - l31*r12);
  let e31 = (ls*r31 + l31*rs) + (l23*r12 - l12*r23);
  let e12 = (ls*r12 + l12*rs) + (l31*r23 - l23*r31);

  let e423 = l423*rs + l4*r23 + l3*r42 - l2*r43
    + l1*r1234 + l321*r41 + l412*r31 - l431*r12;
  let e431 = l431*rs + l4*r31 + l1*r43 - l3*r41
    + l2*r1234 + l321*r42 + l423*r12 - l412*r23;
  let e412 = l412*rs + l4*r12 + l2*r41 - l1*r42
    + l3*r1234 + l321*r43 + l431*r23 - l423*r31;
  let e321 = l321*rs - l1*r23 - l3*r12 - l2*r31;

  let e1234 = (ls*r1234 + l1234*rs) - (l41*r23 + l23*r41)
    - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

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
fn vector_geometric_multivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = l1*r1 + l2*r2 + l3*r3;

  let e1 = l1*rs - l2*r12 + l3*r31;
  let e2 = l2*rs - l3*r23 + l1*r12;
  let e3 = l3*rs - l1*r31 + l2*r23;
  let e4 = l4*rs - l1*r41 - l2*r42 - l3*r43;

  let e41 = (l4*r1 - l1*r4) + l2*r412 - l3*r431;
  let e42 = (l4*r2 - l2*r4) + l3*r423 - l1*r412;
  let e43 = (l4*r3 - l3*r4) + l1*r431 - l2*r423;
  let e23 = (l2*r3 - l3*r2) - l1*r321;
  let e31 = (l3*r1 - l1*r3) - l2*r321;
  let e12 = (l1*r2 - l2*r1) - l3*r321;

  let e423 = l4*r23 + l3*r42 - l2*r43 + l1*r1234;
  let e431 = l4*r31 + l1*r43 - l3*r41 + l2*r1234;
  let e412 = l4*r12 + l2*r41 - l1*r42 + l3*r1234;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

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
fn vector_geometric_vector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> EvenGrade {

  let s = l1*r1 + l2*r2 + l3*r3;

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_geometric_bivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> OddGrade {

  let e1 = -l2*r12 + l3*r31;
  let e2 = -l3*r23 + l1*r12;
  let e3 = -l1*r31 + l2*r23;
  let e4 = -l1*r41 - l2*r42 - l3*r43;

  let e423 = l4*r23 + l3*r42 - l2*r43;
  let e431 = l4*r31 + l1*r43 - l3*r41;
  let e412 = l4*r12 + l2*r41 - l1*r42;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_geometric_trivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> EvenGrade {

  let e41 = l2*r412 - l3*r431;
  let e42 = l3*r423 - l1*r412;
  let e43 = l1*r431 - l2*r423;
  let e23 = -l1*r321;
  let e31 = -l2*r321;
  let e12 = -l3*r321;


  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_geometric_antiscalar(
  Vector { e1: l1, e2: l2, e3: l3, .. }: Vector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Trivector {

  let e423 = l1*r1234;
  let e431 = l2*r1234;
  let e412 = l3*r1234;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn vector_geometric_dual_number(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> OddGrade {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e423 = l1*r1234;
  let e431 = l2*r1234;
  let e412 = l3*r1234;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_geometric_odd_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> EvenGrade {

  let s = l1*r1 + l2*r2 + l3*r3;

  let e41 = (l4*r1 - l1*r4) + l2*r412 - l3*r431;
  let e42 = (l4*r2 - l2*r4) + l3*r423 - l1*r412;
  let e43 = (l4*r3 - l3*r4) + l1*r431 - l2*r423;
  let e23 = (l2*r3 - l3*r2) - l1*r321;
  let e31 = (l3*r1 - l1*r3) - l2*r321;
  let e12 = (l1*r2 - l2*r1) - l3*r321;

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_geometric_even_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*rs - l2*r12 + l3*r31;
  let e2 = l2*rs - l3*r23 + l1*r12;
  let e3 = l3*rs - l1*r31 + l2*r23;
  let e4 = l4*rs - l1*r41 - l2*r42 - l3*r43;

  let e423 = l4*r23 + l3*r42 - l2*r43 + l1*r1234;
  let e431 = l4*r31 + l1*r43 - l3*r41 + l2*r1234;
  let e412 = l4*r12 + l2*r41 - l1*r42 + l3*r1234;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_geometric_multivector(
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

  let s = -l23*r23 - l31*r31 - l12*r12;

  let e1 = l12*r2 - l31*r3 + l23*r321;
  let e2 = l23*r3 - l12*r1 + l31*r321;
  let e3 = l31*r1 - l23*r2 + l12*r321;
  let e4 = l41*r1 + l42*r2 + l43*r3 - l23*r423 - l31*r431 - l12*r412;

  let e41 = l41*rs + (l12*r42 - l42*r12) + (l43*r31 - l31*r43) + l23*r1234;
  let e42 = l42*rs + (l41*r12 - l12*r41) + (l23*r43 - l43*r23) + l31*r1234;
  let e43 = l43*rs + (l31*r41 - l41*r31) + (l42*r23 - l23*r42) + l12*r1234;
  let e23 = l23*rs + (l12*r31 - l31*r12);
  let e31 = l31*rs + (l23*r12 - l12*r23);
  let e12 = l12*rs + (l31*r23 - l23*r31);

  let e423 = l23*r4 + l42*r3 - l43*r2 - l41*r321 - l31*r412 + l12*r431;
  let e431 = l31*r4 + l43*r1 - l41*r3 - l42*r321 - l12*r423 + l23*r412;
  let e412 = l12*r4 + l41*r2 - l42*r1 - l43*r321 - l23*r431 + l31*r423;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

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
fn bivector_geometric_vector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> OddGrade {

  let e1 = l12*r2 - l31*r3;
  let e2 = l23*r3 - l12*r1;
  let e3 = l31*r1 - l23*r2;
  let e4 = l41*r1 + l42*r2 + l43*r3;

  let e423 = l23*r4 + l42*r3 - l43*r2;
  let e431 = l31*r4 + l43*r1 - l41*r3;
  let e412 = l12*r4 + l41*r2 - l42*r1;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_geometric_bivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> EvenGrade {

  let s = -l23*r23 - l31*r31 - l12*r12;

  let e41 = (l12*r42 - l42*r12) + (l43*r31 - l31*r43);
  let e42 = (l41*r12 - l12*r41) + (l23*r43 - l43*r23);
  let e43 = (l31*r41 - l41*r31) + (l42*r23 - l23*r42);
  let e23 = l12*r31 - l31*r12;
  let e31 = l23*r12 - l12*r23;
  let e12 = l31*r23 - l23*r31;

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_geometric_trivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> OddGrade {

  let e1 = l23*r321;
  let e2 = l31*r321;
  let e3 = l12*r321;
  let e4 = -l23*r423 - l31*r431 - l12*r412;

  let e423 = -l41*r321 - l31*r412 + l12*r431;
  let e431 = -l42*r321 - l12*r423 + l23*r412;
  let e412 = -l43*r321 - l23*r431 + l31*r423;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_geometric_antiscalar(
  Bivector { e23: l23, e31: l31, e12: l12, .. }: Bivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Bivector {

  let e41 = l23*r1234;
  let e42 = l31*r1234;
  let e43 = l12*r1234;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn bivector_geometric_dual_number(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> Bivector {

  let e41 = l41*rs + l23*r1234;
  let e42 = l42*rs + l31*r1234;
  let e43 = l43*rs + l12*r1234;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn bivector_geometric_odd_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l12*r2 - l31*r3 + l23*r321;
  let e2 = l23*r3 - l12*r1 + l31*r321;
  let e3 = l31*r1 - l23*r2 + l12*r321;
  let e4 = l41*r1 + l42*r2 + l43*r3 - l23*r423 - l31*r431 - l12*r412;

  let e423 = l23*r4 + l42*r3 - l43*r2 - l41*r321 - l31*r412 + l12*r431;
  let e431 = l31*r4 + l43*r1 - l41*r3 - l42*r321 - l12*r423 + l23*r412;
  let e412 = l12*r4 + l41*r2 - l42*r1 - l43*r321 - l23*r431 + l31*r423;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_geometric_even_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = -l23*r23 - l31*r31 - l12*r12;

  let e41 = l41*rs + (l12*r42 - l42*r12) + (l43*r31 - l31*r43) + l23*r1234;
  let e42 = l42*rs + (l41*r12 - l12*r41) + (l23*r43 - l43*r23) + l31*r1234;
  let e43 = l43*rs + (l31*r41 - l41*r31) + (l42*r23 - l23*r42) + l12*r1234;
  let e23 = l23*rs + (l12*r31 - l31*r12);
  let e31 = l31*rs + (l23*r12 - l12*r23);
  let e12 = l12*rs + (l31*r23 - l23*r31);

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_geometric_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = -l321*r321;

  let e1 = l321*r23;
  let e2 = l321*r31;
  let e3 = l321*r12;
  let e4 = l321*r1234 - l423*r23 - l431*r31 - l412*r12;

  let e41 = l412*r2 - l431*r3 - (l321*r423 - l423*r321);
  let e42 = l423*r3 - l412*r1 - (l321*r431 - l431*r321);
  let e43 = l431*r1 - l423*r2 - (l321*r412 - l412*r321);
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  let e423 = l423*rs + l321*r41 + l412*r31 - l431*r12;
  let e431 = l431*rs + l321*r42 + l423*r12 - l412*r23;
  let e412 = l412*rs + l321*r43 + l431*r23 - l423*r31;
  let e321 = l321*rs;

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

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
fn trivector_geometric_vector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> EvenGrade {

  let e41 = l412*r2 - l431*r3;
  let e42 = l423*r3 - l412*r1;
  let e43 = l431*r1 - l423*r2;
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_geometric_bivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> OddGrade {

  let e1 = l321*r23;
  let e2 = l321*r31;
  let e3 = l321*r12;
  let e4 = -l423*r23 - l431*r31 - l412*r12;

  let e423 = l321*r41 + l412*r31 - l431*r12;
  let e431 = l321*r42 + l423*r12 - l412*r23;
  let e412 = l321*r43 + l431*r23 - l423*r31;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_geometric_trivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> EvenGrade {

  let s = -l321*r321;

  let e41 = -(l321*r423 - l423*r321);
  let e42 = -(l321*r431 - l431*r321);
  let e43 = -(l321*r412 - l412*r321);

  EvenGrade {
    s,
    e41, e42, e43,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_geometric_antiscalar(
  Trivector { e321: l321, .. }: Trivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Vector {

  let e4 = l321*r1234;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn trivector_geometric_dual_number(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> OddGrade {

  let e4 = l321*r1234;

  let e423 = l423*rs;
  let e431 = l431*rs;
  let e412 = l412*rs;
  let e321 = l321*rs;

  OddGrade {
    e4,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_geometric_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> EvenGrade {

  let s = -l321*r321;

  let e41 = l412*r2 - l431*r3 - (l321*r423 - l423*r321);
  let e42 = l423*r3 - l412*r1 - (l321*r431 - l431*r321);
  let e43 = l431*r1 - l423*r2 - (l321*r412 - l412*r321);
  let e23 = -l321*r1;
  let e31 = -l321*r2;
  let e12 = -l321*r3;

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_geometric_even_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> OddGrade {

  let e1 = l321*r23;
  let e2 = l321*r31;
  let e3 = l321*r12;
  let e4 = l321*r1234 - l423*r23 - l431*r31 - l412*r12;

  let e423 = l423*rs + l321*r41 + l412*r31 - l431*r12;
  let e431 = l431*rs + l321*r42 + l423*r12 - l412*r23;
  let e412 = l412*rs + l321*r43 + l431*r23 - l423*r31;
  let e321 = l321*rs;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// antiscalar

#[rustfmt::skip]
#[inline]
fn antiscalar_geometric_multivector(
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

  let e41 = l1234*r23;
  let e42 = l1234*r31;
  let e43 = l1234*r12;

  let e423 = -l1234*r1;
  let e431 = -l1234*r2;
  let e412 = -l1234*r3;

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
fn antiscalar_geometric_vector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Vector { e1: r1, e2: r2, e3: r3, .. }: Vector,
) -> Trivector {

  let e423 = -l1234*r1;
  let e431 = -l1234*r2;
  let e412 = -l1234*r3;

  Trivector { e423, e431, e412, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_geometric_bivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Bivector { e23: r23, e31: r31, e12: r12, .. }: Bivector,
) -> Bivector {

  let e41 = l1234*r23;
  let e42 = l1234*r31;
  let e43 = l1234*r12;

  Bivector { e41, e42, e43, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_geometric_trivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Trivector { e321: r321, .. }: Trivector,
) -> Vector {

  let e4 = -l1234*r321;

  Vector { e4, ..zero() }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_geometric_dual_number(
  Antiscalar { e1234: l1234 }: Antiscalar,
  DualNumber { s: rs, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = l1234*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_geometric_odd_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  OddGrade {
    e1: r1, e2: r2, e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> OddGrade {

  let e4 = -l1234*r321;

  let e423 = -l1234*r1;
  let e431 = -l1234*r2;
  let e412 = -l1234*r3;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_geometric_even_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  EvenGrade {
    s: rs,
    e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let e41 = l1234*r23;
  let e42 = l1234*r31;
  let e43 = l1234*r12;

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
fn dual_number_geometric_multivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = ls*rs;

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4 - l1234*r321;

  let e41 = ls*r41 + l1234*r23;
  let e42 = ls*r42 + l1234*r31;
  let e43 = ls*r43 + l1234*r12;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e423 = ls*r423 - l1234*r1;
  let e431 = ls*r431 - l1234*r2;
  let e412 = ls*r412 - l1234*r3;
  let e321 = ls*r321;

  let e1234 = ls*r1234 + l1234*rs;

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
fn dual_number_geometric_vector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> OddGrade {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e423 = -l1234*r1;
  let e431 = -l1234*r2;
  let e412 = -l1234*r3;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_geometric_bivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Bivector {

  let e41 = ls*r41 + l1234*r23;
  let e42 = ls*r42 + l1234*r31;
  let e43 = ls*r43 + l1234*r12;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_geometric_trivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> OddGrade {

  let e4 = -l1234*r321;

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  OddGrade {
    e4,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_geometric_antiscalar(
  DualNumber { s: ls, .. }: DualNumber,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Antiscalar {

  let e1234 = ls*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_geometric_dual_number(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let s = ls*rs;
  let e1234 = ls*r1234 + l1234*rs;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_geometric_odd_grade(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4 - l1234*r321;

  let e423 = ls*r423 - l1234*r1;
  let e431 = ls*r431 - l1234*r2;
  let e412 = ls*r412 - l1234*r3;
  let e321 = ls*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_geometric_even_grade(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*rs;

  let e41 = ls*r41 + l1234*r23;
  let e42 = ls*r42 + l1234*r31;
  let e43 = ls*r43 + l1234*r12;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e1234 = ls*r1234 + l1234*rs;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

// odd grade

#[rustfmt::skip]
#[inline]
fn odd_grade_geometric_multivector(
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

  let s = l1*r1 + l2*r2 + l3*r3 - l321*r321;

  let e1 = l1*rs - l2*r12 + l3*r31 + l321*r23;
  let e2 = l2*rs - l3*r23 + l1*r12 + l321*r31;
  let e3 = l3*rs - l1*r31 + l2*r23 + l321*r12;
  let e4 = l4*rs - l1*r41 - l2*r42 - l3*r43
    + l321*r1234 - l423*r23 - l431*r31 - l412*r12;

  let e41 = (l4*r1 - l1*r4) + (l2*r412 + l412*r2)
    - (l3*r431 + l431*r3) - (l321*r423 - l423*r321);
  let e42 = (l4*r2 - l2*r4) + (l3*r423 + l423*r3)
    - (l1*r412 + l412*r1) - (l321*r431 - l431*r321);
  let e43 = (l4*r3 - l3*r4) + (l1*r431 + l431*r1)
    - (l2*r423 + l423*r2) - (l321*r412 - l412*r321);
  let e23 = (l2*r3 - l3*r2) - (l1*r321 + l321*r1);
  let e31 = (l3*r1 - l1*r3) - (l2*r321 + l321*r2);
  let e12 = (l1*r2 - l2*r1) - (l3*r321 + l321*r3);

  let e423 = l423*rs + l4*r23 + l3*r42 - l2*r43
    + l1*r1234 + l321*r41 + l412*r31 - l431*r12;
  let e431 = l431*rs + l4*r31 + l1*r43 - l3*r41
    + l2*r1234 + l321*r42 + l423*r12 - l412*r23;
  let e412 = l412*rs + l4*r12 + l2*r41 - l1*r42
    + l3*r1234 + l321*r43 + l431*r23 - l423*r31;
  let e321 = l321*rs - l1*r23 - l3*r12 - l2*r31;

  let e1234 = (l4*r321 - l321*r4) + (l1*r423 - l423*r1)
    + (l2*r431 - l431*r2) + (l3*r412 - l412*r3);

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
fn odd_grade_geometric_vector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> EvenGrade {

  let s = l1*r1 + l2*r2 + l3*r3;

  let e41 = (l4*r1 - l1*r4) + l412*r2 - l431*r3;
  let e42 = (l4*r2 - l2*r4) + l423*r3 - l412*r1;
  let e43 = (l4*r3 - l3*r4) + l431*r1 - l423*r2;
  let e23 = (l2*r3 - l3*r2) - l321*r1;
  let e31 = (l3*r1 - l1*r3) - l321*r2;
  let e12 = (l1*r2 - l2*r1) - l321*r3;

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_geometric_bivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> OddGrade {

  let e1 = -l2*r12 + l3*r31 + l321*r23;
  let e2 = -l3*r23 + l1*r12 + l321*r31;
  let e3 = -l1*r31 + l2*r23 + l321*r12;
  let e4 = -l1*r41 - l2*r42 - l3*r43 - l423*r23 - l431*r31 - l412*r12;

  let e423 = l4*r23 + l3*r42 - l2*r43 + l321*r41 + l412*r31 - l431*r12;
  let e431 = l4*r31 + l1*r43 - l3*r41 + l321*r42 + l423*r12 - l412*r23;
  let e412 = l4*r12 + l2*r41 - l1*r42 + l321*r43 + l431*r23 - l423*r31;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_geometric_trivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> EvenGrade {

  let s = -l321*r321;

  let e41 = l2*r412 - l3*r431 - (l321*r423 - l423*r321);
  let e42 = l3*r423 - l1*r412 - (l321*r431 - l431*r321);
  let e43 = l1*r431 - l2*r423 - (l321*r412 - l412*r321);
  let e23 = -l1*r321;
  let e31 = -l2*r321;
  let e12 = -l3*r321;

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_geometric_antiscalar(
  OddGrade {
    e1: l1, e2: l2, e3: l3,
    e321: l321,
    ..
  }: OddGrade,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> OddGrade {

  let e4 = l321*r1234;

  let e423 = l1*r1234;
  let e431 = l2*r1234;
  let e412 = l3*r1234;

  OddGrade {
    e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_geometric_dual_number(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> OddGrade {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs + l321*r1234;

  let e423 = l423*rs + l1*r1234;
  let e431 = l431*rs + l2*r1234;
  let e412 = l412*rs + l3*r1234;
  let e321 = l321*rs;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_geometric_odd_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> EvenGrade {

  let s = l1*r1 + l2*r2 + l3*r3 - l321*r321;

  let e41 = (l4*r1 - l1*r4) + (l2*r412 + l412*r2)
    - (l3*r431 + l431*r3) - (l321*r423 - l423*r321);
  let e42 = (l4*r2 - l2*r4) + (l3*r423 + l423*r3)
    - (l1*r412 + l412*r1) - (l321*r431 - l431*r321);
  let e43 = (l4*r3 - l3*r4) + (l1*r431 + l431*r1)
    - (l2*r423 + l423*r2) - (l321*r412 - l412*r321);
  let e23 = (l2*r3 - l3*r2) - (l1*r321 + l321*r1);
  let e31 = (l3*r1 - l1*r3) - (l2*r321 + l321*r2);
  let e12 = (l1*r2 - l2*r1) - (l3*r321 + l321*r3);

  let e1234 = (l4*r321 - l321*r4) + (l1*r423 - l423*r1)
    + (l2*r431 - l431*r2) + (l3*r412 - l412*r3);

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_geometric_even_grade(
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

  let e1 = l1*rs - l2*r12 + l3*r31 + l321*r23;
  let e2 = l2*rs - l3*r23 + l1*r12 + l321*r31;
  let e3 = l3*rs - l1*r31 + l2*r23 + l321*r12;
  let e4 = l4*rs - l1*r41 - l2*r42 - l3*r43
    + l321*r1234 - l423*r23 - l431*r31 - l412*r12;

  let e423 = l423*rs + l4*r23 + l3*r42 - l2*r43
    + l1*r1234 + l321*r41 + l412*r31 - l431*r12;
  let e431 = l431*rs + l4*r31 + l1*r43 - l3*r41
    + l2*r1234 + l321*r42 + l423*r12 - l412*r23;
  let e412 = l412*rs + l4*r12 + l2*r41 - l1*r42
    + l3*r1234 + l321*r43 + l431*r23 - l423*r31;
  let e321 = l321*rs - l1*r23 - l3*r12 - l2*r31;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_geometric_multivector(
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

  let s = ls*rs - l23*r23 - l31*r31 - l12*r12;

  let e1 = ls*r1 + l12*r2 - l31*r3 + l23*r321;
  let e2 = ls*r2 + l23*r3 - l12*r1 + l31*r321;
  let e3 = ls*r3 + l31*r1 - l23*r2 + l12*r321;
  let e4 = ls*r4 + l41*r1 + l42*r2 + l43*r3
    - l1234*r321 - l23*r423 - l31*r431 - l12*r412;

  let e41 = (ls*r41 + l41*rs) + (l12*r42 - l42*r12)
    + (l43*r31 - l31*r43) + (l23*r1234 + l1234*r23);
  let e42 = (ls*r42 + l42*rs) + (l41*r12 - l12*r41)
    + (l23*r43 - l43*r23) + (l31*r1234 + l1234*r31);
  let e43 = (ls*r43 + l43*rs) + (l31*r41 - l41*r31)
    + (l42*r23 - l23*r42) + (l12*r1234 + l1234*r12);
  let e23 = (ls*r23 + l23*rs) + (l12*r31 - l31*r12);
  let e31 = (ls*r31 + l31*rs) + (l23*r12 - l12*r23);
  let e12 = (ls*r12 + l12*rs) + (l31*r23 - l23*r31);

  let e423 = ls*r423 + l23*r4 + l42*r3 - l43*r2
    - l1234*r1 - l41*r321 - l31*r412 + l12*r431;
  let e431 = ls*r431 + l31*r4 + l43*r1 - l41*r3
    - l1234*r2 - l42*r321 - l12*r423 + l23*r412;
  let e412 = ls*r412 + l12*r4 + l41*r2 - l42*r1
    - l1234*r3 - l43*r321 - l23*r431 + l31*r423;
  let e321 = ls*r321 - l23*r1 - l12*r3 - l31*r2;

  let e1234 = (ls*r1234 + l1234*rs) - (l41*r23 + l23*r41)
    - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

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
fn even_grade_geometric_vector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> OddGrade {

  let e1 = ls*r1 + l12*r2 - l31*r3;
  let e2 = ls*r2 + l23*r3 - l12*r1;
  let e3 = ls*r3 + l31*r1 - l23*r2;
  let e4 = ls*r4 + l41*r1 + l42*r2 + l43*r3;

  let e423 = l23*r4 + l42*r3 - l43*r2 - l1234*r1;
  let e431 = l31*r4 + l43*r1 - l41*r3 - l1234*r2;
  let e412 = l12*r4 + l41*r2 - l42*r1 - l1234*r3;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_geometric_bivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> EvenGrade {

  let s = -l23*r23 - l31*r31 - l12*r12;

  let e41 = ls*r41 + (l12*r42 - l42*r12) + (l43*r31 - l31*r43) + l1234*r23;
  let e42 = ls*r42 + (l41*r12 - l12*r41) + (l23*r43 - l43*r23) + l1234*r31;
  let e43 = ls*r43 + (l31*r41 - l41*r31) + (l42*r23 - l23*r42) + l1234*r12;
  let e23 = ls*r23 + (l12*r31 - l31*r12);
  let e31 = ls*r31 + (l23*r12 - l12*r23);
  let e12 = ls*r12 + (l31*r23 - l23*r31);

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_geometric_trivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> OddGrade {

  let e1 = l23*r321;
  let e2 = l31*r321;
  let e3 = l12*r321;
  let e4 = -l1234*r321 - l23*r423 - l31*r431 - l12*r412;

  let e423 = ls*r423 - l41*r321 - l31*r412 + l12*r431;
  let e431 = ls*r431 - l42*r321 - l12*r423 + l23*r412;
  let e412 = ls*r412 - l43*r321 - l23*r431 + l31*r423;
  let e321 = ls*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_geometric_antiscalar(
  EvenGrade {
    s: ls,
    e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> EvenGrade {

  let e41 = l23*r1234;
  let e42 = l31*r1234;
  let e43 = l12*r1234;

  let e1234 = ls*r1234;

  EvenGrade {
    e41, e42, e43,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_geometric_dual_number(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> EvenGrade {

  let s = ls*rs;

  let e41 = l41*rs + l23*r1234;
  let e42 = l42*rs + l31*r1234;
  let e43 = l43*rs + l12*r1234;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e1234 = ls*r1234 + l1234*rs;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_geometric_odd_grade(
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

  let e1 = ls*r1 + l12*r2 - l31*r3 + l23*r321;
  let e2 = ls*r2 + l23*r3 - l12*r1 + l31*r321;
  let e3 = ls*r3 + l31*r1 - l23*r2 + l12*r321;
  let e4 = ls*r4 + l41*r1 + l42*r2 + l43*r3
    - l1234*r321 - l23*r423 - l31*r431 - l12*r412;

  let e423 = ls*r423 + l23*r4 + l42*r3 - l43*r2
    - l1234*r1 - l41*r321 - l31*r412 + l12*r431;
  let e431 = ls*r431 + l31*r4 + l43*r1 - l41*r3
    - l1234*r2 - l42*r321 - l12*r423 + l23*r412;
  let e412 = ls*r412 + l12*r4 + l41*r2 - l42*r1
    - l1234*r3 - l43*r321 - l23*r431 + l31*r423;
  let e321 = ls*r321 - l23*r1 - l12*r3 - l31*r2;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_geometric_even_grade(
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

  let s = ls*rs - l23*r23 - l31*r31 - l12*r12;

  let e41 = (ls*r41 + l41*rs) + (l12*r42 - l42*r12)
    + (l43*r31 - l31*r43) + (l23*r1234 + l1234*r23);
  let e42 = (ls*r42 + l42*rs) + (l41*r12 - l12*r41)
    + (l23*r43 - l43*r23) + (l31*r1234 + l1234*r31);
  let e43 = (ls*r43 + l43*rs) + (l31*r41 - l41*r31)
    + (l42*r23 - l23*r42) + (l12*r1234 + l1234*r12);
  let e23 = (ls*r23 + l23*rs) + (l12*r31 - l31*r12);
  let e31 = (ls*r31 + l31*rs) + (l23*r12 - l12*r23);
  let e12 = (ls*r12 + l12*rs) + (l31*r23 - l23*r31);

  let e1234 = (ls*r1234 + l1234*rs) - (l41*r23 + l23*r41)
    - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

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
  fn associativity() {
    assert_eq!(
      geometric_product(
        geometric_product(MULTIVECTOR_A, MULTIVECTOR_B),
        MULTIVECTOR_C
      ),
      geometric_product(
        MULTIVECTOR_A,
        geometric_product(MULTIVECTOR_B, MULTIVECTOR_C)
      )
    );
  }

  #[test]
  fn distributivity() {
    assert_eq!(
      geometric_product(MULTIVECTOR_A, MULTIVECTOR_B + MULTIVECTOR_C),
      (geometric_product(MULTIVECTOR_A, MULTIVECTOR_B)
        + geometric_product(MULTIVECTOR_A, MULTIVECTOR_C))
    );

    assert_eq!(
      geometric_product(MULTIVECTOR_A + MULTIVECTOR_B, MULTIVECTOR_C),
      (geometric_product(MULTIVECTOR_A, MULTIVECTOR_C)
        + geometric_product(MULTIVECTOR_B, MULTIVECTOR_C))
    );
  }

  #[test]
  fn scalar_factorisation() {
    assert_eq!(
      geometric_product(
        geometric_product(SCALAR_A, MULTIVECTOR_A),
        MULTIVECTOR_B
      ),
      geometric_product(
        MULTIVECTOR_A,
        geometric_product(SCALAR_A, MULTIVECTOR_B)
      )
    );

    assert_eq!(
      geometric_product(
        geometric_product(SCALAR_A, MULTIVECTOR_A),
        MULTIVECTOR_B
      ),
      SCALAR_A * geometric_product(MULTIVECTOR_A, MULTIVECTOR_B),
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
          Multivector::from(geometric_product(multivector, variant)),
          Multivector::from(geometric_product(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_product(scalar, variant)),
          Multivector::from(geometric_product(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_product(vector, variant)),
          Multivector::from(geometric_product(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_product(bivector, variant)),
          Multivector::from(geometric_product(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_product(trivector, variant)),
          Multivector::from(geometric_product(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_product(antiscalar, variant)),
          Multivector::from(geometric_product(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_product(dual_number, variant)),
          Multivector::from(geometric_product(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_product(odd_grade, variant)),
          Multivector::from(geometric_product(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(geometric_product(even_grade, variant)),
          Multivector::from(geometric_product(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
