use crate::{
  algebra::{
    operators::{GeometricProduct, Inverse},
    values::{
      zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector,
      OddGrade, Scalar, Trivector, Vector,
    },
  },
  helpers::{
    impl_binary_operation, return_conjugate_rhs, return_neg_rhs, return_rhs,
    return_scalar_nan_binary,
  },
};

/// a ⟑ b ⟑ a⁻¹
#[inline]
pub fn sandwich<Operator, Argument>(
  a: Operator,
  b: Argument,
) -> <Operator as SandwichProduct<Argument>>::Output
where
  Operator: SandwichProduct<Argument>,
{
  a.sandwich(b)
}

/// a ⟑ b ⟑ a⁻¹
pub trait SandwichProduct<Arg> {
  type Output;

  /// a ⟑ b ⟑ a⁻¹
  ///
  /// Geometric sandwich product with the inverse applied on the right
  #[doc(alias = "product")]
  fn sandwich(self, b: Arg) -> Self::Output;
}

fn general_impl<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <<Lhs as GeometricProduct<Rhs>>::Output as GeometricProduct<Lhs>>::Output
where
  Lhs: Copy + Inverse + GeometricProduct<Rhs>,
  <Lhs as GeometricProduct<Rhs>>::Output: GeometricProduct<Lhs>,
{
  a.geometric_product(b).geometric_product(a.inverse())
}

impl_binary_operation!(SandwichProduct::sandwich {
  Multivector, Multivector => Multivector: general_impl;
  Multivector, Scalar => Scalar: return_rhs;
  Multivector, Vector => Multivector: general_impl;
  Multivector, Bivector => Multivector: general_impl;
  Multivector, Trivector => Multivector: general_impl;
  Multivector, Antiscalar => Multivector: general_impl;
  Multivector, DualNumber => Multivector: general_impl;
  Multivector, EvenGrade => Multivector: general_impl;
  Multivector, OddGrade => Multivector: general_impl;

  Scalar, Multivector => Multivector: return_rhs;
  Scalar, Scalar => Scalar: return_rhs;
  Scalar, Vector => Vector: return_rhs;
  Scalar, Bivector => Bivector: return_rhs;
  Scalar, Trivector => Trivector: return_rhs;
  Scalar, Antiscalar => Antiscalar: return_rhs;
  Scalar, DualNumber => DualNumber: return_rhs;
  Scalar, EvenGrade => EvenGrade: return_rhs;
  Scalar, OddGrade => OddGrade: return_rhs;

  Vector, Multivector => Multivector: vector_sandwich_multivector;
  Vector, Scalar => Scalar: return_rhs;
  Vector, Vector => Vector: vector_sandwich_vector;
  Vector, Bivector => Bivector: vector_sandwich_bivector;
  Vector, Trivector => Trivector: vector_sandwich_trivector;
  Vector, Antiscalar => Antiscalar: return_neg_rhs;
  Vector, DualNumber => DualNumber: return_conjugate_rhs;
  Vector, EvenGrade => EvenGrade: vector_sandwich_even_grade;
  Vector, OddGrade => OddGrade: vector_sandwich_odd_grade;

  Bivector, Multivector => Multivector: bivector_sandwich_multivector;
  Bivector, Scalar => Scalar: return_rhs;
  Bivector, Vector => OddGrade: bivector_sandwich_vector;
  Bivector, Bivector => Bivector: bivector_sandwich_bivector;
  Bivector, Trivector => OddGrade: bivector_sandwich_trivector;
  Bivector, Antiscalar => Antiscalar: return_rhs;
  Bivector, DualNumber => DualNumber: return_rhs;
  Bivector, EvenGrade => EvenGrade: bivector_sandwich_evengrade;
  Bivector, OddGrade => OddGrade: bivector_sandwich_oddgrade;

  Trivector, Multivector => Multivector: trivector_sandwich_multivector;
  Trivector, Scalar => Scalar: return_rhs;
  Trivector, Vector => Vector: trivector_sandwich_vector;
  Trivector, Bivector => Bivector: trivector_sandwich_bivector;
  Trivector, Trivector => Trivector: trivector_sandwich_trivector;
  Trivector, Antiscalar => Antiscalar: return_neg_rhs;
  Trivector, DualNumber => DualNumber: return_conjugate_rhs;
  Trivector, EvenGrade => EvenGrade: trivector_sandwich_even_grade;
  Trivector, OddGrade => OddGrade: trivector_sandwich_odd_grade;

  Antiscalar, Multivector => Scalar: return_scalar_nan_binary;
  Antiscalar, Scalar => Scalar: return_scalar_nan_binary;
  Antiscalar, Vector => Scalar: return_scalar_nan_binary;
  Antiscalar, Bivector => Scalar: return_scalar_nan_binary;
  Antiscalar, Trivector => Scalar: return_scalar_nan_binary;
  Antiscalar, Antiscalar => Scalar: return_scalar_nan_binary;
  Antiscalar, DualNumber => Scalar: return_scalar_nan_binary;
  Antiscalar, EvenGrade => Scalar: return_scalar_nan_binary;
  Antiscalar, OddGrade => Scalar: return_scalar_nan_binary;

  DualNumber, Multivector => Multivector: dual_number_sandwich_multivector;
  DualNumber, Scalar => Scalar: return_rhs;
  DualNumber, Vector => OddGrade: dual_number_sandwich_vector;
  DualNumber, Bivector => Bivector: return_rhs;
  DualNumber, Trivector => OddGrade: dual_number_sandwich_trivector;
  DualNumber, Antiscalar => Antiscalar: return_rhs;
  DualNumber, DualNumber => DualNumber: return_rhs;
  DualNumber, EvenGrade => EvenGrade: return_rhs;
  DualNumber, OddGrade => OddGrade: dual_number_sandwich_odd_grade;

  EvenGrade, Multivector => Multivector: evengrade_sandwich_multivector;
  EvenGrade, Scalar => Scalar: return_rhs;
  EvenGrade, Vector => OddGrade: evengrade_sandwich_vector;
  EvenGrade, Bivector => Bivector: evengrade_sandwich_bivector;
  EvenGrade, Trivector => OddGrade: evengrade_sandwich_trivector;
  EvenGrade, Antiscalar => Antiscalar: return_rhs;
  EvenGrade, DualNumber => DualNumber: return_rhs;
  EvenGrade, EvenGrade => EvenGrade: evengrade_sandwich_evengrade;
  EvenGrade, OddGrade => OddGrade: evengrade_sandwich_oddgrade;

  OddGrade, Multivector => Multivector: odd_grade_sandwich_multivector;
  OddGrade, Scalar => Scalar: return_rhs;
  OddGrade, Vector => OddGrade: odd_grade_sandwich_vector;
  OddGrade, Bivector => Bivector: odd_grade_sandwich_bivector;
  OddGrade, Trivector => OddGrade: odd_grade_sandwich_trivector;
  OddGrade, Antiscalar => Antiscalar: return_neg_rhs;
  OddGrade, DualNumber => DualNumber: return_conjugate_rhs;
  OddGrade, EvenGrade => EvenGrade: odd_grade_sandwich_even_grade;
  OddGrade, OddGrade => OddGrade: odd_grade_sandwich_odd_grade;
});

#[rustfmt::skip]
#[inline]
fn vector_sandwich_multivector(
  Vector { e1: o1, e2: o2, e3: o3, e4: o4 }: Vector,
  Multivector {
    s: a_s,
    e1: a1, e2: a2, e3: a3, e4: a4,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e423: a423, e431: a431, e412: a412, e321: a321,
    e1234: a1234,
  }: Multivector,
) -> Multivector {
  let c =   o1*o1 + o2*o2 + o3*o3;
  let d =   o1*o1 - o2*o2 - o3*o3;
  let e = - o1*o1 + o2*o2 - o3*o3;
  let f = - o1*o1 - o2*o2 + o3*o3;

  let cr = 1./c;

  let m = o4*o1;
  let n = o4*o2;
  let o = o4*o3;
  let j = o1*o2;
  let k = o2*o3;
  let l = o3*o1;

  let s = a_s;

  let e1 = cr*(d*a1 + 2.*(j*a2 + l*a3));
  let e2 = cr*(e*a2 + 2.*(k*a3 + j*a1));
  let e3 = cr*(f*a3 + 2.*(l*a1 + k*a2));

  let e4 = -a4 + 2.*cr*(m*a1 + n*a2 + o*a3);

  let e23 = cr*(d*a23 + 2.*(j*a31 + l*a12));
  let e31 = cr*(e*a31 + 2.*(k*a12 + j*a23));
  let e12 = cr*(f*a12 + 2.*(l*a23 + k*a31));

  let e41 = -cr*(d*a41 + 2.*(o*a31 - n*a12 + j*a42 + l*a43));
  let e42 = -cr*(e*a42 + 2.*(m*a12 - o*a23 + k*a43 + j*a41));
  let e43 = -cr*(f*a43 + 2.*(n*a23 - m*a31 + l*a41 + k*a42));

  let e423 = -cr*(d*a423 + 2.*(m*a321 + j*a431 + l*a412));
  let e431 = -cr*(e*a431 + 2.*(n*a321 + k*a412 + j*a423));
  let e412 = -cr*(f*a412 + 2.*(o*a321 + l*a423 + k*a431));

  let e321 = a321;

  let e1234 = -a1234;

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
fn vector_sandwich_vector(
  Vector { e1: o1, e2: o2, e3: o3, e4: o4 }: Vector,
  Vector { e1: a1, e2: a2, e3: a3, e4: a4 }: Vector,
) -> Vector {
  let c =   o1*o1 + o2*o2 + o3*o3;
  let d =   o1*o1 - o2*o2 - o3*o3;
  let e = - o1*o1 + o2*o2 - o3*o3;
  let f = - o1*o1 - o2*o2 + o3*o3;

  let cr = 1./c;

  let m = o4*o1;
  let n = o4*o2;
  let o = o4*o3;
  let j = o1*o2;
  let k = o2*o3;
  let l = o3*o1;

  let e1 = cr*(d*a1 + 2.*(j*a2 + l*a3));
  let e2 = cr*(e*a2 + 2.*(k*a3 + j*a1));
  let e3 = cr*(f*a3 + 2.*(l*a1 + k*a2));
  let e4 = -a4 + 2.*cr*(m*a1 + n*a2 + o*a3);

  Vector {
    e1, e2, e3, e4,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_sandwich_bivector(
  Vector { e1: o1, e2: o2, e3: o3, e4: o4 }: Vector,
  Bivector {
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
  }: Bivector,
) -> Bivector {
  let c =   o1*o1 + o2*o2 + o3*o3;
  let d =   o1*o1 - o2*o2 - o3*o3;
  let e = - o1*o1 + o2*o2 - o3*o3;
  let f = - o1*o1 - o2*o2 + o3*o3;

  let cr = 1./c;

  let m = o4*o1;
  let n = o4*o2;
  let o = o4*o3;
  let j = o1*o2;
  let k = o2*o3;
  let l = o3*o1;

  let e23 = cr*(d*a23 + 2.*(j*a31 + l*a12));
  let e31 = cr*(e*a31 + 2.*(k*a12 + j*a23));
  let e12 = cr*(f*a12 + 2.*(l*a23 + k*a31));

  let e41 = -cr*(d*a41 + 2.*(o*a31 - n*a12 + j*a42 + l*a43));
  let e42 = -cr*(e*a42 + 2.*(m*a12 - o*a23 + k*a43 + j*a41));
  let e43 = -cr*(f*a43 + 2.*(n*a23 - m*a31 + l*a41 + k*a42));

  Bivector {
    e41, e42, e43, e23, e31, e12,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_sandwich_trivector(
  Vector { e1: o1, e2: o2, e3: o3, e4: o4 }: Vector,
  Trivector { e423: a423, e431: a431, e412: a412, e321: a321 }: Trivector,
) -> Trivector {
  let c =   o1*o1 + o2*o2 + o3*o3;
  let d =   o1*o1 - o2*o2 - o3*o3;
  let e = - o1*o1 + o2*o2 - o3*o3;
  let f = - o1*o1 - o2*o2 + o3*o3;

  let cr = 1./c;

  let m = o4*o1;
  let n = o4*o2;
  let o = o4*o3;
  let j = o1*o2;
  let k = o2*o3;
  let l = o3*o1;

  let e423 = -cr*(d*a423 + 2.*(m*a321 + j*a431 + l*a412));
  let e431 = -cr*(e*a431 + 2.*(n*a321 + k*a412 + j*a423));
  let e412 = -cr*(f*a412 + 2.*(o*a321 + l*a423 + k*a431));

  let e321 = a321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn vector_sandwich_even_grade(
  Vector { e1: o1, e2: o2, e3: o3, e4: o4 }: Vector,
  EvenGrade {
    s: a_s,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e1234: a1234,
  }: EvenGrade,
) -> EvenGrade {
  let c =   o1*o1 + o2*o2 + o3*o3;
  let d =   o1*o1 - o2*o2 - o3*o3;
  let e = - o1*o1 + o2*o2 - o3*o3;
  let f = - o1*o1 - o2*o2 + o3*o3;

  let cr = 1./c;

  let m = o4*o1;
  let n = o4*o2;
  let o = o4*o3;
  let j = o1*o2;
  let k = o2*o3;
  let l = o3*o1;

  let s = a_s;

  let e23 = cr*(d*a23 + 2.*(j*a31 + l*a12));
  let e31 = cr*(e*a31 + 2.*(k*a12 + j*a23));
  let e12 = cr*(f*a12 + 2.*(l*a23 + k*a31));

  let e41 = -cr*(d*a41 + 2.*(o*a31 - n*a12 + j*a42 + l*a43));
  let e42 = -cr*(e*a42 + 2.*(m*a12 - o*a23 + k*a43 + j*a41));
  let e43 = -cr*(f*a43 + 2.*(n*a23 - m*a31 + l*a41 + k*a42));

  let e1234 = -a1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn vector_sandwich_odd_grade(
  Vector { e1: o1, e2: o2, e3: o3, e4: o4 }: Vector,
  OddGrade {
    e1: a1, e2: a2, e3: a3, e4: a4,
    e423: a423, e431: a431, e412: a412, e321: a321,
  }: OddGrade,
) -> OddGrade {
  let c =   o1*o1 + o2*o2 + o3*o3;
  let d =   o1*o1 - o2*o2 - o3*o3;
  let e = - o1*o1 + o2*o2 - o3*o3;
  let f = - o1*o1 - o2*o2 + o3*o3;

  let cr = 1./c;

  let m = o4*o1;
  let n = o4*o2;
  let o = o4*o3;
  let j = o1*o2;
  let k = o2*o3;
  let l = o3*o1;

  let e1 = cr*(d*a1 + 2.*(j*a2 + l*a3));
  let e2 = cr*(e*a2 + 2.*(k*a3 + j*a1));
  let e3 = cr*(f*a3 + 2.*(l*a1 + k*a2));
  let e4 = -a4 + 2.*cr*(m*a1 + n*a2 + o*a3);

  let e423 = -cr*(d*a423 + 2.*(m*a321 + j*a431 + l*a412));
  let e431 = -cr*(e*a431 + 2.*(n*a321 + k*a412 + j*a423));
  let e412 = -cr*(f*a412 + 2.*(o*a321 + l*a423 + k*a431));
  let e321 = a321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_sandwich_multivector(
  Bivector {
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
  }: Bivector,
  Multivector {
    s: a_s,
    e1: a1, e2: a2, e3: a3, e4: a4,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e423: a423, e431: a431, e412: a412, e321: a321,
    e1234: a1234,
  }: Multivector,
) -> Multivector {
  let c =   o23*o23 + o31*o31 + o12*o12;
  let d =   o23*o23 - o31*o31 - o12*o12;
  let e = - o23*o23 + o31*o31 - o12*o12;
  let f = - o23*o23 - o31*o31 + o12*o12;

  let cr = 1. / c;

  let m =   o43*o31 + o42*o12;
  let m_c = o43*o31 - o42*o12;

  let n =   o41*o12 + o43*o23;
  let n_c = o41*o12 - o43*o23;

  let o =   o42*o23 + o41*o31;
  let o_c = o42*o23 - o41*o31;

  let p =      o41*o23 + o42*o31 + o43*o12;
  let p_41 =   o41*o23 - o42*o31 - o43*o12;
  let p_42 = - o41*o23 + o42*o31 - o43*o12;
  let p_43 = - o41*o23 - o42*o31 + o43*o12;

  let j = o31*o12;
  let k = o12*o23;
  let l = o23*o31;

  let s = a_s;

  let e1 = cr*(d*a1 + 2.*(l*a2 + k*a3));
  let e2 = cr*(e*a2 + 2.*(j*a3 + l*a1));
  let e3 = cr*(f*a3 + 2.*(k*a1 + j*a2));
  let e4 = a4 + 2.*cr*(m_c*a1 + n_c*a2 + o_c*a3 - p*a321);

  let e23 = cr*(d*a23 + 2.*(l*a31 + k*a12));
  let e31 = cr*(e*a31 + 2.*(j*a12 + l*a23));
  let e12 = cr*(f*a12 + 2.*(k*a23 + j*a31));
  let e41 = cr*(d*a41 +
    2.*(-p*e23 + o*a31 + n*a12 + p_41*a23 + l*a42 + k*a43));
  let e42 = cr*(e*a42 +
    2.*(-p*e31 + m*a12 + o*a23 + p_42*a31 + j*a43 + l*a41));
  let e43 = cr*(f*a43 +
    2.*(-p*e12 + n*a23 + m*a31 + p_43*a12 + k*a41 + j*a42));

  let e423 = cr*(d*a423 + 2.*(-p*e1 + m_c*a321 + l*a431 + k*a412));
  let e431 = cr*(e*a431 + 2.*(-p*e2 + n_c*a321 + j*a412 + l*a423));
  let e412 = cr*(f*a412 + 2.*(-p*e3 + o_c*a321 + k*a423 + j*a431));
  let e321 = a321;

  let e1234 = a1234;

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
fn bivector_sandwich_vector(
  Bivector {
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
  }: Bivector,
  Vector { e1: a1, e2: a2, e3: a3, e4: a4 }: Vector,
) -> OddGrade {
  let c =   o23*o23 + o31*o31 + o12*o12;
  let d =   o23*o23 - o31*o31 - o12*o12;
  let e = - o23*o23 + o31*o31 - o12*o12;
  let f = - o23*o23 - o31*o31 + o12*o12;

  let cr = 1. / c;

  let m = - o42*o12 + o43*o31;
  let n =   o41*o12 - o43*o23;
  let o = - o41*o31 + o42*o23;

  let p = o41*o23 + o42*o31 + o43*o12;

  let j = o31*o12;
  let k = o12*o23;
  let l = o23*o31;

  let e1 = cr*(d*a1 + 2.*(l*a2 + k*a3));
  let e2 = cr*(e*a2 + 2.*(j*a3 + l*a1));
  let e3 = cr*(f*a3 + 2.*(k*a1 + j*a2));
  let e4 = a4 + 2.*cr*(m*a1 + n*a2 + o*a3);

  let e423 = -2.*cr*p*e1;
  let e431 = -2.*cr*p*e2;
  let e412 = -2.*cr*p*e3;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_sandwich_bivector(
  Bivector {
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
  }: Bivector,
  Bivector {
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
  }: Bivector,
) -> Bivector {
  let c =   o23*o23 + o31*o31 + o12*o12;
  let d =   o23*o23 - o31*o31 - o12*o12;
  let e = - o23*o23 + o31*o31 - o12*o12;
  let f = - o23*o23 - o31*o31 + o12*o12;

  let cr = 1. / c;

  let m = o43*o31 + o42*o12;
  let n = o41*o12 + o43*o23;
  let o = o42*o23 + o41*o31;

  let p =      o41*o23 + o42*o31 + o43*o12;
  let p_41 =   o41*o23 - o42*o31 - o43*o12;
  let p_42 = - o41*o23 + o42*o31 - o43*o12;
  let p_43 = - o41*o23 - o42*o31 + o43*o12;

  let j = o31*o12;
  let k = o12*o23;
  let l = o23*o31;

  let e23 = cr*(d*a23 + 2.*(l*a31 + k*a12));
  let e31 = cr*(e*a31 + 2.*(j*a12 + l*a23));
  let e12 = cr*(f*a12 + 2.*(k*a23 + j*a31));
  let e41 = cr*(d*a41
    + 2.*(-p*e23 + o*a31 + n*a12 + p_41*a23 + l*a42 + k*a43));
  let e42 = cr*(e*a42
    + 2.*(-p*e31 + m*a12 + o*a23 + p_42*a31 + j*a43 + l*a41));
  let e43 = cr*(f*a43
    + 2.*(-p*e12 + n*a23 + m*a31 + p_43*a12 + k*a41 + j*a42));

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn bivector_sandwich_trivector(
  Bivector {
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
  }: Bivector,
  Trivector { e423: a423, e431: a431, e412: a412, e321: a321 }: Trivector,
) -> OddGrade {
  let c =   o23*o23 + o31*o31 + o12*o12;
  let d =   o23*o23 - o31*o31 - o12*o12;
  let e = - o23*o23 + o31*o31 - o12*o12;
  let f = - o23*o23 - o31*o31 + o12*o12;

  let cr = 1. / c;

  let m = - o42*o12 + o43*o31;
  let n =   o41*o12 - o43*o23;
  let o = - o41*o31 + o42*o23;

  let p =   o41*o23 + o42*o31 + o43*o12;

  let j =   o31*o12;
  let j_c = o31*o12;
  let k =   o12*o23;
  let k_c = o12*o23;
  let l =   o23*o31;
  let l_c = o23*o31;

  let e4 = -2.*cr*p*a321;

  let e423 = cr*(d*a423 + 2.*(m*a321 + l*a431 + k_c*a412));
  let e431 = cr*(e*a431 + 2.*(n*a321 + j*a412 + l_c*a423));
  let e412 = cr*(f*a412 + 2.*(o*a321 + k*a423 + j_c*a431));
  let e321 = a321;

  OddGrade {
    e4,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_sandwich_evengrade(
  Bivector {
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
  }: Bivector,
  EvenGrade {
    s: a_s,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e1234: a1234,
  }: EvenGrade,
) -> EvenGrade {
  let c =   o23*o23 + o31*o31 + o12*o12;
  let d =   o23*o23 - o31*o31 - o12*o12;
  let e = - o23*o23 + o31*o31 - o12*o12;
  let f = - o23*o23 - o31*o31 + o12*o12;

  let cr = 1. / c;

  let m = o43*o31 + o42*o12;
  let n = o41*o12 + o43*o23;
  let o = o42*o23 + o41*o31;

  let p =      o41*o23 + o42*o31 + o43*o12;
  let p_41 =   o41*o23 - o42*o31 - o43*o12;
  let p_42 = - o41*o23 + o42*o31 - o43*o12;
  let p_43 = - o41*o23 - o42*o31 + o43*o12;

  let j = o31*o12;
  let k = o12*o23;
  let l = o23*o31;

  let s = a_s;

  let e23 = cr*(d*a23 + 2.*(l*a31 + k*a12));
  let e31 = cr*(e*a31 + 2.*(j*a12 + l*a23));
  let e12 = cr*(f*a12 + 2.*(k*a23 + j*a31));
  let e41 = cr*(d*a41
    + 2.*(-p*e23 + o*a31 + n*a12 + p_41*a23 + l*a42 + k*a43));
  let e42 = cr*(e*a42
    + 2.*(-p*e31 + m*a12 + o*a23 + p_42*a31 + j*a43 + l*a41));
  let e43 = cr*(f*a43
    + 2.*(-p*e12 + n*a23 + m*a31 + p_43*a12 + k*a41 + j*a42));

  let e1234 = a1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn bivector_sandwich_oddgrade(
  Bivector {
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
  }: Bivector,
  OddGrade {
    e1: a1, e2: a2, e3: a3, e4: a4,
    e423: a423, e431: a431, e412: a412, e321: a321,
  }: OddGrade,
) -> OddGrade {
  let c =   o23*o23 + o31*o31 + o12*o12;
  let d =   o23*o23 - o31*o31 - o12*o12;
  let e = - o23*o23 + o31*o31 - o12*o12;
  let f = - o23*o23 - o31*o31 + o12*o12;

  let cr = 1. / c;

  let m_c = o43*o31 - o42*o12;
  let n_c = o41*o12 - o43*o23;
  let o_c = o42*o23 - o41*o31;

  let p = o41*o23 + o42*o31 + o43*o12;

  let j = o31*o12;
  let k = o12*o23;
  let l = o23*o31;

  let e1 = cr*(d*a1 + 2.*(l*a2 + k*a3));
  let e2 = cr*(e*a2 + 2.*(j*a3 + l*a1));
  let e3 = cr*(f*a3 + 2.*(k*a1 + j*a2));
  let e4 = a4 + 2.*cr*(m_c*a1 + n_c*a2 + o_c*a3 - p*a321);

  let e423 = cr*(d*a423 + 2.*(-p*e1 + m_c*a321 + l*a431 + k*a412));
  let e431 = cr*(e*a431 + 2.*(-p*e2 + n_c*a321 + j*a412 + l*a423));
  let e412 = cr*(f*a412 + 2.*(-p*e3 + o_c*a321 + k*a423 + j*a431));
  let e321 = a321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_sandwich_multivector(
  Trivector { e423: o423, e431: o431, e412: o412, e321: o321 }: Trivector,
  Multivector {
    s: a_s,
    e1: a1, e2: a2, e3: a3, e4: a4,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e423: a423, e431: a431, e412: a412, e321: a321,
    e1234: a1234,
  }: Multivector,
) -> Multivector {
  let cr = 1./o321;

  let m = 2.*o423*cr;
  let n = 2.*o431*cr;
  let o = 2.*o412*cr;

  let s = a_s;

  let e1 = a1;
  let e2 = a2;
  let e3 = a3;

  let e4 = -m*a1 - n*a2 - o*a3 - a4;

  let e23 = a23;
  let e31 = a31;
  let e12 = a12;

  let e41 = n*a12 - o*a31 - a41;
  let e42 = o*a23 - m*a12 - a42;
  let e43 = m*a31 - n*a23 - a43;

  let e423 = m*a321 - a423;
  let e431 = n*a321 - a431;
  let e412 = o*a321 - a412;

  let e321 = a321;

  let e1234 = -a1234;

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
fn trivector_sandwich_vector(
  Trivector { e423: o423, e431: o431, e412: o412, e321: o321 }: Trivector,
  Vector { e1: a1, e2: a2, e3: a3, e4: a4 }: Vector,
) -> Vector {

  let q = -2./o321;

  let e1 = a1;
  let e2 = a2;
  let e3 = a3;
  let e4 = q*(a1*o423 + a2*o431 + a3*o412) - a4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
fn trivector_sandwich_bivector(
  Trivector { e423: o423, e431: o431, e412: o412, e321: o321 }: Trivector,
  Bivector {
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
  }: Bivector,
) -> Bivector {

  let q = 2./o321;

  let e41 = q*(o431*a12 - o412*a31) - a41;
  let e42 = q*(o412*a23 - o423*a12) - a42;
  let e43 = q*(o423*a31 - o431*a23) - a43;
  let e23 = a23;
  let e31 = a31;
  let e12 = a12;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
fn trivector_sandwich_trivector(
  Trivector { e423: o423, e431: o431, e412: o412, e321: o321 }: Trivector,
  Trivector { e423: a423, e431: a431, e412: a412, e321: a321 }: Trivector,
) -> Trivector {

  let q = 2.*a321/o321;

  let e423 = q*o423 - a423;
  let e431 = q*o431 - a431;
  let e412 = q*o412 - a412;
  let e321 = a321;

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
fn trivector_sandwich_even_grade(
  Trivector { e423: o423, e431: o431, e412: o412, e321: o321 }: Trivector,
  EvenGrade {
    s: a_s,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e1234: a1234,
  }: EvenGrade,
) -> EvenGrade {
  let cr = 1./o321;

  let m = 2.*o423*cr;
  let n = 2.*o431*cr;
  let o = 2.*o412*cr;

  let s = a_s;

  let e23 = a23;
  let e31 = a31;
  let e12 = a12;

  let e41 = n*a12 - o*a31 - a41;
  let e42 = o*a23 - m*a12 - a42;
  let e43 = m*a31 - n*a23 - a43;

  let e1234 = -a1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn trivector_sandwich_odd_grade(
  Trivector { e423: o423, e431: o431, e412: o412, e321: o321 }: Trivector,
  OddGrade {
    e1: a1, e2: a2, e3: a3, e4: a4,
    e423: a423, e431: a431, e412: a412, e321: a321,
  }: OddGrade,
) -> OddGrade {
  let cr = 1./o321;

  let m = 2.*o423*cr;
  let n = 2.*o431*cr;
  let o = 2.*o412*cr;

  let e1 = a1;
  let e2 = a2;
  let e3 = a3;
  let e4 = -m*a1 - n*a2 - o*a3 - a4;

  let e423 = m*a321 - a423;
  let e431 = n*a321 - a431;
  let e412 = o*a321 - a412;
  let e321 = a321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_sandwich_multivector(
  DualNumber { s: o_s, e1234: o1234 }: DualNumber,
  Multivector {
    s: a_s,
    e1: a1, e2: a2, e3: a3, e4: a4,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e423: a423, e431: a431, e412: a412, e321: a321,
    e1234: a1234,
  }: Multivector,
) -> Multivector {
  let p = -2.*o1234/o_s;

  let s = a_s;

  let e1 = a1;
  let e2 = a2;
  let e3 = a3;
  let e4 = a4 + p*a321;

  let e23 = a23;
  let e31 = a31;
  let e12 = a12;
  let e41 = a41;
  let e42 = a42;
  let e43 = a43;

  let e423 = a423 + p*a1;
  let e431 = a431 + p*a2;
  let e412 = a412 + p*a3;
  let e321 = a321;

  let e1234 = a1234;

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
fn dual_number_sandwich_vector(
  DualNumber { s: o_s, e1234: o1234 }: DualNumber,
  Vector { e1: a1, e2: a2, e3: a3, e4: a4 }: Vector,
) -> OddGrade {
  let x = -2.*o1234/o_s;

  let e1 = a1;
  let e2 = a2;
  let e3 = a3;
  let e4 = a4;

  let e423 = x*a1;
  let e431 = x*a2;
  let e412 = x*a3;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_sandwich_trivector(
  DualNumber { s: o_s, e1234: o1234 }: DualNumber,
  Trivector { e423: a423, e431: a431, e412: a412, e321: a321 }: Trivector,
) -> OddGrade {
  let x = -2.*o1234/o_s;

  let e4 = x*a321;

  let e423 = a423;
  let e431 = a431;
  let e412 = a412;
  let e321 = a321;

  OddGrade {
    e4,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn dual_number_sandwich_odd_grade(
  DualNumber { s: o_s, e1234: o1234 }: DualNumber,
  OddGrade {
    e1: a1, e2: a2, e3: a3, e4: a4,
    e423: a423, e431: a431, e412: a412, e321: a321,
  }: OddGrade,
) -> OddGrade {
  let x = -2.*o1234/o_s;

  let e1 = a1;
  let e2 = a2;
  let e3 = a3;
  let e4 = a4 + x*a321;

  let e423 = a423 + x*a1;
  let e431 = a431 + x*a2;
  let e412 = a412 + x*a3;
  let e321 = a321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn evengrade_sandwich_multivector(
  EvenGrade {
    s: o_s,
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
    e1234: o1234,
  }: EvenGrade,
  Multivector {
    s: a_s,
    e1: a1, e2: a2, e3: a3, e4: a4,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e423: a423, e431: a431, e412: a412, e321: a321,
    e1234: a1234,
  }: Multivector,
) -> Multivector {
  let c =   o23*o23 + o31*o31 + o12*o12 + o_s*o_s;
  let d =   o23*o23 - o31*o31 - o12*o12 + o_s*o_s;
  let e = - o23*o23 + o31*o31 - o12*o12 + o_s*o_s;
  let f = - o23*o23 - o31*o31 + o12*o12 + o_s*o_s;

  let cr = 1. / c;

  let m =       o41*o_s - o42*o12 + o43*o31 - o1234*o23;
  let m_42 =    o41*o_s + o42*o12 + o43*o31 + o1234*o23;
  let m_43 =  - o41*o_s + o42*o12 + o43*o31 - o1234*o23;
  let m_423 = - o41*o_s - o42*o12 + o43*o31 + o1234*o23;

  let n =       o41*o12 + o42*o_s - o43*o23 - o1234*o31;
  let n_41 =    o41*o12 - o42*o_s + o43*o23 - o1234*o31;
  let n_43 =    o41*o12 + o42*o_s + o43*o23 + o1234*o31;
  let n_431 =   o41*o12 - o42*o_s - o43*o23 + o1234*o31;

  let o = -     o41*o31 + o42*o23 + o43*o_s - o1234*o12;
  let o_41 =    o41*o31 + o42*o23 + o43*o_s + o1234*o12;
  let o_42 =    o41*o31 + o42*o23 - o43*o_s - o1234*o12;
  let o_412 = - o41*o31 + o42*o23 - o43*o_s + o1234*o12;

  let p =       o41*o23 + o42*o31 + o43*o12 + o1234*o_s; // geometric residual
  let p_41 =    o41*o23 - o42*o31 - o43*o12 + o1234*o_s;
  let p_42 =  - o41*o23 + o42*o31 - o43*o12 + o1234*o_s;
  let p_43 =  - o41*o23 - o42*o31 + o43*o12 + o1234*o_s;

  let j =   o31*o12 + o23*o_s;
  let j_c = o31*o12 - o23*o_s;
  let k =   o12*o23 + o31*o_s;
  let k_c = o12*o23 - o31*o_s;
  let l =   o23*o31 + o12*o_s;
  let l_c = o23*o31 - o12*o_s;

  let s = a_s;

  let e1 = cr*(d*a1 + 2.*(l*a2 + k_c*a3));
  let e2 = cr*(e*a2 + 2.*(j*a3 + l_c*a1));
  let e3 = cr*(f*a3 + 2.*(k*a1 + j_c*a2));
  let e4 = a4 + 2.*cr*(m*a1 + n*a2 + o*a3 - p*a321);

  let e23 = cr*(d*a23 + 2.*(l*a31 + k_c*a12));
  let e31 = cr*(e*a31 + 2.*(j*a12 + l_c*a23));
  let e12 = cr*(f*a12 + 2.*(k*a23 + j_c*a31));
  let e41 = cr*(d*a41
    + 2.*(-p*e23 + o_41*a31 + n_41*a12 + p_41*a23 + l*a42 + k_c*a43));
  let e42 = cr*(e*a42
    + 2.*(-p*e31 + m_42*a12 + o_42*a23 + p_42*a31 + j*a43 + l_c*a41));
  let e43 = cr*(f*a43
    + 2.*(-p*e12 + n_43*a23 + m_43*a31 + p_43*a12 + k*a41 + j_c*a42));

  let e423 = cr*(d*a423 + 2.*(-p*e1 + m_423*a321 + l*a431 + k_c*a412));
  let e431 = cr*(e*a431 + 2.*(-p*e2 + n_431*a321 + j*a412 + l_c*a423));
  let e412 = cr*(f*a412 + 2.*(-p*e3 + o_412*a321 + k*a423 + j_c*a431));
  let e321 = a321;

  let e1234 = a1234;

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
fn evengrade_sandwich_vector(
  EvenGrade {
    s: o_s,
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
    e1234: o1234,
  }: EvenGrade,
  Vector { e1: a1, e2: a2, e3: a3, e4: a4 }: Vector,
) -> OddGrade {
  let c =   o23*o23 + o31*o31 + o12*o12 + o_s*o_s;
  let d =   o23*o23 - o31*o31 - o12*o12 + o_s*o_s;
  let e = - o23*o23 + o31*o31 - o12*o12 + o_s*o_s;
  let f = - o23*o23 - o31*o31 + o12*o12 + o_s*o_s;

  let cr = 1. / c;

  let m =   o41*o_s - o42*o12 + o43*o31 - o1234*o23;
  let n =   o41*o12 + o42*o_s - o43*o23 - o1234*o31;
  let o = - o41*o31 + o42*o23 + o43*o_s - o1234*o12;
  let p =   o41*o23 + o42*o31 + o43*o12 + o1234*o_s;

  let j =   o31*o12 + o23*o_s;
  let j_c = o31*o12 - o23*o_s;
  let k =   o12*o23 + o31*o_s;
  let k_c = o12*o23 - o31*o_s;
  let l =   o23*o31 + o12*o_s;
  let l_c = o23*o31 - o12*o_s;

  let e1 = cr*(d*a1 + 2.*(l*a2 + k_c*a3));
  let e2 = cr*(e*a2 + 2.*(j*a3 + l_c*a1));
  let e3 = cr*(f*a3 + 2.*(k*a1 + j_c*a2));
  let e4 = a4 + 2.*cr*(m*a1 + n*a2 + o*a3);

  let z = -2.*p*cr;

  let e423 = z*e1;
  let e431 = z*e2;
  let e412 = z*e3;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn evengrade_sandwich_bivector(
  EvenGrade {
    s: o_s,
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
    e1234: o1234,
  }: EvenGrade,
  Bivector {
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
  }: Bivector,
) -> Bivector {
  let c =   o23*o23 + o31*o31 + o12*o12 + o_s*o_s;
  let d =   o23*o23 - o31*o31 - o12*o12 + o_s*o_s;
  let e = - o23*o23 + o31*o31 - o12*o12 + o_s*o_s;
  let f = - o23*o23 - o31*o31 + o12*o12 + o_s*o_s;

  let cr = 1. / c;

  let m_42 =   o41*o_s + o42*o12 + o43*o31 + o1234*o23;
  let m_43 = - o41*o_s + o42*o12 + o43*o31 - o1234*o23;

  let n_41 =   o41*o12 - o42*o_s + o43*o23 - o1234*o31;
  let n_43 =   o41*o12 + o42*o_s + o43*o23 + o1234*o31;

  let o_41 =   o41*o31 + o42*o23 + o43*o_s + o1234*o12;
  let o_42 =   o41*o31 + o42*o23 - o43*o_s - o1234*o12;

  let p =      o41*o23 + o42*o31 + o43*o12 + o1234*o_s;
  let p_41 =   o41*o23 - o42*o31 - o43*o12 + o1234*o_s;
  let p_42 = - o41*o23 + o42*o31 - o43*o12 + o1234*o_s;
  let p_43 = - o41*o23 - o42*o31 + o43*o12 + o1234*o_s;

  let j =   o31*o12 + o23*o_s;
  let j_c = o31*o12 - o23*o_s;
  let k =   o12*o23 + o31*o_s;
  let k_c = o12*o23 - o31*o_s;
  let l =   o23*o31 + o12*o_s;
  let l_c = o23*o31 - o12*o_s;

  let e23 = cr*(d*a23 + 2.*(l*a31 + k_c*a12));
  let e31 = cr*(e*a31 + 2.*(j*a12 + l_c*a23));
  let e12 = cr*(f*a12 + 2.*(k*a23 + j_c*a31));
  let e41 = cr*(d*a41
    + 2.*(-p*e23 + o_41*a31 + n_41*a12 + p_41*a23 + l*a42 + k_c*a43));
  let e42 = cr*(e*a42
    + 2.*(-p*e31 + m_42*a12 + o_42*a23 + p_42*a31 + j*a43 + l_c*a41));
  let e43 = cr*(f*a43
    + 2.*(-p*e12 + n_43*a23 + m_43*a31 + p_43*a12 + k*a41 + j_c*a42));

  Bivector {
    e41, e42, e43, e23, e31, e12,
  }
}

#[rustfmt::skip]
#[inline]
fn evengrade_sandwich_trivector(
  EvenGrade {
    s: o_s,
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
    e1234: o1234,
  }: EvenGrade,
  Trivector { e423: a423, e431: a431, e412: a412, e321: a321 }: Trivector,
) -> OddGrade {
  let c =   o23*o23 + o31*o31 + o12*o12 + o_s*o_s;
  let d =   o23*o23 - o31*o31 - o12*o12 + o_s*o_s;
  let e = - o23*o23 + o31*o31 - o12*o12 + o_s*o_s;
  let f = - o23*o23 - o31*o31 + o12*o12 + o_s*o_s;

  let cr = 1. / c;

  let m = - o41*o_s - o42*o12 + o43*o31 + o1234*o23;
  let n =   o41*o12 - o42*o_s - o43*o23 + o1234*o31;
  let o = - o41*o31 + o42*o23 - o43*o_s + o1234*o12;
  let p =   o41*o23 + o42*o31 + o43*o12 + o1234*o_s;

  let j =   o31*o12 + o23*o_s;
  let j_c = o31*o12 - o23*o_s;
  let k =   o12*o23 + o31*o_s;
  let k_c = o12*o23 - o31*o_s;
  let l =   o23*o31 + o12*o_s;
  let l_c = o23*o31 - o12*o_s;

  let e4 = -2.*cr*p*a321;

  let e423 = cr*(d*a423 + 2.*(m*a321 + l*a431 + k_c*a412));
  let e431 = cr*(e*a431 + 2.*(n*a321 + j*a412 + l_c*a423));
  let e412 = cr*(f*a412 + 2.*(o*a321 + k*a423 + j_c*a431));
  let e321 = a321;

  OddGrade {
    e4,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn evengrade_sandwich_evengrade(
  EvenGrade {
    s: o_s,
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
    e1234: o1234,
  }: EvenGrade,
  EvenGrade {
    s: a_s,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e1234: a1234,
  }: EvenGrade,
) -> EvenGrade {
  let c =   o23*o23 + o31*o31 + o12*o12 + o_s*o_s;
  let d =   o23*o23 - o31*o31 - o12*o12 + o_s*o_s;
  let e = - o23*o23 + o31*o31 - o12*o12 + o_s*o_s;
  let f = - o23*o23 - o31*o31 + o12*o12 + o_s*o_s;

  let cr = 1. / c;

  let m_42 =   o41*o_s + o42*o12 + o43*o31 + o1234*o23;
  let m_43 = - o41*o_s + o42*o12 + o43*o31 - o1234*o23;

  let n_41 =   o41*o12 - o42*o_s + o43*o23 - o1234*o31;
  let n_43 =   o41*o12 + o42*o_s + o43*o23 + o1234*o31;

  let o_41 =   o41*o31 + o42*o23 + o43*o_s + o1234*o12;
  let o_42 =   o41*o31 + o42*o23 - o43*o_s - o1234*o12;

  let p =      o41*o23 + o42*o31 + o43*o12 + o1234*o_s;
  let p_41 =   o41*o23 - o42*o31 - o43*o12 + o1234*o_s;
  let p_42 = - o41*o23 + o42*o31 - o43*o12 + o1234*o_s;
  let p_43 = - o41*o23 - o42*o31 + o43*o12 + o1234*o_s;

  let j =   o31*o12 + o23*o_s;
  let j_c = o31*o12 - o23*o_s;
  let k =   o12*o23 + o31*o_s;
  let k_c = o12*o23 - o31*o_s;
  let l =   o23*o31 + o12*o_s;
  let l_c = o23*o31 - o12*o_s;

  let s = a_s;

  let e23 = cr*(d*a23 + 2.*(l*a31 + k_c*a12));
  let e31 = cr*(e*a31 + 2.*(j*a12 + l_c*a23));
  let e12 = cr*(f*a12 + 2.*(k*a23 + j_c*a31));

  let e41 = cr*(d*a41
    + 2.*(-p*e23 + o_41*a31 + n_41*a12 + p_41*a23 + l*a42 + k_c*a43));
  let e42 = cr*(e*a42
    + 2.*(-p*e31 + m_42*a12 + o_42*a23 + p_42*a31 + j*a43 + l_c*a41));
  let e43 = cr*(f*a43
    + 2.*(-p*e12 + n_43*a23 + m_43*a31 + p_43*a12 + k*a41 + j_c*a42));

  let e1234 = a1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn evengrade_sandwich_oddgrade(
  EvenGrade {
    s: o_s,
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
    e1234: o1234,
  }: EvenGrade,
  OddGrade {
    e1: a1, e2: a2, e3: a3, e4: a4,
    e423: a423, e431: a431, e412: a412, e321: a321,
  }: OddGrade,
) -> OddGrade {
  let c =   o23*o23 + o31*o31 + o12*o12 + o_s*o_s;
  let d =   o23*o23 - o31*o31 - o12*o12 + o_s*o_s;
  let e = - o23*o23 + o31*o31 - o12*o12 + o_s*o_s;
  let f = - o23*o23 - o31*o31 + o12*o12 + o_s*o_s;

  let cr = 1. / c;

  let m =       o41*o_s - o42*o12 + o43*o31 - o1234*o23;
  let m_423 = - o41*o_s - o42*o12 + o43*o31 + o1234*o23;

  let n =       o41*o12 + o42*o_s - o43*o23 - o1234*o31;
  let n_431 =   o41*o12 - o42*o_s - o43*o23 + o1234*o31;

  let o = -     o41*o31 + o42*o23 + o43*o_s - o1234*o12;
  let o_412 = - o41*o31 + o42*o23 - o43*o_s + o1234*o12;

  let p =       o41*o23 + o42*o31 + o43*o12 + o1234*o_s;

  let j =   o31*o12 + o23*o_s;
  let j_c = o31*o12 - o23*o_s;
  let k =   o12*o23 + o31*o_s;
  let k_c = o12*o23 - o31*o_s;
  let l =   o23*o31 + o12*o_s;
  let l_c = o23*o31 - o12*o_s;

  let e1 = cr*(d*a1 + 2.*(l*a2 + k_c*a3));
  let e2 = cr*(e*a2 + 2.*(j*a3 + l_c*a1));
  let e3 = cr*(f*a3 + 2.*(k*a1 + j_c*a2));
  let e4 = a4 + 2.*cr*(m*a1 + n*a2 + o*a3 - p*a321);

  let e423 = cr*(d*a423 + 2.*(-p*e1 + m_423*a321 + l*a431 + k_c*a412));
  let e431 = cr*(e*a431 + 2.*(-p*e2 + n_431*a321 + j*a412 + l_c*a423));
  let e412 = cr*(f*a412 + 2.*(-p*e3 + o_412*a321 + k*a423 + j_c*a431));
  let e321 = a321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_sandwich_multivector(
  OddGrade {
    e1: o1, e2: o2, e3: o3, e4: o4,
    e423: o423, e431: o431, e412: o412, e321: o321,
  }: OddGrade,
  Multivector {
    s: a_s,
    e1: a1, e2: a2, e3: a3, e4: a4,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e423: a423, e431: a431, e412: a412, e321: a321,
    e1234: a1234,
  }: Multivector,
) -> Multivector {
  let c =   o1*o1 + o2*o2 + o3*o3 + o321*o321;
  let d =   o1*o1 - o2*o2 - o3*o3 + o321*o321;
  let e = - o1*o1 + o2*o2 - o3*o3 + o321*o321;
  let f = - o1*o1 - o2*o2 + o3*o3 + o321*o321;

  let cr = 1./c;

  let m_4 =   - o423*o321 + o431*o3 - o412*o2 + o4*o1;
  let m_42 =  - o423*o321 - o431*o3 - o412*o2 - o4*o1;
  let m_43 =    o423*o321 - o431*o3 - o412*o2 + o4*o1;
  let m_423 =   o423*o321 + o431*o3 - o412*o2 - o4*o1;

  let n_4 =   - o423*o3 - o431*o321 + o412*o1 + o4*o2;
  let n_41 =  - o423*o3 + o431*o321 - o412*o1 + o4*o2;
  let n_43 =  - o423*o3 - o431*o321 - o412*o1 - o4*o2;
  let n_431 = - o423*o3 + o431*o321 + o412*o1 - o4*o2;

  let o_4 =     o423*o2 - o431*o1 - o412*o321 + o4*o3;
  let o_41 =  - o423*o2 - o431*o1 - o412*o321 - o4*o3;
  let o_42 =  - o423*o2 - o431*o1 + o412*o321 + o4*o3;
  let o_412 =   o423*o2 - o431*o1 + o412*o321 - o4*o3;

  let p =       o423*o1 + o431*o2 + o412*o3 + o321*o4;
  let p_41 =  - o423*o1 + o431*o2 + o412*o3 - o321*o4;
  let p_42 =    o423*o1 - o431*o2 + o412*o3 - o321*o4;
  let p_43 =    o423*o1 + o431*o2 - o412*o3 - o321*o4;

  let j =   o1*o2 + o3*o321;
  let j_c = o1*o2 - o3*o321;
  let k =   o2*o3 + o1*o321;
  let k_c = o2*o3 - o1*o321;
  let l =   o3*o1 + o2*o321;
  let l_c = o3*o1 - o2*o321;

  let s = a_s;

  let e1 = cr*(d*a1 + 2.*(j*a2 + l_c*a3));
  let e2 = cr*(e*a2 + 2.*(k*a3 + j_c*a1));
  let e3 = cr*(f*a3 + 2.*(l*a1 + k_c*a2));

  let e4 = -a4 + 2.*cr*(m_4*a1 + n_4*a2 + o_4*a3 + p*a321);

  let e23 = cr*(d*a23 + 2.*(j*a31 + l_c*a12));
  let e31 = cr*(e*a31 + 2.*(k*a12 + j_c*a23));
  let e12 = cr*(f*a12 + 2.*(l*a23 + k_c*a31));

  let e41 = cr*(-d*a41
    + 2.*(p*e23 + p_41*a23 + o_41*a31 + n_41*a12 - j*a42 - l_c*a43));
  let e42 = cr*(-e*a42
    + 2.*(p*e31 + o_42*a23 + p_42*a31 + m_42*a12 - k*a43 - j_c*a41));
  let e43 = cr*(-f*a43
    + 2.*(p*e12 + n_43*a23 + m_43*a31 + p_43*a12 - l*a41 - k_c*a42));

  let e423 = cr*(-d*a423
    + 2.*(p*e1 + m_423*a321 - j*a431 - l_c*a412));
  let e431 = cr*(-e*a431
    + 2.*(p*e2 + n_431*a321 - k*a412 - j_c*a423));
  let e412 = cr*(-f*a412
    + 2.*(p*e3 + o_412*a321 - l*a423 - k_c*a431));

  let e321 = a321;

  let e1234 = -a1234;

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
fn odd_grade_sandwich_vector(
  OddGrade {
    e1: o1, e2: o2, e3: o3, e4: o4,
    e423: o423, e431: o431, e412: o412, e321: o321,
  }: OddGrade,
  Vector { e1: a1, e2: a2, e3: a3, e4: a4 }: Vector,
) -> OddGrade {
  let c =   o1*o1 + o2*o2 + o3*o3 + o321*o321;
  let d =   o1*o1 - o2*o2 - o3*o3 + o321*o321;
  let e = - o1*o1 + o2*o2 - o3*o3 + o321*o321;
  let f = - o1*o1 - o2*o2 + o3*o3 + o321*o321;

  let cr = 1./c;

  let m_4 = - o423*o321 + o431*o3 - o412*o2 + o4*o1;
  let n_4 = - o423*o3 - o431*o321 + o412*o1 + o4*o2;
  let o_4 =   o423*o2 - o431*o1 - o412*o321 + o4*o3;
  let p =     o423*o1 + o431*o2 + o412*o3 + o321*o4;

  let j =   o1*o2 + o3*o321;
  let j_c = o1*o2 - o3*o321;
  let k =   o2*o3 + o1*o321;
  let k_c = o2*o3 - o1*o321;
  let l =   o3*o1 + o2*o321;
  let l_c = o3*o1 - o2*o321;

  let e1 = cr*(d*a1 + 2.*(j*a2 + l_c*a3));
  let e2 = cr*(e*a2 + 2.*(k*a3 + j_c*a1));
  let e3 = cr*(f*a3 + 2.*(l*a1 + k_c*a2));
  let e4 = -a4 + 2.*cr*(m_4*a1 + n_4*a2 + o_4*a3);

  let e423 = 2.*p*cr*e1;
  let e431 = 2.*p*cr*e2;
  let e412 = 2.*p*cr*e3;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_sandwich_bivector(
  OddGrade {
    e1: o1, e2: o2, e3: o3, e4: o4,
    e423: o423, e431: o431, e412: o412, e321: o321,
  }: OddGrade,
  Bivector {
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
  }: Bivector,
) -> Bivector {
  let c =   o1*o1 + o2*o2 + o3*o3 + o321*o321;
  let d =   o1*o1 - o2*o2 - o3*o3 + o321*o321;
  let e = - o1*o1 + o2*o2 - o3*o3 + o321*o321;
  let f = - o1*o1 - o2*o2 + o3*o3 + o321*o321;

  let cr = 1./c;

  let m_42 = - o423*o321 - o431*o3 - o412*o2 - o4*o1;
  let m_43 =   o423*o321 - o431*o3 - o412*o2 + o4*o1;

  let n_41 = - o423*o3 + o431*o321 - o412*o1 + o4*o2;
  let n_43 = - o423*o3 - o431*o321 - o412*o1 - o4*o2;

  let o_41 = - o423*o2 - o431*o1 - o412*o321 - o4*o3;
  let o_42 = - o423*o2 - o431*o1 + o412*o321 + o4*o3;

  let p =      o423*o1 + o431*o2 + o412*o3 + o321*o4;
  let p_41 = - o423*o1 + o431*o2 + o412*o3 - o321*o4;
  let p_42 =   o423*o1 - o431*o2 + o412*o3 - o321*o4;
  let p_43 =   o423*o1 + o431*o2 - o412*o3 - o321*o4;

  let j =   o1*o2 + o3*o321;
  let j_c = o1*o2 - o3*o321;
  let k =   o2*o3 + o1*o321;
  let k_c = o2*o3 - o1*o321;
  let l =   o3*o1 + o2*o321;
  let l_c = o3*o1 - o2*o321;

  let e23 = cr*(d*a23 + 2.*(j*a31 + l_c*a12));
  let e31 = cr*(e*a31 + 2.*(k*a12 + j_c*a23));
  let e12 = cr*(f*a12 + 2.*(l*a23 + k_c*a31));

  let e41 = cr*(-d*a41
    + 2.*(p*e23 + p_41*a23 + o_41*a31 + n_41*a12 - j*a42 - l_c*a43));
  let e42 = cr*(-e*a42
    + 2.*(p*e31 + o_42*a23 + p_42*a31 + m_42*a12 - k*a43 - j_c*a41));
  let e43 = cr*(-f*a43
    + 2.*(p*e12 + n_43*a23 + m_43*a31 + p_43*a12 - l*a41 - k_c*a42));

  Bivector {
    e41, e42, e43, e23, e31, e12,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_sandwich_trivector(
  OddGrade {
    e1: o1, e2: o2, e3: o3, e4: o4,
    e423: o423, e431: o431, e412: o412, e321: o321,
  }: OddGrade,
  Trivector { e423: a423, e431: a431, e412: a412, e321: a321 }: Trivector,
) -> OddGrade {
  let c =   o1*o1 + o2*o2 + o3*o3 + o321*o321;
  let d =   o1*o1 - o2*o2 - o3*o3 + o321*o321;
  let e = - o1*o1 + o2*o2 - o3*o3 + o321*o321;
  let f = - o1*o1 - o2*o2 + o3*o3 + o321*o321;

  let cr = 1./c;

  let m_423 =   o423*o321 + o431*o3 - o412*o2 - o4*o1;
  let n_431 = - o423*o3 + o431*o321 + o412*o1 - o4*o2;
  let o_412 =   o423*o2 - o431*o1 + o412*o321 - o4*o3;
  let p =       o423*o1 + o431*o2 + o412*o3 + o321*o4;

  let j =   o1*o2 + o3*o321;
  let j_c = o1*o2 - o3*o321;
  let k =   o2*o3 + o1*o321;
  let k_c = o2*o3 - o1*o321;
  let l =   o3*o1 + o2*o321;
  let l_c = o3*o1 - o2*o321;

  let e4 = 2.*cr*p*a321;

  let e423 = cr*(-d*a423 + 2.*(m_423*a321 - j*a431 - l_c*a412));
  let e431 = cr*(-e*a431 + 2.*(n_431*a321 - k*a412 - j_c*a423));
  let e412 = cr*(-f*a412 + 2.*(o_412*a321 - l*a423 - k_c*a431));

  let e321 = a321;

  OddGrade {
    e4,
    e423, e431, e412, e321,
    ..zero()
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_sandwich_even_grade(
  OddGrade {
    e1: o1, e2: o2, e3: o3, e4: o4,
    e423: o423, e431: o431, e412: o412, e321: o321,
  }: OddGrade,
  EvenGrade {
    s: a_s,
    e41: a41, e42: a42, e43: a43, e23: a23, e31: a31, e12: a12,
    e1234: a1234,
  }: EvenGrade,
) -> EvenGrade {
  let c =   o1*o1 + o2*o2 + o3*o3 + o321*o321;
  let d =   o1*o1 - o2*o2 - o3*o3 + o321*o321;
  let e = - o1*o1 + o2*o2 - o3*o3 + o321*o321;
  let f = - o1*o1 - o2*o2 + o3*o3 + o321*o321;

  let cr = 1./c;

  let m_42 = - o423*o321 - o431*o3 - o412*o2 - o4*o1;
  let m_43 =   o423*o321 - o431*o3 - o412*o2 + o4*o1;

  let n_41 = - o423*o3 + o431*o321 - o412*o1 + o4*o2;
  let n_43 = - o423*o3 - o431*o321 - o412*o1 - o4*o2;

  let o_41 = - o423*o2 - o431*o1 - o412*o321 - o4*o3;
  let o_42 = - o423*o2 - o431*o1 + o412*o321 + o4*o3;

  let p =      o423*o1 + o431*o2 + o412*o3 + o321*o4;
  let p_41 = - o423*o1 + o431*o2 + o412*o3 - o321*o4;
  let p_42 =   o423*o1 - o431*o2 + o412*o3 - o321*o4;
  let p_43 =   o423*o1 + o431*o2 - o412*o3 - o321*o4;

  let j =   o1*o2 + o3*o321;
  let j_c = o1*o2 - o3*o321;
  let k =   o2*o3 + o1*o321;
  let k_c = o2*o3 - o1*o321;
  let l =   o3*o1 + o2*o321;
  let l_c = o3*o1 - o2*o321;

  let s = a_s;

  let e23 = cr*(d*a23 + 2.*(j*a31 + l_c*a12));
  let e31 = cr*(e*a31 + 2.*(k*a12 + j_c*a23));
  let e12 = cr*(f*a12 + 2.*(l*a23 + k_c*a31));

  let e41 = cr*(-d*a41
    + 2.*(p*e23 + p_41*a23 + o_41*a31 + n_41*a12 - j*a42 - l_c*a43));
  let e42 = cr*(-e*a42
    + 2.*(p*e31 + o_42*a23 + p_42*a31 + m_42*a12 - k*a43 - j_c*a41));
  let e43 = cr*(-f*a43
    + 2.*(p*e12 + n_43*a23 + m_43*a31 + p_43*a12 - l*a41 - k_c*a42));

  let e1234 = -a1234;

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_sandwich_odd_grade(
  OddGrade {
    e1: o1, e2: o2, e3: o3, e4: o4,
    e423: o423, e431: o431, e412: o412, e321: o321,
  }: OddGrade,
  OddGrade {
    e1: a1, e2: a2, e3: a3, e4: a4,
    e423: a423, e431: a431, e412: a412, e321: a321,
  }: OddGrade,
) -> OddGrade {
  let c =   o1*o1 + o2*o2 + o3*o3 + o321*o321;
  let d =   o1*o1 - o2*o2 - o3*o3 + o321*o321;
  let e = - o1*o1 + o2*o2 - o3*o3 + o321*o321;
  let f = - o1*o1 - o2*o2 + o3*o3 + o321*o321;

  let cr = 1./c;

  let m_4 =   - o423*o321 + o431*o3 - o412*o2 + o4*o1;
  let m_423 =   o423*o321 + o431*o3 - o412*o2 - o4*o1;

  let n_4 =   - o423*o3 - o431*o321 + o412*o1 + o4*o2;
  let n_431 = - o423*o3 + o431*o321 + o412*o1 - o4*o2;

  let o_4 =     o423*o2 - o431*o1 - o412*o321 + o4*o3;
  let o_412 =   o423*o2 - o431*o1 + o412*o321 - o4*o3;

  let p =       o423*o1 + o431*o2 + o412*o3 + o321*o4;

  let j =   o1*o2 + o3*o321;
  let j_c = o1*o2 - o3*o321;
  let k =   o2*o3 + o1*o321;
  let k_c = o2*o3 - o1*o321;
  let l =   o3*o1 + o2*o321;
  let l_c = o3*o1 - o2*o321;

  let e1 = cr*(d*a1 + 2.*(j*a2 + l_c*a3));
  let e2 = cr*(e*a2 + 2.*(k*a3 + j_c*a1));
  let e3 = cr*(f*a3 + 2.*(l*a1 + k_c*a2));

  let e4 = -a4 + 2.*cr*(m_4*a1 + n_4*a2 + o_4*a3 + p*a321);

  let e423 = cr*(-d*a423 + 2.*(m_423*a321 - j*a431 - l_c*a412 + p*e1));
  let e431 = cr*(-e*a431 + 2.*(n_431*a321 - k*a412 - j_c*a423 + p*e2));
  let e412 = cr*(-f*a412 + 2.*(o_412*a321 - l*a423 - k_c*a431 + p*e3));

  let e321 = a321;

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[cfg(test)]
mod tests {
  use {
    crate::{
      algebra::{operators::*, values::*},
      helpers::def_for_each,
      test_values::*,
    },
    ::approx::assert_ulps_eq,
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
        assert_ulps_eq!(
          dbg!(geometric_product(
            geometric_product(multivector, variant),
            inverse(multivector)
          )),
          dbg!(multivector.sandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_product(
            geometric_product(scalar, variant),
            inverse(scalar)
          )),
          dbg!(scalar.sandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_product(
            geometric_product(vector, variant),
            inverse(vector)
          )),
          dbg!(vector.sandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_product(
            geometric_product(bivector, variant),
            inverse(bivector)
          )),
          dbg!(bivector.sandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_product(
            geometric_product(trivector, variant),
            inverse(trivector)
          )),
          dbg!(trivector.sandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert!(dbg!(antiscalar.sandwich(variant)).is_nan());
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_product(
            geometric_product(dual_number, variant),
            inverse(dual_number)
          )),
          dbg!(dual_number.sandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_product(
            geometric_product(odd_grade, variant),
            inverse(odd_grade)
          )),
          dbg!(odd_grade.sandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_product(
            geometric_product(even_grade, variant),
            inverse(even_grade)
          )),
          dbg!(even_grade.sandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
    }
  }
}
