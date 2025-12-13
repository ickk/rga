use {crate::F, ::core::fmt};

/// p<sub>x</sub>𝐞₁ + p<sub>y</sub>𝐞₂ + p<sub>z</sub>𝐞₃ + p<sub>w</sub>𝐞₄
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Vector {
  /// p<sub>x</sub> 𝐞₁
  pub e1: F,
  /// p<sub>y</sub> 𝐞₂
  pub e2: F,
  /// p<sub>z</sub> 𝐞₃
  pub e3: F,
  /// p<sub>w</sub> 𝐞₄
  pub e4: F,
}

/// [`struct@Vector`] constructor
#[allow(non_snake_case)]
pub const fn Vector(e1: F, e2: F, e3: F, e4: F) -> Vector {
  Vector { e1, e2, e3, e4 }
}

impl Vector {
  /// 𝟎
  pub const ZERO: Self = Self {
    e1: 0.,
    e2: 0.,
    e3: 0.,
    e4: 0.,
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
  /// All NaNs
  pub const NAN: Self = Self {
    e1: F::NAN,
    e2: F::NAN,
    e3: F::NAN,
    e4: F::NAN,
  };
}

impl fmt::Debug for Vector {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    let width = fmt.width().unwrap_or(8);
    let precision = fmt.precision().unwrap_or(2);
    if fmt.alternate() {
      // pretty print
      fmt.write_fmt(format_args!(
        "Vector {{\n\
        \x20 e1: {e1:width$.precision$},\
        \x20e2: {e2:width$.precision$},\
        \x20e3: {e3:width$.precision$},\
        \x20e4: {e4:width$.precision$},\n\
        }}",
        e1 = &self.e1,
        e2 = &self.e2,
        e3 = &self.e3,
        e4 = &self.e4,
      ))
    } else {
      fmt
        .debug_struct("Vector")
        .field("e1", &self.e1)
        .field("e2", &self.e2)
        .field("e3", &self.e3)
        .field("e4", &self.e4)
        .finish()
    }
  }
}
