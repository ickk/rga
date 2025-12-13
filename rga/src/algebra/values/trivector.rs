use {crate::F, ::core::fmt};

/// g<sub>x</sub>𝐞₄₂₃ + g<sub>y</sub>𝐞₄₃₁ + g<sub>z</sub>𝐞₄₁₂ + g<sub>w</sub>𝐞₃₂₁
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Trivector {
  /// g<sub>x</sub> 𝐞₄₂₃ = g<sub>x</sub> 𝐞₄∧𝐞₂∧𝐞₃
  pub e423: F,
  /// g<sub>y</sub> 𝐞₄₃₁ = g<sub>y</sub> 𝐞₄∧𝐞₃∧𝐞₁
  pub e431: F,
  /// g<sub>z</sub> 𝐞₄₁₂ = g<sub>z</sub> 𝐞₄∧𝐞₁∧𝐞₂
  pub e412: F,
  /// g<sub>w</sub> 𝐞₃₂₁ = g<sub>w</sub> 𝐞₃∧𝐞₂∧𝐞₁
  pub e321: F,
}

/// [`struct@Trivector`] constructor
#[allow(non_snake_case)]
pub const fn Trivector(e423: F, e431: F, e412: F, e321: F) -> Trivector {
  Trivector {
    e423,
    e431,
    e412,
    e321,
  }
}

impl Trivector {
  /// 𝟎
  pub const ZERO: Self = Self {
    e423: 0.,
    e431: 0.,
    e412: 0.,
    e321: 0.,
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
    e423: F::NAN,
    e431: F::NAN,
    e412: F::NAN,
    e321: F::NAN,
  };
}

impl fmt::Debug for Trivector {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    let width = fmt.width().unwrap_or(8);
    let precision = fmt.precision().unwrap_or(2);
    if fmt.alternate() {
      // pretty print
      fmt.write_fmt(format_args!(
        "Trivector {{\n\
        \x20 e423: {e423:width$.precision$},\
        \x20e431: {e431:width$.precision$},\
        \x20e412: {e412:width$.precision$},\
        \x20e321: {e321:width$.precision$},\n\
        }}",
        e423 = &self.e423,
        e431 = &self.e431,
        e412 = &self.e412,
        e321 = &self.e321,
      ))
    } else {
      fmt
        .debug_struct("Trivector")
        .field("e423", &self.e423)
        .field("e431", &self.e431)
        .field("e412", &self.e412)
        .field("e321", &self.e321)
        .finish()
    }
  }
}
