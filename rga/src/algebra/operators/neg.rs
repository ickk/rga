use crate::{
  algebra::values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Matrix4, Multivector,
    OddGrade, Scalar, Trivector, Vector,
  },
  helpers::impl_unary_operation,
};

pub use ::core::ops::Neg;

impl_unary_operation!(Neg::neg {
  Multivector => Multivector: neg_multivector;
  Scalar => Scalar: neg_scalar;
  Vector => Vector: neg_vector;
  Bivector => Bivector: neg_bivector;
  Trivector => Trivector: neg_trivector;
  Antiscalar => Antiscalar: neg_antiscalar;
  DualNumber => DualNumber: neg_dual_number;
  OddGrade => OddGrade: neg_odd_grade;
  EvenGrade => EvenGrade: neg_even_grade;
  Matrix4 => Matrix4: neg_matrix4;
});

#[rustfmt::skip]
#[inline]
fn neg_multivector(
  Multivector {
    s: ms,
    e1: m1, e2: m2, e3: m3, e4: m4,
    e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
    e423: m423, e431: m431, e412: m412, e321: m321,
    e1234: m1234,
  }: Multivector,
) -> Multivector {

  let s = -ms;

  let e1 = -m1;
  let e2 = -m2;
  let e3 = -m3;
  let e4 = -m4;

  let e41 = -m41;
  let e42 = -m42;
  let e43 = -m43;
  let e23 = -m23;
  let e31 = -m31;
  let e12 = -m12;

  let e423 = -m423;
  let e431 = -m431;
  let e412 = -m412;
  let e321 = -m321;

  let e1234 = -m1234;

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
fn neg_scalar(
  Scalar { s: ms }: Scalar,
) -> Scalar {

  let s = -ms;

  Scalar { s }
}

#[rustfmt::skip]
#[inline]
fn neg_vector(
  Vector { e1: m1, e2: m2, e3: m3, e4: m4 }: Vector,
) -> Vector {

  let e1 = -m1;
  let e2 = -m2;
  let e3 = -m3;
  let e4 = -m4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn neg_bivector(
  Bivector {
    e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
  }: Bivector,
) -> Bivector {

  let e41 = -m41;
  let e42 = -m42;
  let e43 = -m43;
  let e23 = -m23;
  let e31 = -m31;
  let e12 = -m12;

  Bivector {
    e41, e42, e43, e23, e31, e12,
  }
}

#[rustfmt::skip]
#[inline]
fn neg_trivector(
  Trivector { e423: m423, e431: m431, e412: m412, e321: m321 }: Trivector,
) -> Trivector {

  let e423 = -m423;
  let e431 = -m431;
  let e412 = -m412;
  let e321 = -m321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn neg_antiscalar(
  Antiscalar { e1234: m1234 }: Antiscalar,
) -> Antiscalar {

  let e1234 = -m1234;

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
fn neg_dual_number(
  DualNumber { s: ms, e1234: m1234 }: DualNumber,
) -> DualNumber {

  let s = -ms;
  let e1234 = -m1234;

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn neg_odd_grade(
  OddGrade {
    e1: m1, e2: m2, e3: m3, e4: m4,
    e423: m423, e431: m431, e412: m412, e321: m321,
  }: OddGrade,
) -> OddGrade {

  let e1 = -m1;
  let e2 = -m2;
  let e3 = -m3;
  let e4 = -m4;

  let e423 = -m423;
  let e431 = -m431;
  let e412 = -m412;
  let e321 = -m321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn neg_even_grade(
  EvenGrade {
    s: ms,
    e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
    e1234: m1234,
  }: EvenGrade,
) -> EvenGrade {

  let s = -ms;

  let e41 = -m41;
  let e42 = -m42;
  let e43 = -m43;
  let e23 = -m23;
  let e31 = -m31;
  let e12 = -m12;

  let e1234 = -m1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234
  }
}

#[rustfmt::skip]
#[inline]
fn neg_matrix4(
  Matrix4 {
    m11: a11, m12: a12, m13: a13, m14: a14,
    m21: a21, m22: a22, m23: a23, m24: a24,
    m31: a31, m32: a32, m33: a33, m34: a34,
    m41: a41, m42: a42, m43: a43, m44: a44,
  }: Matrix4,
) -> Matrix4 {
  let m11 = -a11;
  let m21 = -a21;
  let m31 = -a31;
  let m41 = -a41;

  let m12 = -a12;
  let m22 = -a22;
  let m32 = -a32;
  let m42 = -a42;

  let m13 = -a13;
  let m23 = -a23;
  let m33 = -a33;
  let m43 = -a43;

  let m14 = -a14;
  let m24 = -a24;
  let m34 = -a34;
  let m44 = -a44;

  Matrix4 {
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44,
  }
}
