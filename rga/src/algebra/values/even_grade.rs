use {
  crate::{
    algebra::values::{Antiscalar, Bivector, DualNumber, Scalar},
    F,
  },
  ::core::fmt,
};

/// s +
/// l<sub>vx</sub>𝐞₄₁ + l<sub>vy</sub>𝐞₄₂ + l<sub>vz</sub>𝐞₄₃ +
/// l<sub>mx</sub>𝐞₂₃ + l<sub>my</sub>𝐞₃₁ + l<sub>mz</sub>𝐞₁₂ +
/// v𝐞₁₂₃₄
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct EvenGrade {
  /// l<sub>vx</sub> 𝐞₄₁ = l<sub>vx</sub> 𝐞₄∧𝐞₁
  pub e41: F,
  /// l<sub>vy</sub> 𝐞₄₂ = l<sub>vy</sub> 𝐞₄∧𝐞₂
  pub e42: F,
  /// l<sub>vz</sub> 𝐞₄₃ = l<sub>vz</sub> 𝐞₄∧𝐞₃
  pub e43: F,
  /// v 𝟙 = v 𝐞₁₂₃₄ = v 𝐞₁∧𝐞₂∧𝐞₃∧𝐞₄
  pub e1234: F,
  /// l<sub>mx</sub> 𝐞₂₃ = l<sub>mx</sub> 𝐞₂∧𝐞₃
  pub e23: F,
  /// l<sub>my</sub> 𝐞₃₁ = l<sub>my</sub> 𝐞₃∧𝐞₁
  pub e31: F,
  /// l<sub>mz</sub> 𝐞₁₂ = l<sub>mz</sub> 𝐞₁∧𝐞₂
  pub e12: F,
  /// s
  pub s: F,
}

impl EvenGrade {
  /// 𝟎
  pub const ZERO: Self = Self {
    s: 0.,
    e41: 0.,
    e42: 0.,
    e43: 0.,
    e23: 0.,
    e31: 0.,
    e12: 0.,
    e1234: 0.,
  };
  /// The unit scalar element, 𝟏
  pub const ONE: Self = Self {
    s: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₄₁ = 𝐞₄∧𝐞₁ bivector
  pub const E41: Self = Self {
    e41: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₄₂ = 𝐞₄∧𝐞₂ bivector
  pub const E42: Self = Self {
    e42: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₄₃ = 𝐞₄∧𝐞₃ bivector
  pub const E43: Self = Self {
    e43: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₂₃ = 𝐞₂∧𝐞₃ bivector
  pub const E23: Self = Self {
    e23: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₃₁ = 𝐞₃∧𝐞₁ bivector
  pub const E31: Self = Self {
    e31: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₁₂ = 𝐞₁∧𝐞₂ bivector
  pub const E12: Self = Self {
    e12: 1.,
    ..Self::ZERO
  };
  /// The unit volume element, 𝟙 = 1𝐞₁₂₃₄
  pub const E1234: Self = Self {
    e1234: 1.,
    ..Self::ZERO
  };
  /// All NaNs
  pub const NAN: Self = Self {
    s: F::NAN,
    e41: F::NAN,
    e42: F::NAN,
    e43: F::NAN,
    e23: F::NAN,
    e31: F::NAN,
    e12: F::NAN,
    e1234: F::NAN,
  };
}

impl From<Scalar> for EvenGrade {
  #[rustfmt::skip]
  #[inline]
  fn from(Scalar { s }: Scalar) -> Self {
    EvenGrade { s, ..Self::ZERO }
  }
}

impl From<Bivector> for EvenGrade {
  #[rustfmt::skip]
  #[inline]
  fn from(Bivector { e41, e42, e43, e23, e31, e12 }: Bivector) -> Self {
    EvenGrade { e41, e42, e43, e23, e31, e12, ..Self::ZERO }
  }
}

impl From<Antiscalar> for EvenGrade {
  #[rustfmt::skip]
  #[inline]
  fn from(Antiscalar { e1234 }: Antiscalar) -> Self {
    EvenGrade { e1234, ..Self::ZERO }
  }
}

impl From<DualNumber> for EvenGrade {
  #[rustfmt::skip]
  #[inline]
  fn from(DualNumber { s, e1234 }: DualNumber) -> Self {
    EvenGrade { s, e1234, ..Self::ZERO }
  }
}

impl fmt::Debug for EvenGrade {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    if fmt.alternate() {
      let width = fmt.width().unwrap_or(8);
      let precision = fmt.precision().unwrap_or(2);
      // pretty print
      fmt.write_fmt(format_args!(
        "EvenGrade {{\n\
        \x20scalar: {s:width$.precision$},\n\
        \x20   e41: {e41:width$.precision$},\
        \x20  e42: {e42:width$.precision$},\
        \x20  e43: {e43:width$.precision$},\
        \x20  e23: {e23:width$.precision$},\
        \x20  e31: {e31:width$.precision$},\
        \x20  e12: {e12:width$.precision$},\n\
        \x20 e1234: {e1234:width$.precision$},\n\
        }}",
        s = self.s,
        e41 = self.e41,
        e42 = self.e42,
        e43 = self.e43,
        e23 = self.e23,
        e31 = self.e31,
        e12 = self.e12,
        e1234 = self.e1234,
      ))
    } else {
      fmt
        .debug_struct("EvenGrade")
        .field("s", &self.s)
        .field("e41", &self.e41)
        .field("e42", &self.e42)
        .field("e43", &self.e43)
        .field("e23", &self.e23)
        .field("e31", &self.e31)
        .field("e12", &self.e12)
        .field("e1234", &self.e1234)
        .finish()
    }
  }
}
