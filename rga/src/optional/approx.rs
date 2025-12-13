use {
  crate::{
    algebra::values::{
      Antiscalar, Bivector, DualNumber, EvenGrade, Matrix4, Multivector,
      OddGrade, Scalar, Trivector, Unit, Vector,
    },
    F,
  },
  ::approx::{AbsDiffEq, RelativeEq, UlpsEq},
};

macro_rules! impl_approx_eq_self {
  ($($type:ty { $($field_name:ident),* $(,)? })*) => {$(
    impl AbsDiffEq for $type {
      type Epsilon = F;
      #[inline]
      fn default_epsilon() -> Self::Epsilon {
        F::default_epsilon()
      }
      #[inline]
      fn abs_diff_eq(&self, other: &Self, epsilon: F) -> bool {
        $(F::abs_diff_eq(
          &self.$field_name,
          &other.$field_name,
          epsilon
        ) &&)* true
      }
    }
    impl RelativeEq for $type {
      #[inline]
      fn default_max_relative() -> F {
        F::default_max_relative()
      }
      #[inline]
      fn relative_eq(&self, other: &Self, epsilon: F, max_relative: F) -> bool {
        $(F::relative_eq(
          &self.$field_name,
          &other.$field_name,
          epsilon,
          max_relative
        ) &&)* true
      }
    }
    impl UlpsEq for $type {
      #[inline]
      fn default_max_ulps() -> u32 {
        F::default_max_ulps()
      }
      #[inline]
      fn ulps_eq(&self, other: &Self, epsilon: F, max_ulps: u32) -> bool {
        $(F::ulps_eq(
          &self.$field_name,
          &other.$field_name,
          epsilon,
          max_ulps
        ) &&)* true
      }
    }
  )*};
}

impl_approx_eq_self! {
  Multivector {
    e1, e2, e3, e4,
    e41, e42, e43, s,
    e23, e31, e12, e1234,
    e423, e431, e412, e321,
  }
  Scalar { s }
  Vector { e1, e2, e3, e4 }
  Bivector { e41, e42, e43, e23, e31, e12 }
  Trivector { e423, e431, e412, e321 }
  Antiscalar { e1234 }
  DualNumber { s, e1234 }
  OddGrade { e1, e2, e3, e4, e423, e431, e412, e321 }
  EvenGrade { e41, e42, e43, s, e23, e31, e12, e1234 }
  Matrix4 {
    m11, m21, m31, m41,
    m12, m22, m32, m42,
    m13, m23, m33, m43,
    m14, m24, m34, m44,
  }
}

macro_rules! impl_approx_eq_unit_self {
  ($($lhs:ty, $rhs:ty;)*) => {$(
    impl AbsDiffEq<Unit<$rhs>> for Unit<$lhs> {
      type Epsilon = F;
      #[inline]
      fn default_epsilon() -> Self::Epsilon {
        F::default_epsilon()
      }
      #[inline]
      fn abs_diff_eq(&self, other: &Self, epsilon: F) -> bool {
        <$lhs>::abs_diff_eq(&self.0, &other.0, epsilon)
      }
    }
    impl RelativeEq<Unit<$rhs>> for Unit<$lhs> {
      #[inline]
      fn default_max_relative() -> F {
        F::default_max_relative()
      }
      #[inline]
      fn relative_eq(&self, other: &Self, epsilon: F, max_relative: F)
      -> bool {
        <$lhs>::relative_eq(
          &self.0,
          &other.0,
          epsilon,
          max_relative
        )
      }
    }
    impl UlpsEq<Unit<$rhs>> for Unit<$lhs> {
      #[inline]
      fn default_max_ulps() -> u32 {
        F::default_max_ulps()
      }
      #[inline]
      fn ulps_eq(&self, other: &Self, epsilon: F, max_ulps: u32) -> bool {
        <$lhs>::ulps_eq(
          &self.0,
          &other.0,
          epsilon,
          max_ulps
        )
      }
    }
  )*};
}

impl_approx_eq_unit_self!(
  Multivector, Multivector;
  Scalar, Scalar;
  Vector, Vector;
  Bivector, Bivector;
  Trivector, Trivector;
  Antiscalar, Antiscalar;
  DualNumber, DualNumber;
  OddGrade, OddGrade;
  EvenGrade, EvenGrade;
);
