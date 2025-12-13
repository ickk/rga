use {crate::F, ::core::fmt};

/// l<sub>vx</sub>𝐞₄₁ + l<sub>vy</sub>𝐞₄₂ + l<sub>vz</sub>𝐞₄₃ +
/// l<sub>mx</sub>𝐞₂₃ + l<sub>my</sub>𝐞₃₁ + l<sub>mz</sub>𝐞₁₂
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Bivector {
  /// l<sub>vx</sub> 𝐞₄₁ = l<sub>vx</sub> 𝐞₄∧𝐞₁
  pub e41: F,
  /// l<sub>vy</sub> 𝐞₄₂ = l<sub>vy</sub> 𝐞₄∧𝐞₂
  pub e42: F,
  /// l<sub>vz</sub> 𝐞₄₃ = l<sub>vz</sub> 𝐞₄∧𝐞₃
  pub e43: F,
  /// l<sub>mx</sub> 𝐞₂₃ = l<sub>mx</sub> 𝐞₂∧𝐞₃
  pub e23: F,
  /// l<sub>my</sub> 𝐞₃₁ = l<sub>my</sub> 𝐞₃∧𝐞₁
  pub e31: F,
  /// l<sub>mz</sub> 𝐞₁₂ = l<sub>mz</sub> 𝐞₁∧𝐞₂
  pub e12: F,
}

/// [`struct@Bivector`] constructor
#[allow(non_snake_case)]
pub const fn Bivector(
  e41: F,
  e42: F,
  e43: F,
  e23: F,
  e31: F,
  e12: F,
) -> Bivector {
  Bivector {
    e41,
    e42,
    e43,
    e23,
    e31,
    e12,
  }
}

impl Bivector {
  /// 𝟎
  pub const ZERO: Self = Self {
    e41: 0.,
    e42: 0.,
    e43: 0.,
    e23: 0.,
    e31: 0.,
    e12: 0.,
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
  /// All NaNs
  pub const NAN: Self = Self {
    e41: F::NAN,
    e42: F::NAN,
    e43: F::NAN,
    e23: F::NAN,
    e31: F::NAN,
    e12: F::NAN,
  };
}

impl fmt::Debug for Bivector {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    if fmt.alternate() {
      // pretty print
      let width = fmt.width().unwrap_or(8);
      let precision = fmt.precision().unwrap_or(2);
      fmt.write_fmt(format_args!(
        "Bivector {{\n\
        \x20 e41: {e41:width$.precision$},\
        \x20e42: {e42:width$.precision$},\
        \x20e43: {e43:width$.precision$},\
        \x20e23: {e23:width$.precision$},\
        \x20e31: {e31:width$.precision$},\
        \x20e12: {e12:width$.precision$},\n\
        }}",
        e41 = &self.e41,
        e42 = &self.e42,
        e43 = &self.e43,
        e23 = &self.e23,
        e31 = &self.e31,
        e12 = &self.e12,
      ))
    } else {
      fmt
        .debug_struct("Bivector")
        .field("e41", &self.e41)
        .field("e42", &self.e42)
        .field("e43", &self.e43)
        .field("e23", &self.e23)
        .field("e31", &self.e31)
        .field("e12", &self.e12)
        .finish()
    }
  }
}
