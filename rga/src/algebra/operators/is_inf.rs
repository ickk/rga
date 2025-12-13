use crate::algebra::{
  operators::IsNan,
  values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Matrix4, Multivector,
    OddGrade, Scalar, Trivector, Vector,
  },
};

pub trait IsInf {
  /// returns `true` if any element is positive or negative infinity
  fn is_infinite(&self) -> bool;

  /// returns `false` if any element is NaN or is positive or negative infinity
  fn is_finite(&self) -> bool
  where
    Self: IsNan,
  {
    !(self.is_infinite() && self.is_nan())
  }
}

impl IsInf for Multivector {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.s.is_infinite()
      || self.e1.is_infinite()
      || self.e2.is_infinite()
      || self.e3.is_infinite()
      || self.e4.is_infinite()
      || self.e41.is_infinite()
      || self.e42.is_infinite()
      || self.e43.is_infinite()
      || self.e23.is_infinite()
      || self.e31.is_infinite()
      || self.e12.is_infinite()
      || self.e423.is_infinite()
      || self.e431.is_infinite()
      || self.e412.is_infinite()
      || self.e321.is_infinite()
      || self.e1234.is_infinite()
  }
}

impl IsInf for Scalar {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.s.is_infinite()
  }
}

impl IsInf for Vector {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.e1.is_infinite()
      || self.e2.is_infinite()
      || self.e3.is_infinite()
      || self.e4.is_infinite()
  }
}

impl IsInf for Bivector {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.e41.is_infinite()
      || self.e42.is_infinite()
      || self.e43.is_infinite()
      || self.e23.is_infinite()
      || self.e31.is_infinite()
      || self.e12.is_infinite()
  }
}

impl IsInf for Trivector {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.e423.is_infinite()
      || self.e431.is_infinite()
      || self.e412.is_infinite()
      || self.e321.is_infinite()
  }
}

impl IsInf for Antiscalar {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.e1234.is_infinite()
  }
}

impl IsInf for DualNumber {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.s.is_infinite() || self.e1234.is_infinite()
  }
}

impl IsInf for EvenGrade {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.s.is_infinite()
      || self.e41.is_infinite()
      || self.e42.is_infinite()
      || self.e43.is_infinite()
      || self.e23.is_infinite()
      || self.e31.is_infinite()
      || self.e12.is_infinite()
      || self.e1234.is_infinite()
  }
}

impl IsInf for OddGrade {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.e1.is_infinite()
      || self.e2.is_infinite()
      || self.e3.is_infinite()
      || self.e4.is_infinite()
      || self.e423.is_infinite()
      || self.e431.is_infinite()
      || self.e412.is_infinite()
      || self.e321.is_infinite()
  }
}

impl IsInf for Matrix4 {
  #[inline]
  fn is_infinite(&self) -> bool {
    self.m11.is_infinite()
      || self.m21.is_infinite()
      || self.m31.is_infinite()
      || self.m41.is_infinite()
      || self.m12.is_infinite()
      || self.m22.is_infinite()
      || self.m32.is_infinite()
      || self.m42.is_infinite()
      || self.m13.is_infinite()
      || self.m23.is_infinite()
      || self.m33.is_infinite()
      || self.m43.is_infinite()
      || self.m14.is_infinite()
      || self.m24.is_infinite()
      || self.m34.is_infinite()
      || self.m44.is_infinite()
  }
}
