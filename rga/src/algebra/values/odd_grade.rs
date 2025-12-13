use {
  crate::{
    algebra::values::{Trivector, Vector},
    F,
  },
  ::core::fmt,
};

/// p<sub>x</sub>𝐞₁ + p<sub>y</sub>𝐞₂ + p<sub>z</sub>𝐞₃ + p<sub>w</sub>𝐞₄ +
/// g<sub>x</sub>𝐞₄₂₃ + g<sub>y</sub>𝐞₄₃₁ + g<sub>z</sub>𝐞₄₁₂ + g<sub>w</sub>𝐞₃₂₁
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct OddGrade {
  /// p<sub>x</sub> 𝐞₁
  pub e1: F,
  /// p<sub>y</sub> 𝐞₂
  pub e2: F,
  /// p<sub>z</sub> 𝐞₃
  pub e3: F,
  /// p<sub>w</sub> 𝐞₄
  pub e4: F,
  /// g<sub>x</sub> 𝐞₄₂₃ = g<sub>x</sub> 𝐞₄∧𝐞₂∧𝐞₃
  pub e423: F,
  /// g<sub>y</sub> 𝐞₄₃₁ = g<sub>y</sub> 𝐞₄∧𝐞₃∧𝐞₁
  pub e431: F,
  /// g<sub>z</sub> 𝐞₄₁₂ = g<sub>z</sub> 𝐞₄∧𝐞₁∧𝐞₂
  pub e412: F,
  /// g<sub>w</sub> 𝐞₃₂₁ = g<sub>w</sub> 𝐞₃∧𝐞₂∧𝐞₁
  pub e321: F,
}

impl OddGrade {
  /// 𝟎
  pub const ZERO: Self = Self {
    e1: 0.,
    e2: 0.,
    e3: 0.,
    e4: 0.,
    e423: 0.,
    e431: 0.,
    e412: 0.,
    e321: 0.,
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
  /// All NaNs
  pub const NAN: Self = Self {
    e1: F::NAN,
    e2: F::NAN,
    e3: F::NAN,
    e4: F::NAN,
    e423: F::NAN,
    e431: F::NAN,
    e412: F::NAN,
    e321: F::NAN,
  };
}

impl From<Vector> for OddGrade {
  #[rustfmt::skip]
  #[inline]
  fn from(Vector { e1, e2, e3, e4 }: Vector) -> Self {
    OddGrade { e1, e2, e3, e4, ..Self::ZERO }
  }
}

impl From<Trivector> for OddGrade {
  #[rustfmt::skip]
  #[inline]
  fn from(Trivector { e423, e431, e412, e321 }: Trivector) -> Self {
    OddGrade { e423, e431, e412, e321, ..Self::ZERO }
  }
}

impl fmt::Debug for OddGrade {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    if fmt.alternate() {
      let width = fmt.width().unwrap_or(8);
      let precision = fmt.precision().unwrap_or(2);
      // pretty print
      fmt.write_fmt(format_args!(
        "OddGrade {{\n\
        \x20  e1: {e1:width$.precision$},\
        \x20  e2: {e2:width$.precision$},\
        \x20  e3: {e3:width$.precision$},\
        \x20  e4: {e4:width$.precision$},\n\
        \x20e423: {e423:width$.precision$},\
        \x20e431: {e431:width$.precision$},\
        \x20e412: {e412:width$.precision$},\
        \x20e321: {e321:width$.precision$},\n\
        }}",
        e1 = self.e1,
        e2 = self.e2,
        e3 = self.e3,
        e4 = self.e4,
        e423 = self.e423,
        e431 = self.e431,
        e412 = self.e412,
        e321 = self.e321,
      ))
    } else {
      fmt
        .debug_struct("OddGrade")
        .field("e1", &self.e1)
        .field("e2", &self.e2)
        .field("e3", &self.e3)
        .field("e4", &self.e4)
        .field("e423", &self.e423)
        .field("e431", &self.e431)
        .field("e412", &self.e412)
        .field("e321", &self.e321)
        .finish()
    }
  }
}
