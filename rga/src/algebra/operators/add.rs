use crate::{
  algebra::values::{
    zero, Antiscalar, Bivector, DualNumber, EvenGrade, Matrix4, Multivector,
    OddGrade, Scalar, Trivector, Vector,
  },
  helpers::impl_binary_operation,
};

pub use ::core::ops::Add;

/// a + b
#[inline]
pub fn add<Lhs, Rhs>(a: Lhs, b: Rhs) -> <Lhs as Add<Rhs>>::Output
where
  Lhs: Add<Rhs>,
{
  a.add(b)
}

impl_binary_operation!(Add::add {
  Multivector, Multivector => Multivector: multivector_add_multivector;
  Multivector, Scalar => Multivector: multivector_add_scalar;
  Multivector, Vector => Multivector: multivector_add_vector;
  Multivector, Bivector => Multivector: multivector_add_bivector;
  Multivector, Trivector => Multivector: multivector_add_trivector;
  Multivector, Antiscalar => Multivector: multivector_add_antiscalar;
  Multivector, DualNumber => Multivector: multivector_add_dual_number;
  Multivector, OddGrade => Multivector: multivector_add_odd_grade;
  Multivector, EvenGrade => Multivector: multivector_add_even_grade;

  Scalar, Multivector => Multivector: scalar_add_multivector;
  Scalar, Scalar => Scalar: scalar_add_scalar;
  Scalar, Vector => Multivector: scalar_add_vector;
  Scalar, Bivector => EvenGrade: scalar_add_bivector;
  Scalar, Trivector => Multivector: scalar_add_trivector;
  Scalar, Antiscalar => DualNumber: scalar_add_antiscalar;
  Scalar, DualNumber => DualNumber: scalar_add_dual_number;
  Scalar, OddGrade => Multivector: scalar_add_odd_grade;
  Scalar, EvenGrade => EvenGrade: scalar_add_even_grade;

  Vector, Multivector => Multivector: vector_add_multivector;
  Vector, Scalar => Multivector: vector_add_scalar;
  Vector, Vector => Vector: vector_add_vector;
  Vector, Bivector => Multivector: vector_add_bivector;
  Vector, Trivector => OddGrade: vector_add_trivector;
  Vector, Antiscalar => Multivector: vector_add_antiscalar;
  Vector, DualNumber => Multivector: vector_add_dual_number;
  Vector, OddGrade => OddGrade: vector_add_odd_grade;
  Vector, EvenGrade => Multivector: vector_add_even_grade;

  Bivector, Multivector => Multivector: bivector_add_multivector;
  Bivector, Scalar => EvenGrade: bivector_add_scalar;
  Bivector, Vector => Multivector: bivector_add_vector;
  Bivector, Bivector => Bivector: bivector_add_bivector;
  Bivector, Trivector => Multivector: bivector_add_trivector;
  Bivector, Antiscalar => EvenGrade: bivector_add_antiscalar;
  Bivector, DualNumber => EvenGrade: bivector_add_dual_number;
  Bivector, OddGrade => Multivector: bivector_add_odd_grade;
  Bivector, EvenGrade => EvenGrade: bivector_add_even_grade;

  Trivector, Multivector => Multivector: trivector_add_multivector;
  Trivector, Scalar => Multivector: trivector_add_scalar;
  Trivector, Vector => OddGrade: trivector_add_vector;
  Trivector, Bivector => Multivector: trivector_add_bivector;
  Trivector, Trivector => Trivector: trivector_add_trivector;
  Trivector, Antiscalar => Multivector: trivector_add_antiscalar;
  Trivector, DualNumber => Multivector: trivector_add_dual_number;
  Trivector, OddGrade => OddGrade: trivector_add_odd_grade;
  Trivector, EvenGrade => Multivector: trivector_add_even_grade;

  Antiscalar, Multivector => Multivector: antiscalar_add_multivector;
  Antiscalar, Scalar => DualNumber: antiscalar_add_scalar;
  Antiscalar, Vector => Multivector: antiscalar_add_vector;
  Antiscalar, Bivector => EvenGrade: antiscalar_add_bivector;
  Antiscalar, Trivector => Multivector: antiscalar_add_trivector;
  Antiscalar, Antiscalar => Antiscalar: antiscalar_add_antiscalar;
  Antiscalar, DualNumber => DualNumber: antiscalar_add_dual_number;
  Antiscalar, OddGrade => Multivector: antiscalar_add_odd_grade;
  Antiscalar, EvenGrade => EvenGrade: antiscalar_add_even_grade;

  DualNumber, Multivector => Multivector: dual_number_add_multivector;
  DualNumber, Scalar => DualNumber: dual_number_add_scalar;
  DualNumber, Vector => Multivector: dual_number_add_vector;
  DualNumber, Bivector => EvenGrade: dual_number_add_bivector;
  DualNumber, Trivector => Multivector: dual_number_add_trivector;
  DualNumber, Antiscalar => DualNumber: dual_number_add_antiscalar;
  DualNumber, DualNumber => DualNumber: dual_number_add_dual_number;
  DualNumber, OddGrade => Multivector: dual_number_add_odd_grade;
  DualNumber, EvenGrade => EvenGrade: dual_number_add_even_grade;

  OddGrade, Multivector => Multivector: odd_grade_add_multivector;
  OddGrade, Scalar => Multivector: odd_grade_add_scalar;
  OddGrade, Vector => OddGrade: odd_grade_add_vector;
  OddGrade, Bivector => Multivector: odd_grade_add_bivector;
  OddGrade, Trivector => OddGrade: odd_grade_add_trivector;
  OddGrade, Antiscalar => Multivector: odd_grade_add_antiscalar;
  OddGrade, DualNumber => Multivector: odd_grade_add_dual_number;
  OddGrade, OddGrade => OddGrade: odd_grade_add_odd_grade;
  OddGrade, EvenGrade => Multivector: odd_grade_add_even_grade;

  EvenGrade, Multivector => Multivector: even_grade_add_multivector;
  EvenGrade, Scalar => EvenGrade: even_grade_add_scalar;
  EvenGrade, Vector => Multivector: even_grade_add_vector;
  EvenGrade, Bivector => EvenGrade: even_grade_add_bivector;
  EvenGrade, Trivector => Multivector: even_grade_add_trivector;
  EvenGrade, Antiscalar => EvenGrade: even_grade_add_antiscalar;
  EvenGrade, DualNumber => EvenGrade: even_grade_add_dual_number;
  EvenGrade, OddGrade => Multivector: even_grade_add_odd_grade;
  EvenGrade, EvenGrade => EvenGrade: even_grade_add_even_grade;

  Matrix4, Matrix4 => Matrix4: matrix4_add_matrix4;
});

// multivector

#[rustfmt::skip]
#[inline]
fn multivector_add_multivector(
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

  let s = ls + rs;

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

  let e1234 = l1234 + r1234;

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
fn multivector_add_scalar(
  Multivector {
    s: ls,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }: Multivector,
  Scalar { s: rs }: Scalar,
) -> Multivector {

  let s = ls + rs;

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
fn multivector_add_vector(
  Multivector {
    s,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }: Multivector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Multivector {

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

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
fn multivector_add_bivector(
  Multivector {
    s,
    e1, e2, e3, e4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423, e431, e412, e321,
    e1234,
  }: Multivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Multivector {

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

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
fn multivector_add_trivector(
  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234,
  }: Multivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Multivector {

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

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
fn multivector_add_antiscalar(
  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234: l1234,
  }: Multivector,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Multivector {

  let e1234 = l1234 + r1234;

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
fn multivector_add_dual_number(
  Multivector {
    s: ls,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234: l1234,
  }: Multivector,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> Multivector {

  let s = ls + rs;
  let e1234 = l1234 + r1234;

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
fn multivector_add_odd_grade(
  Multivector {
    s,
    e1: l1, e2: l2, e3: l3, e4: l4,
    e41, e42, e43, e23, e31, e12,
    e423: l423, e431: l431, e412: l412, e321: l321,
    e1234,
  }: Multivector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> Multivector {

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

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
fn multivector_add_even_grade(
  Multivector {
    s: ls,
    e1, e2, e3, e4,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e423, e431, e412, e321,
    e1234: l1234,
  }: Multivector,
  EvenGrade {
    s: rs,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234: r1234,
  }: EvenGrade,
) -> Multivector {

  let s = ls + rs;

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

  let e1234 = l1234 + r1234;

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
fn scalar_add_multivector(
  Scalar { s: ls }: Scalar,
  Multivector {
    s: rs,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }: Multivector,
) -> Multivector {

  let s = ls + rs;

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }
}

#[inline]
fn scalar_add_scalar(
  Scalar { s: ls }: Scalar,
  Scalar { s: rs }: Scalar,
) -> Scalar {
  Scalar { s: ls + rs }
}

#[rustfmt::skip]
#[inline]
fn scalar_add_vector(
  Scalar { s }: Scalar,
  Vector { e1, e2, e3, e4 }: Vector,
) -> Multivector {

  Multivector {
    s,
    e1, e2, e3, e4,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_add_bivector(
  Scalar { s }: Scalar,
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
) -> EvenGrade {

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_add_trivector(
  Scalar { s }: Scalar,
  Trivector { e423, e431, e412, e321 }: Trivector,
) -> Multivector {

  Multivector {
    s,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_add_antiscalar(
  Scalar { s }: Scalar,
  Antiscalar { e1234 }: Antiscalar,
) -> DualNumber {

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn scalar_add_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { s: rs, e1234 }: DualNumber,
) -> DualNumber {

  let s = ls + rs;

  DualNumber {
    s,
    e1234
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_add_odd_grade(
  Scalar { s }: Scalar,
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn scalar_add_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade {
    s: rs,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = ls + rs;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234
  }
}

// vector

#[rustfmt::skip]
#[inline]
fn vector_add_multivector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Multivector {
    s,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }: Multivector,
) -> Multivector {

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

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
fn vector_add_scalar(
  Vector { e1, e2, e3, e4 }: Vector,
  Scalar { s }: Scalar,
) -> Multivector {

  Multivector {
    s,
    e1, e2, e3, e4,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_add_vector(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> Vector {

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn vector_add_bivector(
  Vector { e1, e2, e3, e4 }: Vector,
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
) -> Multivector {
  Multivector {
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_add_trivector(
  Vector { e1, e2, e3, e4 }: Vector,
  Trivector { e423, e431, e412, e321 }: Trivector,
) -> OddGrade {
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_add_antiscalar(
  Vector { e1, e2, e3, e4 }: Vector,
  Antiscalar { e1234 }: Antiscalar,
) -> Multivector {
  Multivector {
    e1, e2, e3, e4,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_add_dual_number(
  Vector { e1, e2, e3, e4 }: Vector,
  DualNumber { s, e1234 }: DualNumber,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn vector_add_odd_grade(
  Vector { e1: l1, e2: l2, e3: l3, e4: l4 }: Vector,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423, e431, e412, e321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_add_even_grade(
  Vector { e1, e2, e3, e4 }: Vector,
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

// bivector

#[rustfmt::skip]
#[inline]
fn bivector_add_multivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Multivector {
    s,
    e1, e2, e3, e4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423, e431, e412, e321,
    e1234,
  }: Multivector,
) -> Multivector {

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

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
fn bivector_add_scalar(
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
  Scalar { s }: Scalar,
) -> EvenGrade {
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_add_vector(
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
  Vector { e1, e2, e3, e4 }: Vector,
) -> Multivector {
  Multivector {
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_add_bivector(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> Bivector {

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn bivector_add_trivector(
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
  Trivector { e423, e431, e412, e321 }: Trivector,
) -> Multivector {
  Multivector {
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_add_antiscalar(
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
  Antiscalar { e1234 }: Antiscalar,
) -> EvenGrade {
  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_add_dual_number(
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
  DualNumber { s, e1234 }: DualNumber,
) -> EvenGrade {
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_add_odd_grade(
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
) -> Multivector {
  Multivector {
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_add_even_grade(
  Bivector {
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
  }: Bivector,
  EvenGrade {
    s,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e1234,
  }: EvenGrade,
) -> EvenGrade {

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234
  }
}

// trivector

#[rustfmt::skip]
#[inline]
fn trivector_add_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234,
  }: Multivector,
) -> Multivector {

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

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
fn trivector_add_scalar(
  Trivector { e423, e431, e412, e321 }: Trivector,
  Scalar { s }: Scalar,
) -> Multivector {
  Multivector {
    s,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_add_vector(
  Trivector { e423, e431, e412, e321 }: Trivector,
  Vector { e1, e2, e3, e4 }: Vector,
) -> OddGrade {
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_add_bivector(
  Trivector { e423, e431, e412, e321 }: Trivector,
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
) -> Multivector {
  Multivector {
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_add_trivector(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> Trivector {

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

  Trivector {
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_add_antiscalar(
  Trivector { e423, e431, e412, e321 }: Trivector,
  Antiscalar { e1234 }: Antiscalar,
) -> Multivector {
  Multivector {
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_add_dual_number(
  Trivector { e423, e431, e412, e321 }: Trivector,
  DualNumber { s, e1234 }: DualNumber,
) -> Multivector {
  Multivector {
    s,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_add_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, e321: l321 }: Trivector,
  OddGrade {
    e1, e2, e3, e4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_add_even_grade(
  Trivector { e423, e431, e412, e321 }: Trivector,
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
) -> Multivector {
  Multivector {
    s,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

// antiscalar

#[rustfmt::skip]
#[inline]
fn antiscalar_add_multivector(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let e1234 = l1234 + r1234;

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
fn antiscalar_add_scalar(
  Antiscalar { e1234 }: Antiscalar,
  Scalar { s }: Scalar,
) -> DualNumber {
  DualNumber {
    s,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_add_vector(
  Antiscalar { e1234 }: Antiscalar,
  Vector { e1, e2, e3, e4 }: Vector,
) -> Multivector {
  Multivector {
    e1, e2, e3, e4,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_add_bivector(
  Antiscalar { e1234 }: Antiscalar,
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
) -> EvenGrade {
  EvenGrade {
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_add_trivector(
  Antiscalar { e1234 }: Antiscalar,
  Trivector { e423, e431, e412, e321 }: Trivector,
) -> Multivector {
  Multivector {
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_add_antiscalar(
  Antiscalar { e1234: l1234 }: Antiscalar,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> Antiscalar {

  let e1234 = l1234 + r1234;

  Antiscalar {
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_add_dual_number(
  Antiscalar { e1234: l1234 }: Antiscalar,
  DualNumber { s, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let e1234 = l1234 + r1234;

  DualNumber {
    s,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_add_odd_grade(
  Antiscalar { e1234 }: Antiscalar,
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
) -> Multivector {
  Multivector {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_add_even_grade(
  Antiscalar { e1234: l1234 }: Antiscalar,
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let e1234 = l1234 + r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

// dual number

#[rustfmt::skip]
#[inline]
fn dual_number_add_multivector(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  Multivector {
    s: rs,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = ls + rs;
  let e1234 = l1234 + r1234;

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
fn dual_number_add_scalar(
  DualNumber { s: ls, e1234 }: DualNumber,
  Scalar { s: rs }: Scalar,
) -> DualNumber {

  let s = ls + rs;

  DualNumber {
    s,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_add_vector(
  DualNumber { s, e1234 }: DualNumber,
  Vector { e1, e2, e3, e4 }: Vector,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_add_bivector(
  DualNumber { s, e1234 }: DualNumber,
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
) -> EvenGrade {
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_add_trivector(
  DualNumber { s, e1234 }: DualNumber,
  Trivector { e423, e431, e412, e321 }: Trivector,
) -> Multivector {
  Multivector {
    s,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_add_antiscalar(
  DualNumber { s, e1234: l1234 }: DualNumber,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> DualNumber {

  let e1234 = l1234 + r1234;

  DualNumber {
    s,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_add_dual_number(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> DualNumber {

  let s = ls + rs;
  let e1234 = l1234 + r1234;

  DualNumber {
    s,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_add_odd_grade(
  DualNumber { s, e1234 }: DualNumber,
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_add_even_grade(
  DualNumber { s: ls, e1234: l1234 }: DualNumber,
  EvenGrade {
    s: rs,
    e41, e42, e43, e23, e31, e12,
    e1234: r1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = ls + rs;
  let e1234 = l1234 + r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

// odd grade

#[rustfmt::skip]
#[inline]
fn odd_grade_add_multivector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Multivector {
    s,
    e1: r1, e2: r2, e3: r3, e4: r4,
    e41, e42, e43, e23, e31, e12,
    e423: r423, e431: r431, e412: r412, e321: r321,
    e1234,
  }: Multivector,
) -> Multivector {

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

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
fn odd_grade_add_scalar(
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
  Scalar { s }: Scalar,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_add_vector(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423, e431, e412, e321,
  }: OddGrade,
  Vector { e1: r1, e2: r2, e3: r3, e4: r4 }: Vector,
) -> OddGrade {

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_add_bivector(
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
  Bivector { e41, e42, e43, e23, e31, e12 }: Bivector,
) -> Multivector {
  Multivector {
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_add_trivector(
  OddGrade {
    e1, e2, e3, e4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  Trivector { e423: r423, e431: r431, e412: r412, e321: r321 }: Trivector,
) -> OddGrade {

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_add_antiscalar(
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
  Antiscalar { e1234 }: Antiscalar,
) -> Multivector {
  Multivector {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_add_dual_number(
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
  DualNumber { s, e1234 }: DualNumber,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_add_odd_grade(
  OddGrade {
    e1: l1, e2: l2, e3: l3, e4: l4,
    e423: l423, e431: l431, e412: l412, e321: l321,
  }: OddGrade,
  OddGrade {
    e1: r1, e2: r2, e3: r3, e4: r4,
    e423: r423, e431: r431, e412: r412, e321: r321,
  }: OddGrade,
) -> OddGrade {

  let e1 = l1 + r1;
  let e2 = l2 + r2;
  let e3 = l3 + r3;
  let e4 = l4 + r4;

  let e423 = l423 + r423;
  let e431 = l431 + r431;
  let e412 = l412 + r412;
  let e321 = l321 + r321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_add_even_grade(
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }
}

// even grade

#[rustfmt::skip]
#[inline]
fn even_grade_add_multivector(
  EvenGrade {
    s: ls,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234: l1234,
  }: EvenGrade,
  Multivector {
    s: rs,
    e1, e2, e3, e4,
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
    e423, e431, e412, e321,
    e1234: r1234,
  }: Multivector,
) -> Multivector {

  let s = ls + rs;

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

  let e1234 = l1234 + r1234;

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
fn even_grade_add_scalar(
  EvenGrade {
    s: ls,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
  Scalar { s: rs }: Scalar,
) -> EvenGrade {

  let s = ls + rs;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_add_vector(
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
  Vector { e1, e2, e3, e4 }: Vector,
) -> Multivector {
  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_add_bivector(
  EvenGrade {
    s,
    e41: l41, e42: l42, e43: l43, e23: l23, e31: l31, e12: l12,
    e1234,
  }: EvenGrade,
  Bivector {
    e41: r41, e42: r42, e43: r43, e23: r23, e31: r31, e12: r12,
  }: Bivector,
) -> EvenGrade {

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_add_trivector(
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
  Trivector { e423, e431, e412, e321 }: Trivector,
) -> Multivector {
  Multivector {
    s,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_add_antiscalar(
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234: l1234,
  }: EvenGrade,
  Antiscalar { e1234: r1234 }: Antiscalar,
) -> EvenGrade {

  let e1234 = l1234 + r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_add_dual_number(
  EvenGrade {
    s: ls,
    e41, e42, e43, e23, e31, e12,
    e1234: l1234,
  }: EvenGrade,
  DualNumber { s: rs, e1234: r1234 }: DualNumber,
) -> EvenGrade {

  let s = ls + rs;
  let e1234 = l1234 + r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn even_grade_add_odd_grade(
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
) -> Multivector {
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
fn even_grade_add_even_grade(
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

  let s = ls + rs;

  let e41 = l41 + r41;
  let e42 = l42 + r42;
  let e43 = l43 + r43;
  let e23 = l23 + r23;
  let e31 = l31 + r31;
  let e12 = l12 + r12;

  let e1234 = l1234 + r1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn matrix4_add_matrix4(
  Matrix4 {
    m11: a11, m12: a12, m13: a13, m14: a14,
    m21: a21, m22: a22, m23: a23, m24: a24,
    m31: a31, m32: a32, m33: a33, m34: a34,
    m41: a41, m42: a42, m43: a43, m44: a44,
  }: Matrix4,
  Matrix4 {
    m11: b11, m12: b12, m13: b13, m14: b14,
    m21: b21, m22: b22, m23: b23, m24: b24,
    m31: b31, m32: b32, m33: b33, m34: b34,
    m41: b41, m42: b42, m43: b43, m44: b44,
  }: Matrix4,
) -> Matrix4 {
  let m11 = a11 + b11;
  let m21 = a21 + b21;
  let m31 = a31 + b31;
  let m41 = a41 + b41;

  let m12 = a12 + b12;
  let m22 = a22 + b22;
  let m32 = a32 + b32;
  let m42 = a42 + b42;

  let m13 = a13 + b13;
  let m23 = a23 + b23;
  let m33 = a33 + b33;
  let m43 = a43 + b43;

  let m14 = a14 + b14;
  let m24 = a24 + b24;
  let m34 = a34 + b34;
  let m44 = a44 + b44;

  Matrix4 {
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44,
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    algebra::{operators::*, values::*},
    helpers::def_for_each,
    test_values::*,
  };

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
          Multivector::from(multivector + variant),
          Multivector::from(multivector) + Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(scalar + variant),
          Multivector::from(scalar) + Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(vector + variant),
          Multivector::from(vector) + Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bivector + variant),
          Multivector::from(bivector) + Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(trivector + variant),
          Multivector::from(trivector) + Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiscalar + variant),
          Multivector::from(antiscalar) + Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = DualNumber {
          s: MULTIVECTOR_A.s,
          e1234: MULTIVECTOR_A.e1234,
        };
        assert_eq!(
          Multivector::from(dual_number + variant),
          Multivector::from(dual_number) + Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = OddGrade {
          e1: MULTIVECTOR_A.e1,
          e2: MULTIVECTOR_A.e2,
          e3: MULTIVECTOR_A.e3,
          e4: MULTIVECTOR_A.e4,
          e423: MULTIVECTOR_A.e423,
          e431: MULTIVECTOR_A.e431,
          e412: MULTIVECTOR_A.e412,
          e321: MULTIVECTOR_A.e321,
        };
        assert_eq!(
          Multivector::from(odd_grade + variant),
          Multivector::from(odd_grade) + Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = EvenGrade {
          s: MULTIVECTOR_A.s,
          e41: MULTIVECTOR_A.e41,
          e42: MULTIVECTOR_A.e42,
          e43: MULTIVECTOR_A.e43,
          e23: MULTIVECTOR_A.e23,
          e31: MULTIVECTOR_A.e31,
          e12: MULTIVECTOR_A.e12,
          e1234: MULTIVECTOR_A.e1234,
        };
        assert_eq!(
          Multivector::from(even_grade + variant),
          Multivector::from(even_grade) + Multivector::from(variant)
        );
      }
    }
  }
}
