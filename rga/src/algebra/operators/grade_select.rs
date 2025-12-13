use crate::algebra::values::{
  zero, Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
  Scalar, Trivector, Vector,
};

/// 〈u〉₀
///
/// Get the grade 0 [`struct@Scalar`] element
#[inline]
pub fn grade_0<M>(u: M) -> Scalar
where
  M: GradeSelect,
{
  u.grade_0()
}

/// 〈u〉₁
///
/// Get the grade 1 [`struct@Vector`] elements
#[inline]
pub fn grade_1<M>(u: M) -> Vector
where
  M: GradeSelect,
{
  u.grade_1()
}

/// 〈u〉₂
///
/// Get the grade 2 [`struct@Bivector`] elements
#[inline]
pub fn grade_2<M>(u: M) -> Bivector
where
  M: GradeSelect,
{
  u.grade_2()
}

/// 〈u〉₃
///
/// Get the grade 3 [`struct@Trivector`] elements
#[inline]
pub fn grade_3<M>(u: M) -> Trivector
where
  M: GradeSelect,
{
  u.grade_3()
}

/// 〈u〉₄
///
/// Get the grade 4 [`struct@Antiscalar`] element
#[inline]
pub fn grade_4<M>(u: M) -> Antiscalar
where
  M: GradeSelect,
{
  u.grade_4()
}

/// 〈u〉₀ + 〈u〉₄
///
/// Get the grades 0 & 4 elements
#[inline]
pub fn grade_0_4<M>(u: M) -> DualNumber
where
  M: GradeSelect + Copy,
{
  u.grade_0_4()
}

/// 〈u〉₁ + 〈u〉₃
///
/// Get the grades 1 & 3 elements
#[inline]
pub fn grade_1_3<M>(u: M) -> OddGrade
where
  M: GradeSelect + Copy,
{
  u.grade_1_3()
}

/// 〈u〉₀ + 〈u〉₂ + 〈u〉₄
///
/// Get the grades 0, 2, & 4 elements
#[inline]
pub fn grade_0_2_4<M>(u: M) -> EvenGrade
where
  M: GradeSelect + Copy,
{
  u.grade_0_2_4()
}

/// 〈u〉ₖ
pub trait GradeSelect {
  /// 〈u〉₀
  ///
  /// Get the grade 0 [`struct@Scalar`] element
  fn grade_0(self) -> Scalar;

  /// 〈u〉₁
  ///
  /// Get the grade 1 [`struct@Vector`] elements
  fn grade_1(self) -> Vector;

  /// 〈u〉₂
  ///
  /// Get the grade 2 [`struct@Bivector`] elements
  fn grade_2(self) -> Bivector;

  /// 〈u〉₃
  ///
  /// Get the grade 3 [`struct@Trivector`] elements
  fn grade_3(self) -> Trivector;

  /// 〈u〉₄
  ///
  /// Get the grade 4 [`struct@Antiscalar`] element
  fn grade_4(self) -> Antiscalar;

  /// 〈u〉₀ + 〈u〉₄
  ///
  /// Get the grades 0 & 4 elements
  #[inline]
  fn grade_0_4(self) -> DualNumber
  where
    Self: Copy,
  {
    self.grade_0() + self.grade_4()
  }

  /// 〈u〉₁ + 〈u〉₃
  ///
  /// Get the grades 1 & 3 elements
  #[inline]
  fn grade_1_3(self) -> OddGrade
  where
    Self: Copy,
  {
    self.grade_1() + self.grade_3()
  }

  /// 〈u〉₀ + 〈u〉₂ + 〈u〉₄
  ///
  /// Get the grades 0, 2, & 4 elements
  #[inline]
  fn grade_0_2_4(self) -> EvenGrade
  where
    Self: Copy,
  {
    self.grade_0() + self.grade_2() + self.grade_4()
  }
}

impl GradeSelect for Multivector {
  #[inline]
  fn grade_0(self) -> Scalar {
    Scalar { s: self.s }
  }
  #[inline]
  fn grade_1(self) -> Vector {
    Vector {
      e1: self.e1,
      e2: self.e2,
      e3: self.e3,
      e4: self.e4,
    }
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    Bivector {
      e41: self.e41,
      e42: self.e42,
      e43: self.e43,
      e23: self.e23,
      e31: self.e31,
      e12: self.e12,
    }
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    Trivector {
      e423: self.e423,
      e431: self.e431,
      e412: self.e412,
      e321: self.e321,
    }
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    Antiscalar { e1234: self.e1234 }
  }
}

impl GradeSelect for Scalar {
  #[inline]
  fn grade_0(self) -> Scalar {
    Scalar { s: self.s }
  }
  #[inline]
  fn grade_1(self) -> Vector {
    zero()
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    zero()
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    zero()
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    zero()
  }
}

impl GradeSelect for Vector {
  #[inline]
  fn grade_0(self) -> Scalar {
    zero()
  }
  #[inline]
  fn grade_1(self) -> Vector {
    Vector {
      e1: self.e1,
      e2: self.e2,
      e3: self.e3,
      e4: self.e4,
    }
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    zero()
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    zero()
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    zero()
  }
}

impl GradeSelect for Bivector {
  #[inline]
  fn grade_0(self) -> Scalar {
    zero()
  }
  #[inline]
  fn grade_1(self) -> Vector {
    zero()
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    Bivector {
      e41: self.e41,
      e42: self.e42,
      e43: self.e43,
      e23: self.e23,
      e31: self.e31,
      e12: self.e12,
    }
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    zero()
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    zero()
  }
}

impl GradeSelect for Trivector {
  #[inline]
  fn grade_0(self) -> Scalar {
    zero()
  }
  #[inline]
  fn grade_1(self) -> Vector {
    zero()
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    zero()
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    Trivector {
      e423: self.e423,
      e431: self.e431,
      e412: self.e412,
      e321: self.e321,
    }
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    zero()
  }
}

impl GradeSelect for Antiscalar {
  #[inline]
  fn grade_0(self) -> Scalar {
    zero()
  }
  #[inline]
  fn grade_1(self) -> Vector {
    zero()
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    zero()
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    zero()
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    Antiscalar { e1234: self.e1234 }
  }
}

impl GradeSelect for DualNumber {
  #[inline]
  fn grade_0(self) -> Scalar {
    Scalar { s: self.s }
  }
  #[inline]
  fn grade_1(self) -> Vector {
    zero()
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    zero()
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    zero()
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    Antiscalar { e1234: self.e1234 }
  }
}

impl GradeSelect for OddGrade {
  #[inline]
  fn grade_0(self) -> Scalar {
    zero()
  }
  #[inline]
  fn grade_1(self) -> Vector {
    Vector {
      e1: self.e1,
      e2: self.e2,
      e3: self.e3,
      e4: self.e4,
    }
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    zero()
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    Trivector {
      e423: self.e423,
      e431: self.e431,
      e412: self.e412,
      e321: self.e321,
    }
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    zero()
  }
}

impl GradeSelect for EvenGrade {
  #[inline]
  fn grade_0(self) -> Scalar {
    Scalar { s: self.s }
  }
  #[inline]
  fn grade_1(self) -> Vector {
    zero()
  }
  #[inline]
  fn grade_2(self) -> Bivector {
    Bivector {
      e41: self.e41,
      e42: self.e42,
      e43: self.e43,
      e23: self.e23,
      e31: self.e31,
      e12: self.e12,
    }
  }
  #[inline]
  fn grade_3(self) -> Trivector {
    zero()
  }
  #[inline]
  fn grade_4(self) -> Antiscalar {
    Antiscalar { e1234: self.e1234 }
  }
}
