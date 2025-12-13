use {
  crate::{
    algebra::values::{Antiscalar, Scalar},
    F,
  },
  ::core::fmt,
};

/// s + v𝐞₁₂₃₄
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct DualNumber {
  /// s
  pub s: F,
  /// v 𝟙 = v 𝐞₁₂₃₄ = v 𝐞₁∧𝐞₂∧𝐞₃∧𝐞₄
  pub e1234: F,
}

/// [`struct@DualNumber`] constructor
#[allow(non_snake_case)]
pub const fn DualNumber(s: F, e1234: F) -> DualNumber {
  DualNumber { s, e1234 }
}

impl DualNumber {
  /// 𝟎
  pub const ZERO: Self = Self { s: 0., e1234: 0. };
  /// The unit scalar element, 𝟏
  pub const ONE: Self = Self { s: 1., e1234: 0. };
  /// The unit volume element, 𝟙 = 1𝐞₁₂₃₄
  pub const E1234: Self = Self { s: 0., e1234: 1. };
  /// All NaNs
  pub const NAN: Self = Self {
    s: F::NAN,
    e1234: F::NAN,
  };
}

impl From<Scalar> for DualNumber {
  #[inline]
  fn from(Scalar { s }: Scalar) -> Self {
    DualNumber { s, ..Self::ZERO }
  }
}

impl From<Antiscalar> for DualNumber {
  #[rustfmt::skip]
  #[inline]
  fn from(Antiscalar { e1234 }: Antiscalar) -> Self {
    DualNumber { e1234, ..Self::ZERO }
  }
}

impl fmt::Debug for DualNumber {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    if fmt.alternate() {
      // pretty print
      let width = fmt.width().unwrap_or(8);
      let precision = fmt.precision().unwrap_or(2);
      fmt.write_fmt(format_args!(
        "DualNumber {{\n\
        \x20 s: {s:width$.precision$},\n\
        \x20 e1234: {e1234:width$.precision$},\n\
        }}",
        s = &self.s,
        e1234 = &self.e1234,
      ))
    } else {
      fmt
        .debug_struct("DualNumber")
        .field("s", &self.s)
        .field("e1234", &self.e1234)
        .finish()
    }
  }
}
