use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{impl_binary_operation, return_scalar_zero_binary},
};

/// a ∧ b
///
/// The exterior product.
#[inline]
pub fn wedge<Lhs, Rhs>(a: Lhs, b: Rhs) -> <Lhs as WedgeProduct<Rhs>>::Output
where
  Lhs: WedgeProduct<Rhs>,
{
  WedgeProduct::wedge(a, b)
}

/// a ∧ b
///
/// The exterior product.
pub trait WedgeProduct<Rhs> {
  type Output;

  /// a ∧ b
  ///
  /// The exterior product.
  #[doc(alias = "exterior product", alias = "product")]
  fn wedge(self, b: Rhs) -> Self::Output;
}

impl_binary_operation!(WedgeProduct::wedge {
  Multivector, Multivector => Multivector: multivector_wedge_multivector;
  Multivector, Scalar => Multivector: multivector_wedge_scalar;
  Multivector, Vector => Multivector: multivector_wedge_vector;
  Multivector, Bivector => Multivector: multivector_wedge_bivector;
  Multivector, Trivector => Multivector: multivector_wedge_trivector;
  Multivector, Antiscalar => Antiscalar: multivector_wedge_antiscalar;
  Multivector, DualNumber => Multivector: multivector_wedge_dual_number;
  Multivector, OddGrade => Multivector: multivector_wedge_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_wedge_even_grade;

  Scalar, Multivector => Multivector: scalar_wedge_multivector;
  Scalar, Scalar => Scalar: scalar_wedge_scalar;
  Scalar, Vector => Vector: scalar_wedge_vector;
  Scalar, Bivector => Bivector: scalar_wedge_bivector;
  Scalar, Trivector => Trivector: scalar_wedge_trivector;
  Scalar, Antiscalar => Antiscalar: scalar_wedge_antiscalar;
  Scalar, DualNumber => DualNumber: scalar_wedge_dual_number;
  Scalar, OddGrade => OddGrade: scalar_wedge_odd_grade;
  Scalar, EvenGrade => EvenGrade: scalar_wedge_even_grade;

  Vector, Multivector => Multivector: vector_wedge_multivector;
  Vector, Scalar => Vector: vector_wedge_scalar;
  Vector, Vector => Bivector: vector_wedge_vector;
  Vector, Bivector => Trivector: vector_wedge_bivector;
  Vector, Trivector => Antiscalar: vector_wedge_trivector;
  Vector, Antiscalar => Scalar: return_scalar_zero_binary;
  Vector, DualNumber => Vector: vector_wedge_dual_number;
  Vector, OddGrade => EvenGrade: vector_wedge_odd_grade;
  Vector, EvenGrade => OddGrade: vector_wedge_even_grade;

  Bivector, Multivector => Multivector: bivector_wedge_multivector;
  Bivector, Scalar => Bivector: bivector_wedge_scalar;
  Bivector, Vector => Trivector: bivector_wedge_vector;
  Bivector, Bivector => Antiscalar: bivector_wedge_bivector;
  Bivector, Trivector => Scalar: return_scalar_zero_binary;
  Bivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Bivector, DualNumber => Bivector: bivector_wedge_dual_number;
  Bivector, OddGrade => Trivector: bivector_wedge_odd_grade;
  Bivector, EvenGrade => EvenGrade: bivector_wedge_even_grade;

  Trivector, Multivector => Multivector: trivector_wedge_multivector;
  Trivector, Scalar => Trivector: trivector_wedge_scalar;
  Trivector, Vector => Antiscalar: trivector_wedge_vector;
  Trivector, Bivector => Scalar: return_scalar_zero_binary;
  Trivector, Trivector => Scalar: return_scalar_zero_binary;
  Trivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Trivector, DualNumber => Trivector: trivector_wedge_dual_number;
  Trivector, OddGrade => Antiscalar: trivector_wedge_odd_grade;
  Trivector, EvenGrade => Trivector: trivector_wedge_even_grade;

  Antiscalar, Multivector => Antiscalar: antiscalar_wedge_multivector;
  Antiscalar, Scalar => Antiscalar: antiscalar_wedge_scalar;
  Antiscalar, Vector => Scalar: return_scalar_zero_binary;
  Antiscalar, Bivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Trivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Antiscalar, DualNumber => Antiscalar: antiscalar_wedge_dual_number;
  Antiscalar, OddGrade => Scalar: return_scalar_zero_binary;
  Antiscalar, EvenGrade => Antiscalar: antiscalar_wedge_even_grade;

  DualNumber, Multivector => Multivector: dual_number_wedge_multivector;
  DualNumber, Scalar => DualNumber: dual_number_wedge_scalar;
  DualNumber, Vector => Vector: dual_number_wedge_vector;
  DualNumber, Bivector => Bivector: dual_number_wedge_bivector;
  DualNumber, Trivector => Trivector: dual_number_wedge_trivector;
  DualNumber, Antiscalar => Antiscalar: dual_number_wedge_antiscalar;
  DualNumber, DualNumber => DualNumber: dual_number_wedge_dual_number;
  DualNumber, OddGrade => OddGrade: dual_number_wedge_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_wedge_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_wedge_multivector;
  OddGrade, Scalar => OddGrade: odd_grade_wedge_scalar;
  OddGrade, Vector => EvenGrade: odd_grade_wedge_vector;
  OddGrade, Bivector => Trivector: odd_grade_wedge_bivector;
  OddGrade, Trivector => Antiscalar: odd_grade_wedge_trivector;
  OddGrade, Antiscalar => Scalar: return_scalar_zero_binary;
  OddGrade, DualNumber => OddGrade: odd_grade_wedge_dual_number;
  OddGrade, OddGrade => EvenGrade: odd_grade_wedge_odd_grade;
  OddGrade, EvenGrade => OddGrade: odd_grade_wedge_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_wedge_multivector;
  EvenGrade, Scalar => EvenGrade: even_grade_wedge_scalar;
  EvenGrade, Vector => OddGrade: even_grade_wedge_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_wedge_bivector;
  EvenGrade, Trivector => Trivector: even_grade_wedge_trivector;
  EvenGrade, Antiscalar => Antiscalar: even_grade_wedge_antiscalar;
  EvenGrade, DualNumber => EvenGrade: even_grade_wedge_dual_number;
  EvenGrade, OddGrade => OddGrade: even_grade_wedge_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_wedge_even_grade;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_wedge_multivector(
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

  let s = ls*rs;

  let e1 = ls*r1 + l1*rs;
  let e2 = ls*r2 + l2*rs;
  let e3 = ls*r3 + l3*rs;
  let e4 = ls*r4 + l4*rs;

  let e41 = (ls*r41 + l41*rs) + (l4*r1 - l1*r4);
  let e42 = (ls*r42 + l42*rs) + (l4*r2 - l2*r4);
  let e43 = (ls*r43 + l43*rs) + (l4*r3 - l3*r4);
  let e23 = (ls*r23 + l23*rs) + (l2*r3 - l3*r2);
  let e31 = (ls*r31 + l31*rs) + (l3*r1 - l1*r3);
  let e12 = (ls*r12 + l12*rs) + (l1*r2 - l2*r1);

  let e423 = (ls*r423 + rs*l423) + (l4*r23 + l23*r4)
    + (l3*r42 + l42*r3) - (l2*r43 + l43*r2);
  let e431 = (ls*r431 + rs*l431) + (l4*r31 + l31*r4)
    + (l1*r43 + l43*r1) - (l3*r41 + l41*r3);
  let e412 = (ls*r412 + rs*l412) + (l4*r12 + l12*r4)
    + (l2*r41 + l41*r2) - (l1*r42 + l42*r1);
  let e321 = (ls*r321 + rs*l321) - (l1*r23 + l23*r1)
    - (l3*r12 + l12*r3) - (l2*r31 + l31*r2);

  let e1234 = (ls*r1234 + l1234*rs) + (l4*r321 - l321*r4)
    + (l1*r423 - l423*r1) + (l2*r431 - l431*r2)
    + (l3*r412 - l412*r3) - (l41*r23 + l23*r41)
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
fn multivector_wedge_scalar(
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
fn multivector_wedge_vector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    ..
  }: Multivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Multivector {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  let e423 = l23*r4 + l42*r3 - l43*r2;
  let e431 = l31*r4 + l43*r1 - l41*r3;
  let e412 = l12*r4 + l41*r2 - l42*r1;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

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
fn multivector_wedge_bivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    ..
  }: Multivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Multivector {

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e423 = l4*r23 + l3*r42 - l2*r43;
  let e431 = l4*r31 + l1*r43 - l3*r41;
  let e412 = l4*r12 + l2*r41 - l1*r42;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

  Multivector {
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_wedge_trivector(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    ..
  }: Multivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Multivector {

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

  Multivector {
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn multivector_wedge_antiscalar(
  Multivector { s: ls, .. }: Multivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Antiscalar {

  let e1234 = ls*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn multivector_wedge_dual_number(
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
  let e4 = l4*rs;

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e423 = rs*l423;
  let e431 = rs*l431;
  let e412 = rs*l412;
  let e321 = rs*l321;

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
fn multivector_wedge_odd_grade(
  Multivector {
    s: ls,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    ..
  }: Multivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> Multivector {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  let e423 = ls*r423 + l23*r4 + l42*r3 - l43*r2;
  let e431 = ls*r431 + l31*r4 + l43*r1 - l41*r3;
  let e412 = ls*r412 + l12*r4 + l41*r2 - l42*r1;
  let e321 = ls*r321 - l23*r1 - l12*r3 - l31*r2;

  let e1234 = (l4*r321 - l321*r4) + (l1*r423 - l423*r1)
    + (l2*r431 - l431*r2) + (l3*r412 - l412*r3);

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
fn multivector_wedge_even_grade(
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

  let s = ls*rs;

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e41 = ls*r41 + l41*rs;
  let e42 = ls*r42 + l42*rs;
  let e43 = ls*r43 + l43*rs;
  let e23 = ls*r23 + l23*rs;
  let e31 = ls*r31 + l31*rs;
  let e12 = ls*r12 + l12*rs;

  let e423 = rs*l423 + l4*r23 + l3*r42 - l2*r43;
  let e431 = rs*l431 + l4*r31 + l1*r43 - l3*r41;
  let e412 = rs*l412 + l4*r12 + l2*r41 - l1*r42;
  let e321 = rs*l321 - l1*r23 - l3*r12 - l2*r31;

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

// scalar

#[rustfmt::skip]
#[inline]
fn scalar_wedge_multivector(
  Scalar { s: ls }: Scalar,
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
  let e4 = ls*r4;

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  let e1234 = ls*r1234;

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
fn scalar_wedge_scalar(
  Scalar { s: ls }: Scalar,
  Scalar { s: rs }: Scalar,
) -> Scalar {

  let s = ls*rs;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn scalar_wedge_vector(
  Scalar { s: ls }: Scalar,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Vector {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn scalar_wedge_bivector(
  Scalar { s: ls }: Scalar,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Bivector {

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn scalar_wedge_trivector(
  Scalar { s: ls }: Scalar,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Trivector {

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn scalar_wedge_antiscalar(
  Scalar { s: ls }: Scalar,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Antiscalar {

  let e1234 = ls*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn scalar_wedge_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let s = ls*rs;
  let e1234 = ls*r1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn scalar_wedge_odd_grade(
  Scalar { s: ls }: Scalar,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_wedge_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*rs;

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e1234 = ls*r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

// vector

#[rustfmt::skip]
#[inline]
fn vector_wedge_multivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  let e423 = l4*r23 + l3*r42 - l2*r43;
  let e431 = l4*r31 + l1*r43 - l3*r41;
  let e412 = l4*r12 + l2*r41 - l1*r42;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

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
fn vector_wedge_scalar(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
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
fn vector_wedge_vector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Bivector {

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn vector_wedge_bivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Trivector {

  let e423 = l4*r23 + l3*r42 - l2*r43;
  let e431 = l4*r31 + l1*r43 - l3*r41;
  let e412 = l4*r12 + l2*r41 - l1*r42;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn vector_wedge_trivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Antiscalar {

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn vector_wedge_dual_number(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
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
fn vector_wedge_odd_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> EvenGrade {

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_wedge_even_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e423 = l4*r23 + l3*r42 - l2*r43;
  let e431 = l4*r31 + l1*r43 - l3*r41;
  let e412 = l4*r12 + l2*r41 - l1*r42;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_wedge_multivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    ..
  }: Multivector,
) -> Multivector {

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e423 = l23*r4 + l42*r3 - l43*r2;
  let e431 = l31*r4 + l43*r1 - l41*r3;
  let e412 = l12*r4 + l41*r2 - l42*r1;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

  Multivector {
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_wedge_scalar(
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
fn bivector_wedge_vector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Trivector {

  let e423 = l23*r4 + l42*r3 - l43*r2;
  let e431 = l31*r4 + l43*r1 - l41*r3;
  let e412 = l12*r4 + l41*r2 - l42*r1;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn bivector_wedge_bivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Antiscalar {

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn bivector_wedge_dual_number(
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
fn bivector_wedge_odd_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  OddGrade { e1: r1, e2: r2, e3: r3, e4: r4, .. }: OddGrade,
) -> Trivector {

  let e423 = l23*r4 + l42*r3 - l43*r2;
  let e431 = l31*r4 + l43*r1 - l41*r3;
  let e412 = l12*r4 + l41*r2 - l42*r1;
  let e321 = -l23*r1 - l12*r3 - l31*r2;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn bivector_wedge_even_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> EvenGrade {

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
  let e23 = l23*rs;
  let e31 = l31*rs;
  let e12 = l12*rs;

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_wedge_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    ..
  }: Multivector,
) -> Multivector {

  let e423 = rs*l423;
  let e431 = rs*l431;
  let e412 = rs*l412;
  let e321 = rs*l321;

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

  Multivector {
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_wedge_scalar(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Scalar { s: rs }: Scalar,
) -> Trivector {

  let e423 = rs*l423;
  let e431 = rs*l431;
  let e412 = rs*l412;
  let e321 = rs*l321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn trivector_wedge_vector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Antiscalar {

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn trivector_wedge_dual_number(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  DualNumber { s: rs, .. }: DualNumber,
) -> Trivector {

  let e423 = rs*l423;
  let e431 = rs*l431;
  let e412 = rs*l412;
  let e321 = rs*l321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn trivector_wedge_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  OddGrade { e1: r1, e2: r2, e3: r3, e4: r4, .. }: OddGrade,
) -> Antiscalar {

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn trivector_wedge_even_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  EvenGrade { s: rs, .. }: EvenGrade,
) -> Trivector {

  let e423 = rs*l423;
  let e431 = rs*l431;
  let e412 = rs*l412;
  let e321 = rs*l321;

  Trivector { e423, e431, e412, e321 }
}

// antiscalar

#[rustfmt::skip]
#[inline]
fn antiscalar_wedge_multivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Multivector { s: rs, .. }: Multivector,
) -> Antiscalar {

  let e1234 = l1234*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_wedge_scalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Scalar { s: rs }: Scalar,
) -> Antiscalar {

  let e1234 = l1234*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_wedge_dual_number(
  Antiscalar { e1234: l1234 }: Antiscalar,
  DualNumber { s: rs, .. }: DualNumber,
) -> Antiscalar {

  let e1234 = l1234*rs;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_wedge_even_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  EvenGrade { s: rs, .. }: EvenGrade,
) -> Antiscalar {

  let e1234 = l1234*rs;

  Antiscalar { e1234 }
}

// dual number

#[rustfmt::skip]
#[inline]
fn dual_number_wedge_multivector(
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
  let e4 = ls*r4;

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
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
fn dual_number_wedge_scalar(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Scalar { s: rs }: Scalar,
) -> DualNumber {

  let s = ls*rs;
  let e1234 = l1234*rs;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_wedge_vector(
  DualNumber { s: ls, .. }: DualNumber,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Vector {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_wedge_bivector(
  DualNumber { s: ls, .. }: DualNumber,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Bivector {

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_wedge_trivector(
  DualNumber { s: ls, .. }: DualNumber,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Trivector {

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_wedge_antiscalar(
  DualNumber { s: ls, .. }: DualNumber,
  Antiscalar { e1234: r1234 }: Antiscalar
) -> Antiscalar {

  let e1234 = ls*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_wedge_dual_number(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let s = ls*rs;
  let e1234 = ls*r1234 + l1234*rs;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_wedge_odd_grade(
  DualNumber { s: ls, .. }: DualNumber,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_wedge_even_grade(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = ls*rs;

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
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
fn odd_grade_wedge_multivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Multivector {
    s: rs,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    ..
  }: Multivector,
) -> Multivector {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  let e423 = rs*l423 + l4*r23 + l3*r42 - l2*r43;
  let e431 = rs*l431 + l4*r31 + l1*r43 - l3*r41;
  let e412 = rs*l412 + l4*r12 + l2*r41 - l1*r42;
  let e321 = rs*l321 - l1*r23 - l3*r12 - l2*r31;

  let e1234 = (l4*r321 - l321*r4) + (l1*r423 - l423*r1)
    + (l2*r431 - l431*r2) + (l3*r412 - l412*r3);

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
fn odd_grade_wedge_scalar(
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

  let e423 = rs*l423;
  let e431 = rs*l431;
  let e412 = rs*l412;
  let e321 = rs*l321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_wedge_vector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> EvenGrade {

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  let e1234 = -l321*r4 - l423*r1 - l431*r2 - l412*r3;

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_wedge_bivector(
  OddGrade { e1: l1, e2: l2, e3: l3, e4: l4, .. }: OddGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Trivector {

  let e423 = l4*r23 + l3*r42 - l2*r43;
  let e431 = l4*r31 + l1*r43 - l3*r41;
  let e412 = l4*r12 + l2*r41 - l1*r42;
  let e321 = -l1*r23 - l3*r12 - l2*r31;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_wedge_trivector(
  OddGrade { e1: l1, e2: l2, e3: l3, e4: l4, .. }: OddGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Antiscalar {

  let e1234 = l4*r321 + l1*r423 + l2*r431 + l3*r412;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_wedge_dual_number(
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

  let e423 = rs*l423;
  let e431 = rs*l431;
  let e412 = rs*l412;
  let e321 = rs*l321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_wedge_odd_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
    ..
  }: OddGrade,
) -> EvenGrade {

  let e41 = l4*r1 - l1*r4;
  let e42 = l4*r2 - l2*r4;
  let e43 = l4*r3 - l3*r4;
  let e23 = l2*r3 - l3*r2;
  let e31 = l3*r1 - l1*r3;
  let e12 = l1*r2 - l2*r1;

  let e1234 = (l4*r321 - l321*r4) + (l1*r423 - l423*r1)
    + (l2*r431 - l431*r2) + (l3*r412 - l412*r3);

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_wedge_even_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    ..
  }: EvenGrade,
) -> OddGrade {

  let e1 = l1*rs;
  let e2 = l2*rs;
  let e3 = l3*rs;
  let e4 = l4*rs;

  let e423 = rs*l423 + l4*r23 + l3*r42 - l2*r43;
  let e431 = rs*l431 + l4*r31 + l1*r43 - l3*r41;
  let e412 = rs*l412 + l4*r12 + l2*r41 - l1*r42;
  let e321 = rs*l321 - l1*r23 - l3*r12 - l2*r31;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_wedge_multivector(
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

  let s = ls*rs;

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e41 = ls*r41 + l41*rs;
  let e42 = ls*r42 + l42*rs;
  let e43 = ls*r43 + l43*rs;
  let e23 = ls*r23 + l23*rs;
  let e31 = ls*r31 + l31*rs;
  let e12 = ls*r12 + l12*rs;

  let e423 = ls*r423 + l23*r4 + l42*r3 - l43*r2;
  let e431 = ls*r431 + l31*r4 + l43*r1 - l41*r3;
  let e412 = ls*r412 + l12*r4 + l41*r2 - l42*r1;
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
fn even_grade_wedge_scalar(
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
fn even_grade_wedge_vector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> OddGrade {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

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
fn even_grade_wedge_bivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> EvenGrade {

  let e41 = ls*r41;
  let e42 = ls*r42;
  let e43 = ls*r43;
  let e23 = ls*r23;
  let e31 = ls*r31;
  let e12 = ls*r12;

  let e1234 = -(l41*r23 + l23*r41) - (l42*r31 + l31*r42) - (l43*r12 + l12*r43);

  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_wedge_trivector(
  EvenGrade { s: ls, .. }: EvenGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Trivector {

  let e423 = ls*r423;
  let e431 = ls*r431;
  let e412 = ls*r412;
  let e321 = ls*r321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_wedge_antiscalar(
  EvenGrade { s: ls, .. }: EvenGrade,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Antiscalar {

  let e1234 = ls*r1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_wedge_dual_number(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> EvenGrade {

  let s = ls*rs;

  let e41 = l41*rs;
  let e42 = l42*rs;
  let e43 = l43*rs;
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
fn even_grade_wedge_odd_grade(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    ..
  }: EvenGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = ls*r1;
  let e2 = ls*r2;
  let e3 = ls*r3;
  let e4 = ls*r4;

  let e423 = ls*r423 + l23*r4 + l42*r3 - l43*r2;
  let e431 = ls*r431 + l31*r4 + l43*r1 - l41*r3;
  let e412 = ls*r412 + l12*r4 + l41*r2 - l42*r1;
  let e321 = ls*r321 - l23*r1 - l12*r3 - l31*r2;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_wedge_even_grade(
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

  let s = ls*rs;

  let e41 = ls*r41 + l41*rs;
  let e42 = ls*r42 + l42*rs;
  let e43 = ls*r43 + l43*rs;
  let e23 = ls*r23 + l23*rs;
  let e31 = ls*r31 + l31*rs;
  let e12 = ls*r12 + l12*rs;

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

  #[rustfmt::skip]
  const A: Multivector = Multivector {
    e1: 2., e2: 3., e3: 5., e4: 7.,
    ..Multivector::ZERO
  };
  #[rustfmt::skip]
  const B: Multivector = Multivector {
    e1: -11., e2: -13., e3: -17., e4: -19.,
    ..Multivector::ZERO
  };
  #[rustfmt::skip]
  const C: Multivector = Multivector {
    e1: -11., e2: -13., e3: -17., e4: -19.,
    ..Multivector::ZERO
  };

  #[test]
  fn vector_self_product() {
    assert_eq!(
      wedge(A, A),
      Multivector::zero(),
      "The Wedge Product of any Vector with itself should be zero"
    );
  }

  #[test]
  fn vector_product_anticommutivity() {
    assert_eq!(
      wedge(A, B),
      -wedge(B, A),
      "The Wedge Product of two Vectors anticommutes"
    );
  }

  #[test]
  fn vector_product_associativity() {
    assert_eq!(
      wedge(A, wedge(B, C)),
      wedge(wedge(A, B), C),
      "The Wedge Product of two Vectors is associative"
    );
  }

  #[test]
  fn vector_product_distributivity() {
    assert_eq!(
      wedge(A, B + C),
      wedge(A, B) + wedge(A, C),
      "The Wedge Product of two Vectors distributes over addition"
    );

    assert_eq!(
      wedge(A + B, C),
      wedge(A, C) + wedge(B, C),
      "The Wedge Product of two Vectors distributes over addition"
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
          Multivector::from(wedge(multivector, variant)),
          Multivector::from(wedge(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(wedge(scalar, variant)),
          Multivector::from(wedge(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(wedge(vector, variant)),
          Multivector::from(wedge(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(wedge(bivector, variant)),
          Multivector::from(wedge(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(wedge(trivector, variant)),
          Multivector::from(wedge(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(wedge(antiscalar, variant)),
          Multivector::from(wedge(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(wedge(dual_number, variant)),
          Multivector::from(wedge(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(wedge(odd_grade, variant)),
          Multivector::from(wedge(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(wedge(even_grade, variant)),
          Multivector::from(wedge(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
