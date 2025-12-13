use {
  crate::{
    algebra::values::{
      Antiscalar, Bivector, DualNumber, EvenGrade, OddGrade, Scalar,
      Trivector, Vector,
    },
    F,
  },
  ::core::fmt,
};

/// s +
/// p<sub>x</sub>𝐞₁ + p<sub>y</sub>𝐞₂ + p<sub>z</sub>𝐞₃ + p<sub>w</sub>𝐞₄ +
/// l<sub>vx</sub>𝐞₄₁ + l<sub>vy</sub>𝐞₄₂ + l<sub>vz</sub>𝐞₄₃ +
/// l<sub>mx</sub>𝐞₂₃ + l<sub>my</sub>𝐞₃₁ + l<sub>mz</sub>𝐞₁₂ +
/// g<sub>x</sub>𝐞₄₂₃ + g<sub>y</sub>𝐞₄₃₁ + g<sub>z</sub>𝐞₄₁₂ + g<sub>w</sub>𝐞₃₂₁ +
/// v𝐞₁₂₃₄
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Multivector {
  /// p<sub>x</sub> 𝐞₁
  pub e1: F,
  /// p<sub>y</sub> 𝐞₂
  pub e2: F,
  /// p<sub>z</sub> 𝐞₃
  pub e3: F,
  /// p<sub>w</sub> 𝐞₄
  pub e4: F,
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
  /// g<sub>x</sub> 𝐞₄₂₃ = g<sub>x</sub> 𝐞₄∧𝐞₂∧𝐞₃
  pub e423: F,
  /// g<sub>y</sub> 𝐞₄₃₁ = g<sub>y</sub> 𝐞₄∧𝐞₃∧𝐞₁
  pub e431: F,
  /// g<sub>z</sub> 𝐞₄₁₂ = g<sub>z</sub> 𝐞₄∧𝐞₁∧𝐞₂
  pub e412: F,
  /// g<sub>w</sub> 𝐞₃₂₁ = g<sub>w</sub> 𝐞₃∧𝐞₂∧𝐞₁
  pub e321: F,
}

impl Multivector {
  /// 𝟎
  pub const ZERO: Self = Self {
    s: 0.,
    e1: 0.,
    e2: 0.,
    e3: 0.,
    e4: 0.,
    e41: 0.,
    e42: 0.,
    e43: 0.,
    e23: 0.,
    e31: 0.,
    e12: 0.,
    e423: 0.,
    e431: 0.,
    e412: 0.,
    e321: 0.,
    e1234: 0.,
  };
  /// The unit scalar element, 𝟏
  pub const ONE: Self = Self {
    s: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₁ basis vector
  pub const E1: Self = Self {
    e1: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₂ basis vector
  pub const E2: Self = Self {
    e2: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₃ basis vector
  pub const E3: Self = Self {
    e3: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₄ basis vector
  ///
  /// where 𝐞₄ ≠ 0 and 𝐞₄² = 0
  ///
  /// The point at the origin with unit weight.
  pub const E4: Self = Self {
    e4: 1.,
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
  /// The unit 𝐞₄₂₃ = 𝐞₄∧𝐞₂∧𝐞₃ trivector
  pub const E423: Self = Self {
    e423: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₄₃₁ = 𝐞₄∧𝐞₃∧𝐞₁ trivector
  pub const E431: Self = Self {
    e431: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₄₁₂ = 𝐞₄∧𝐞₁∧𝐞₂ trivector
  pub const E412: Self = Self {
    e412: 1.,
    ..Self::ZERO
  };
  /// The unit 𝐞₃₂₁ = 𝐞₃∧𝐞₂∧𝐞₁ trivector
  ///
  /// The plane at the horizon.
  pub const E321: Self = Self {
    e321: 1.,
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
    e1: F::NAN,
    e2: F::NAN,
    e3: F::NAN,
    e4: F::NAN,
    e41: F::NAN,
    e42: F::NAN,
    e43: F::NAN,
    e23: F::NAN,
    e31: F::NAN,
    e12: F::NAN,
    e423: F::NAN,
    e431: F::NAN,
    e412: F::NAN,
    e321: F::NAN,
    e1234: F::NAN,
  };
}

impl From<Scalar> for Multivector {
  #[inline]
  fn from(Scalar { s }: Scalar) -> Self {
    Multivector { s, ..Self::ZERO }
  }
}

impl From<Vector> for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn from(Vector { e1, e2, e3, e4 }: Vector) -> Self {
    Multivector { e1, e2, e3, e4, ..Self::ZERO }
  }
}

impl From<Bivector> for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn from(Bivector { e41, e42, e43, e23, e31, e12 }: Bivector) -> Self {
    Multivector { e41, e42, e43, e23, e31, e12, ..Self::ZERO }
  }
}

impl From<Trivector> for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn from(Trivector { e423, e431, e412, e321 }: Trivector) -> Self {
    Multivector { e423, e431, e412, e321, ..Self::ZERO }
  }
}

impl From<Antiscalar> for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn from(Antiscalar { e1234 }: Antiscalar) -> Self {
    Multivector { e1234, ..Self::ZERO }
  }
}

impl From<DualNumber> for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn from(DualNumber { s, e1234 }: DualNumber) -> Self {
    Multivector { s, e1234, ..Self::ZERO }
  }
}

impl From<OddGrade> for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn from(
    OddGrade {
      e1, e2, e3, e4,
      e423, e431, e412, e321,
    }: OddGrade
  ) -> Self {
    Multivector {
      e1, e2, e3, e4,
      e423, e431, e412, e321,
      ..Self::ZERO
    }
  }
}

impl From<EvenGrade> for Multivector {
  #[rustfmt::skip]
  #[inline]
  fn from(
    EvenGrade {
      s,
      e41, e42, e43, e23, e31, e12,
      e1234,
    }: EvenGrade
  ) -> Self {
    Multivector {
      s,
      e41, e42, e43, e23, e31, e12,
      e1234,
      ..Self::ZERO
    }
  }
}

impl fmt::Debug for Multivector {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    if fmt.alternate() {
      let width = fmt.width().unwrap_or(8);
      let precision = fmt.precision().unwrap_or(2);
      // pretty print
      fmt.write_fmt(format_args!(
        "Multivector {{\n\
        \x20scalar: {s:width$.precision$},\n\
        \x20    e1: {e1:width$.precision$},\
        \x20   e2: {e2:width$.precision$},\
        \x20   e3: {e3:width$.precision$},\
        \x20   e4: {e4:width$.precision$},\n\
        \x20   e41: {e41:width$.precision$},\
        \x20  e42: {e42:width$.precision$},\
        \x20  e43: {e43:width$.precision$},\
        \x20  e23: {e23:width$.precision$},\
        \x20  e31: {e31:width$.precision$},\
        \x20  e12: {e12:width$.precision$},\n\
        \x20  e423: {e423:width$.precision$},\
        \x20 e431: {e431:width$.precision$},\
        \x20 e412: {e412:width$.precision$},\
        \x20 e321: {e321:width$.precision$},\n\
        \x20 e1234: {e1234:width$.precision$},\n\
        }}",
        s = self.s,
        e1 = self.e1,
        e2 = self.e2,
        e3 = self.e3,
        e4 = self.e4,
        e41 = self.e41,
        e42 = self.e42,
        e43 = self.e43,
        e23 = self.e23,
        e31 = self.e31,
        e12 = self.e12,
        e423 = self.e423,
        e431 = self.e431,
        e412 = self.e412,
        e321 = self.e321,
        e1234 = self.e1234,
      ))
    } else {
      fmt
        .debug_struct("Multivector")
        .field("s", &self.s)
        .field("e1", &self.e1)
        .field("e2", &self.e2)
        .field("e3", &self.e3)
        .field("e4", &self.e4)
        .field("e41", &self.e41)
        .field("e42", &self.e42)
        .field("e43", &self.e43)
        .field("e23", &self.e23)
        .field("e31", &self.e31)
        .field("e12", &self.e12)
        .field("e423", &self.e423)
        .field("e431", &self.e431)
        .field("e412", &self.e412)
        .field("e321", &self.e321)
        .field("e1234", &self.e1234)
        .finish()
    }
  }
}
