use crate::algebra::values::{
  Antiscalar, Bivector, DualNumber, EvenGrade, Matrix4, Multivector, OddGrade,
  Scalar, Trivector, Vector,
};

pub trait IsNan {
  /// returns `true` if element is NaN
  fn is_nan(&self) -> bool;
}

impl IsNan for Multivector {
  #[inline]
  fn is_nan(&self) -> bool {
    self.s.is_nan()
      || self.e1.is_nan()
      || self.e2.is_nan()
      || self.e3.is_nan()
      || self.e4.is_nan()
      || self.e41.is_nan()
      || self.e42.is_nan()
      || self.e43.is_nan()
      || self.e23.is_nan()
      || self.e31.is_nan()
      || self.e12.is_nan()
      || self.e423.is_nan()
      || self.e431.is_nan()
      || self.e412.is_nan()
      || self.e321.is_nan()
      || self.e1234.is_nan()
  }
}

impl IsNan for Scalar {
  #[inline]
  fn is_nan(&self) -> bool {
    self.s.is_nan()
  }
}

impl IsNan for Vector {
  #[inline]
  fn is_nan(&self) -> bool {
    self.e1.is_nan()
      || self.e2.is_nan()
      || self.e3.is_nan()
      || self.e4.is_nan()
  }
}

impl IsNan for Bivector {
  #[inline]
  fn is_nan(&self) -> bool {
    self.e41.is_nan()
      || self.e42.is_nan()
      || self.e43.is_nan()
      || self.e23.is_nan()
      || self.e31.is_nan()
      || self.e12.is_nan()
  }
}

impl IsNan for Trivector {
  #[inline]
  fn is_nan(&self) -> bool {
    self.e423.is_nan()
      || self.e431.is_nan()
      || self.e412.is_nan()
      || self.e321.is_nan()
  }
}

impl IsNan for Antiscalar {
  #[inline]
  fn is_nan(&self) -> bool {
    self.e1234.is_nan()
  }
}

impl IsNan for DualNumber {
  #[inline]
  fn is_nan(&self) -> bool {
    self.s.is_nan() || self.e1234.is_nan()
  }
}

impl IsNan for EvenGrade {
  #[inline]
  fn is_nan(&self) -> bool {
    self.s.is_nan()
      || self.e41.is_nan()
      || self.e42.is_nan()
      || self.e43.is_nan()
      || self.e23.is_nan()
      || self.e31.is_nan()
      || self.e12.is_nan()
      || self.e1234.is_nan()
  }
}

impl IsNan for OddGrade {
  #[inline]
  fn is_nan(&self) -> bool {
    self.e1.is_nan()
      || self.e2.is_nan()
      || self.e3.is_nan()
      || self.e4.is_nan()
      || self.e423.is_nan()
      || self.e431.is_nan()
      || self.e412.is_nan()
      || self.e321.is_nan()
  }
}

impl IsNan for Matrix4 {
  #[inline]
  fn is_nan(&self) -> bool {
    self.m11.is_nan()
      || self.m21.is_nan()
      || self.m31.is_nan()
      || self.m41.is_nan()
      || self.m12.is_nan()
      || self.m22.is_nan()
      || self.m32.is_nan()
      || self.m42.is_nan()
      || self.m13.is_nan()
      || self.m23.is_nan()
      || self.m33.is_nan()
      || self.m43.is_nan()
      || self.m14.is_nan()
      || self.m24.is_nan()
      || self.m34.is_nan()
      || self.m44.is_nan()
  }
}
