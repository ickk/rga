use crate::{
  algebra::values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  Inverse, LeftComplement, RightComplement, F,
};

/// Compute the inverse with respect to the
/// [`GeometricAntiproduct`](crate::GeometricAntiproduct) if it exists. If it
/// is non-invertible then the returned value will contain a NaN.
///
/// If the inverse does not exist, then the result will be
/// [`IsNan`](crate::IsNan).
///
/// Note: Inverse of zero is defined to be NaN even for [`struct@Antiscalar`]
/// values, which differs to division in IEEE754.
#[inline]
pub fn antiinverse<M>(u: M) -> M
where
  M: Antiinverse,
{
  u.antiinverse()
}

/// Compute the inverse with respect to the
/// [`GeometricAntiproduct`](crate::GeometricAntiproduct) if it exists. If it
/// is non-invertible then the returned value will contain a NaN.
///
/// If the inverse does not exist, then the result will be
/// [`IsNan`](crate::IsNan).
///
/// Note: Inverse of zero is defined to be NaN even for [`struct@Antiscalar`]
/// values, which differs to division in IEEE754.
pub trait Antiinverse {
  /// Compute the inverse with respect to the
  /// [`GeometricAntiproduct`](crate::GeometricAntiproduct) if it exists. If it
  /// is non-invertible then the returned value will contain a NaN.
  ///
  /// If the inverse does not exist, then the result will be
  /// [`IsNan`](crate::IsNan).
  ///
  /// Note: Inverse of zero is defined to be NaN even for [`struct@Antiscalar`]
  /// values, which differs to division in IEEE754.
  fn antiinverse(self) -> Self;
}

impl Antiinverse for Scalar {
  #[rustfmt::skip]
  #[inline]
  fn antiinverse(self) -> Self {
    Scalar::NAN
  }
}

impl Antiinverse for Vector {
  #[rustfmt::skip]
  #[inline]
  fn antiinverse(self) -> Self {
    let Vector { e1: m1, e2: m2, e3: m3, e4: m4 } = self;

    let q = {
      let d = m4*m4;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let e1 = -q*m1;
    let e2 = -q*m2;
    let e3 = -q*m3;
    let e4 = -q*m4;

    Vector { e1, e2, e3, e4 }
  }
}

impl Antiinverse for Bivector {
  #[rustfmt::skip]
  #[inline]
  fn antiinverse(self) -> Self {
    let Bivector {
      e41: b41, e42: b42, e43: b43, e23: b23, e31: b31, e12: b12,
    } = self;

    let q = {
      let d = b41*b41 + b42*b42 + b43*b43;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let y = -q*2.*(b41*b23 + b42*b31 + b43*b12);

    let e41 = -q*b41;
    let e42 = -q*b42;
    let e43 = -q*b43;
    let e23 = -q*(b23 + b41*y);
    let e31 = -q*(b31 + b42*y);
    let e12 = -q*(b12 + b43*y);

    Bivector { e41, e42, e43, e23, e31, e12 }
  }
}

impl Antiinverse for Trivector {
  #[rustfmt::skip]
  #[inline]
  fn antiinverse(self) -> Self {
    let Trivector { e423: m423, e431: m431, e412: m412, e321: m321 } = self;

    let q = {
      let d = m423*m423 + m431*m431 + m412*m412;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let e423 = q*m423;
    let e431 = q*m431;
    let e412 = q*m412;
    let e321 = q*m321;

    Trivector {
      e423,
      e431,
      e412,
      e321,
    }
  }
}

impl Antiinverse for Antiscalar {
  /// Antiscalars are not-invertible, so the result is unconditionally
  /// [`IsNan`](crate::IsNan)
  ///
  /// The Antiinverse for an Antiscalar does not exist, as there is no pair of
  /// antiscalar *a* & multivector *m*, that can satisfy the geometric product
  /// *a* ⟑ *m* = 1.
  #[inline]
  fn antiinverse(self) -> Self {
    let Antiscalar { e1234: m1234 } = self;
    let e1234 = if m1234 == 0.0 { F::NAN } else { 1. / m1234 };
    Antiscalar { e1234 }
  }
}

impl Antiinverse for DualNumber {
  #[rustfmt::skip]
  #[inline]
  fn antiinverse(self) -> Self {
    let DualNumber { s: ms, e1234: m1234 } = self;

    let q = if m1234 == 0.0 { F::NAN } else { 1./m1234 };

    let s = -q*q*ms;
    let e1234 = q;

    DualNumber { s, e1234 }
  }
}

impl Antiinverse for OddGrade {
  #[rustfmt::skip]
  #[inline]
  fn antiinverse(self) -> Self {
    let OddGrade {
      e1: m1, e2: m2, e3: m3, e4: m4,
      e423: m423, e431: m431, e412: m412, e321: m321,
    } = self;

    let q = {
      let d = m423*m423 + m431*m431 + m412*m412 + m4*m4;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let y = -2.*q*(m1*m423 + m2*m431 + m3*m412 + m4*m321);

    let e1 = -q*(m1 + m423*y);
    let e2 = -q*(m2 + m431*y);
    let e3 = -q*(m3 + m412*y);
    let e4 = -q*m4;
    let e423 = q*m423;
    let e431 = q*m431;
    let e412 = q*m412;
    let e321 = q*(m321 + m4*y);

    OddGrade {
      e1, e2, e3, e4,
      e423, e431, e412, e321,
    }
  }
}

impl Antiinverse for EvenGrade {
  #[rustfmt::skip]
  #[inline]
  fn antiinverse(self) -> Self {
    let EvenGrade {
      s: ms,
      e41: m41, e42: m42, e43: m43, e23: m23, e31: m31, e12: m12,
      e1234: m1234,
    } = self;

    let q = {
      let d = m41*m41 + m42*m42 + m43*m43 + m1234*m1234;
      if d == 0.0 { F::NAN } else { 1./d }
    };

    let y = 2.*q*(m23*m41 + m31*m42 + m12*m43 + ms*m1234);

    let s = q*(ms - m1234*y);
    let e41 = -q*m41;
    let e42 = -q*m42;
    let e43 = -q*m43;
    let e23 = -q*(m23 - m41*y);
    let e31 = -q*(m31 - m42*y);
    let e12 = -q*(m12 - m43*y);
    let e1234 = q*m1234;

    EvenGrade {
      s,
      e41, e42, e43, e23, e31, e12,
      e1234,
    }
  }
}

impl Antiinverse for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn antiinverse(self) -> Self {
    self.left_complement().inverse().right_complement()
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
      fn antiinverse_*() {
        assert_ulps_eq!(
          Multivector::from(dbg!(geometric_antiproduct(variant, antiinverse(variant)))),
          dbg!(Multivector::E1234),
          epsilon = 0.00_000_000_000_1
        );
        assert_ulps_eq!(
          Multivector::from(dbg!(geometric_antiproduct(antiinverse(variant), variant))),
          dbg!(Multivector::E1234),
          epsilon = 0.00_000_000_000_1
        );
      }
    }
  }

  #[test]
  fn no_antiinverse_multivector() {
    assert!(antiinverse(Multivector::E321).is_nan());
    assert!(antiinverse(Multivector::ZERO).is_nan());
  }

  #[test]
  fn no_antiinverse_scalar() {
    assert!(antiinverse(Scalar::ONE).is_nan());
    assert!(antiinverse(Antiscalar::ZERO).is_nan());
  }

  #[test]
  fn no_antiinverse_vector() {
    assert!(antiinverse(Vector::E1).is_nan());
    assert!(antiinverse(Vector::ZERO).is_nan());
  }

  #[test]
  fn no_antiinverse_bivector() {
    assert!(antiinverse(Bivector::E23).is_nan());
    assert!(antiinverse(Bivector::ZERO).is_nan());
  }

  #[test]
  fn no_antiinverse_trivector() {
    assert!(antiinverse(Trivector::E321).is_nan());
    assert!(antiinverse(Trivector::ZERO).is_nan());
  }

  #[test]
  fn no_antiinverse_antiscalar() {
    assert!(antiinverse(Antiscalar::ZERO).is_nan());
  }

  #[test]
  fn no_antiinverse_dual_number() {
    assert!(antiinverse(DualNumber::ONE).is_nan());
    assert!(antiinverse(DualNumber::ZERO).is_nan());
  }

  #[test]
  fn no_antiinverse_odd_grade() {
    assert!(antiinverse(OddGrade::E321).is_nan());
    assert!(antiinverse(OddGrade::ZERO).is_nan());
  }

  #[test]
  fn no_antiinverse_even_grade() {
    assert!(antiinverse(EvenGrade::E23).is_nan());
    assert!(antiinverse(EvenGrade::ZERO).is_nan());
  }
}
