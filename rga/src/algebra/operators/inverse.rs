use crate::{
  algebra::values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  F,
};

/// u⁻¹
///
/// Compute the inverse with respect to the
/// [`GeometricProduct`](crate::GeometricProduct) if it exists. If it is
/// non-invertible then the returned value will contain a NaN.
///
/// If the inverse does not exist, then the result will be
/// [`IsNan`](crate::IsNan).
///
/// Note: Inverse of zero is defined to be NaN even for [`struct@Scalar`]
/// values, which differs to division in IEEE754.
#[inline]
pub fn inverse<M>(u: M) -> M
where
  M: Inverse,
{
  u.inverse()
}

/// u⁻¹
///
/// Compute the inverse with respect to the
/// [`GeometricProduct`](crate::GeometricProduct) if it exists. If it is
/// non-invertible then the returned value will contain a NaN.
///
/// If the inverse does not exist, then the result will be
/// [`IsNan`](crate::IsNan).
///
/// Note: Inverse of zero is defined to be NaN even for [`struct@Scalar`]
/// values, which differs to division in IEEE754.
pub trait Inverse {
  /// u⁻¹
  ///
  /// Compute the inverse with respect to the
  /// [`GeometricProduct`](crate::GeometricProduct) if it exists. If it is
  /// non-invertible then the returned value will contain a NaN.
  ///
  /// If the inverse does not exist, then the result will be
  /// [`IsNan`](crate::IsNan).
  ///
  /// Note: Inverse of zero is defined to be NaN even for [`struct@Scalar`]
  /// values, which differs to division in IEEE754.
  fn inverse(self) -> Self;
}

impl Inverse for Scalar {
  #[rustfmt::skip]
  #[inline]
  fn inverse(self) -> Self {
    let Scalar { s: ms } = self;

    let s = if ms == 0.0 { F::NAN } else { 1./ms };

    Scalar { s }
  }
}

impl Inverse for Vector {
  #[rustfmt::skip]
  #[inline]
  fn inverse(self) -> Self {
    let Vector { e1: m1, e2: m2, e3: m3, e4: m4 } = self;

    let q = {
      let d = m1*m1 + m2*m2 + m3*m3;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let e1 = q*m1;
    let e2 = q*m2;
    let e3 = q*m3;
    let e4 = q*m4;

    Vector { e1, e2, e3, e4 }
  }
}

impl Inverse for Bivector {
  #[rustfmt::skip]
  #[inline]
  fn inverse(self) -> Self {
    let Bivector {
      e41: b41, e42: b42, e43: b43, e23: b23, e31: b31, e12: b12,
    } = self;

    let q = {
      let d = b23*b23 + b31*b31 + b12*b12;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let y = -q*2.*(b41*b23 + b42*b31 + b43*b12);

    let e41 = -q*(b41 + b23*y);
    let e42 = -q*(b42 + b31*y);
    let e43 = -q*(b43 + b12*y);
    let e23 = -q*b23;
    let e31 = -q*b31;
    let e12 = -q*b12;

    Bivector { e41, e42, e43, e23, e31, e12 }
  }
}

impl Inverse for Trivector {
  #[rustfmt::skip]
  #[inline]
  fn inverse(self) -> Self {
    let Trivector { e423: m423, e431: m431, e412: m412, e321: m321 } = self;

    let q = {
      let d = m321*m321;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let e423 = -q*m423;
    let e431 = -q*m431;
    let e412 = -q*m412;
    let e321 = -q*m321;

    Trivector {
      e423,
      e431,
      e412,
      e321,
    }
  }
}

impl Inverse for Antiscalar {
  /// Antiscalars are not-invertible, so the result is unconditionally
  /// [`IsNan`](crate::IsNan)
  ///
  /// The inverse for an Antiscalar does not exist, as there is no pair of
  /// antiscalar *a* & multivector *m*, that can satisfy the geometric product
  /// *a* ⟑ *m* = 1.
  #[inline]
  fn inverse(self) -> Self {
    Antiscalar { e1234: F::NAN }
  }
}

impl Inverse for DualNumber {
  #[rustfmt::skip]
  #[inline]
  fn inverse(self) -> Self {
    let DualNumber { s: ms, e1234: m1234 } = self;

    let q = if ms == 0.0 { F::NAN } else { 1./ms };

    let s = q;
    let e1234 = -q*q*m1234;

    DualNumber { s, e1234 }
  }
}

impl Inverse for OddGrade {
  #[rustfmt::skip]
  #[inline]
  fn inverse(self) -> Self {
    let OddGrade {
      e1: m1, e2: m2, e3: m3, e4: m4,
      e423: m423, e431: m431, e412: m412, e321: m321,
    } = self;

    let q = {
      let d = m1*m1 + m2*m2 + m3*m3 + m321*m321;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let y = -2.*q*(m1*m423 + m2*m431 + m3*m412 + m4*m321);

    let e1 = q*m1;
    let e2 = q*m2;
    let e3 = q*m3;
    let e4 = q*(m4 + m321*y);
    let e423 = -q*(m423 + m1*y);
    let e431 = -q*(m431 + m2*y);
    let e412 = -q*(m412 + m3*y);
    let e321 = -q*m321;

    OddGrade {
      e1, e2, e3, e4,
      e423, e431, e412, e321,
    }
  }
}

impl Inverse for EvenGrade {
  #[rustfmt::skip]
  #[inline]
  fn inverse(self) -> Self {
    let EvenGrade {
      s: ms,
      e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
      e1234: m1234,
    } = self;

    let q = {
      let d = m23*m23 + m31*m31 + m12*m12 + ms*ms;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let y = 2.*q*(m23*m41 + m31*m42 + m12*m43 + ms*m1234);

    let s = q*ms;
    let e41 = -q*(m41 - m23*y);
    let e42 = -q*(m42 - m31*y);
    let e43 = -q*(m43 - m12*y);
    let e23 = -q*m23;
    let e31 = -q*m31;
    let e12 = -q*m12;
    let e1234 = q*(m1234 - ms*y);

    EvenGrade {
      s,
      e41, e42, e43, e23, e31, e12,
      e1234,
    }
  }
}

impl Inverse for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn inverse(self) -> Self {
    let Multivector {
      s: ms,
      e1: m1, e2: m2, e3: m3, e4: m4,
      e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
      e423: m423, e431: m431, e412: m412, e321: m321,
      e1234: m1234,
    } = self;

    let [a1, a2, a3, a4] = [m1*m1, m2*m2, m3*m3, ms*ms];
    let [b1, b2, b3, b4] = [m23*m23, m31*m31, m12*m12, m321*m321];
    let [c1, c2, c3, c4] = [m2*m31, m3*m12, m1*m23, ms*m321];
    let [d1, d2, d3, d4] = [
        b1 - b2 - b3 + b4,
      - b1 + b2 - b3 + b4,
      - b1 - b2 + b3 + b4,
        b1 + b2 + b3 + b4,
    ];
    let [f1, f2, f3, f4] = [
        a1 - a2 - a3 - a4,
      - a1 + a2 - a3 - a4,
      - a1 - a2 + a3 - a4,
        a1 + a2 + a3 - a4,
    ];
    let [g1, g2, g3, g4] = [
      - 2.*b4 + d1 + f1,
      - 2.*b4 + d2 + f2,
      - 2.*b4 + d3 + f3,
      - 2.*b4 + d4 + f4,
    ];
    let [h1, h2, h3, h4] = [
      -2.*(a4 - b4) - d4 - f1,
      -2.*(a4 - b4) - d4 - f2,
      -2.*(a4 - b4) - d4 - f3,
      -2.*(a4 + b4) + d4 - f4,
    ];
    let [i1, i2, i3, i4] = [m31*m12, m12*m23, m23*m31, m1234*m321];
    let [j1, j2, j3, j4] = [m2*m3, m3*m1, m1*m2, ms*m1234];
    let [k1, k2, k3] = [i1+j1, i2+j2, i3+j3];
    let [l1, l2, l3] = [m23*m321, m31*m321, m12*m321];
    let [n1, n2, n3] = [m43*m31-m42*m12, m41*m12-m43*m23, m42*m23-m41*m31];
    let [o1, o2, o3] = [m4*m1, m4*m2, m4*m3];
    let [p1, p2, p3] = [ms*m1, ms*m2, ms*m3];
    let [r1, r2, r3] = [m423*m321, m431*m321, m412*m321];
    let [s1, s2, s3] = [r1-o1, r2-o2, r3-o3];
    let [t1, t2, t3] = [m2*m12-m3*m31, m3*m23-m1*m12, m1*m31-m2*m23];

    let q = {
      let [u1, u2, u3, u4] = [
        a1*(a1 + 2.*(d1 + a2 - a4)),
        a2*(a2 + 2.*(d2 + a3 - a4)),
        a3*(a3 + 2.*(d3 + a1 - a4)),
        a4*(a4 + 2.*d4),
      ];
      let [v1, v2, v3, v4] = [
        b1*(b1 + 2.*(b2 - b4)) + 8.*c3*(c1 + c4),
        b2*(b2 + 2.*(b3 - b4)) + 8.*c1*(c2 + c4),
        b3*(b3 + 2.*(b1 - b4)) + 8.*c2*(c3 + c4),
        b4*b4,
      ];
      let w = (u1 + v1) + (u2 + v2) + (u3 + v3) + (u4 + v4);
      if w == 0.0 { F::NAN } else { 1./w }
    };

    let e1 = q*(m1*(d1 + f4) + 2.*(m2*i3 + m3*i2 + ms*l1));
    let e2 = q*(m2*(d2 + f4) + 2.*(m3*i1 + m1*i3 + ms*l2));
    let e3 = q*(m3*(d3 + f4) + 2.*(m1*i2 + m2*i1 + ms*l3));
    let s  = q*(ms*(d4 - f4) + 2.*(m1*l1 + m2*l2 + m3*l3));

    let e23  = q*(m23*h1 - 2.*(m31*j3 + m12*j2 + m1*c4));
    let e31  = q*(m31*h2 - 2.*(m12*j1 + m23*j3 + m2*c4));
    let e12  = q*(m12*h3 - 2.*(m23*j2 + m31*j1 + m3*c4));
    let e321 = q*(m321*h4 - 2.*ms*(m1*m23 + m2*m31 + m3*m12));

    let e41 = q*(m41*g1 + 2.*(m12*s2 - m31*s3 + m42*k3 + m43*k2
      - m431*p3 + m412*p2 + m1*i4 + m23*j4));
    let e42 = q*(m42*g2 + 2.*(m23*s3 - m12*s1 + m43*k1 + m41*k3
      - m412*p1 + m423*p3 + m2*i4 + m31*j4));
    let e43 = q*(m43*g3 + 2.*(m31*s1 - m23*s2 + m41*k2 + m42*k1
      - m423*p2 + m431*p1 + m3*i4 + m12*j4));
    let e4  = q*(m4*g4 - 2.*(m1*(r1 - n1) + m2*(r2 - n2) + m3*(r3 - n3)
      + ms*(m23*m423 + m31*m431 + m12*m412)));

    let e423  = q*(m423*g1 + 2.*(m431*k3 + m412*k2 + m1234*t1
      + m321*(n1 + o1) + ms*(m3*m42 - m2*m43 + m4*m23)));
    let e431  = q*(m431*g2 + 2.*(m412*k1 + m423*k3 + m1234*t2
      + m321*(n2 + o2) + ms*(m1*m43 - m3*m41 + m4*m31)));
    let e412  = q*(m412*g3 + 2.*(m423*k2 + m431*k1 + m1234*t3
      + m321*(n3 + o3) + ms*(m2*m41 - m1*m42 + m4*m12)));
    let e1234 = q*(m1234*g4 - 2.*(m423*t1 + m431*t2 + m412*t3
      + m321*(m1*m41 + m2*m42 + m3*m43) + ms*(m41*m23 + m42*m31 + m43*m12)));

    Multivector {
      s,
      e1, e2, e3, e4,
      e41, e42, e43, e23, e31, e12,
      e423, e431, e412, e321,
      e1234,
    }
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
      fn inverse_*() {
        assert_ulps_eq!(
          Multivector::from(dbg!(geometric_product(variant, inverse(variant)))),
          Multivector::ONE,
          epsilon = 0.00_000_000_000_1
        );
        assert_ulps_eq!(
          Multivector::from(dbg!(geometric_product(inverse(variant), variant))),
          Multivector::ONE,
          epsilon = 0.00_000_000_000_1
        );
      }
    }
  }

  #[test]
  fn no_inverse_multivector() {
    assert!(inverse(Multivector::E4).is_nan());
    assert!(inverse(Multivector::ZERO).is_nan());
  }

  #[test]
  fn no_inverse_scalar() {
    assert!(inverse(Scalar::ZERO).is_nan());
  }

  #[test]
  fn no_inverse_vector() {
    assert!(inverse(Vector::E4).is_nan());
    assert!(inverse(Vector::ZERO).is_nan());
  }

  #[test]
  fn no_inverse_bivector() {
    assert!(inverse(Bivector::E41).is_nan());
    assert!(inverse(Bivector::ZERO).is_nan());
  }

  #[test]
  fn no_inverse_trivector() {
    assert!(inverse(Trivector::E423).is_nan());
    assert!(inverse(Trivector::ZERO).is_nan());
  }

  #[test]
  fn no_inverse_antiscalar() {
    assert!(inverse(Antiscalar::E1234).is_nan());
    assert!(inverse(Antiscalar::ZERO).is_nan());
  }

  #[test]
  fn no_inverse_dual_number() {
    assert!(inverse(DualNumber::E1234).is_nan());
    assert!(inverse(DualNumber::ZERO).is_nan());
  }

  #[test]
  fn no_inverse_odd_grade() {
    assert!(inverse(OddGrade::E4).is_nan());
    assert!(inverse(OddGrade::ZERO).is_nan());
  }

  #[test]
  fn no_inverse_even_grade() {
    assert!(inverse(EvenGrade::E41).is_nan());
    assert!(inverse(EvenGrade::ZERO).is_nan());
  }
}
