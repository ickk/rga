use crate::F;

/// v𝐞₁₂₃₄
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Antiscalar {
  /// v 𝟙 = a 𝐞₁₂₃₄ = a 𝐞₁∧𝐞₂∧𝐞₃∧𝐞₄
  pub e1234: F,
}

/// [`struct@Antiscalar`] constructor
#[allow(non_snake_case)]
pub const fn Antiscalar(e1234: F) -> Antiscalar {
  Antiscalar { e1234 }
}

impl Antiscalar {
  /// 𝟎
  pub const ZERO: Self = Self { e1234: 0. };
  /// The unit volume element, 𝟙 = 1𝐞₁₂₃₄
  pub const E1234: Self = Self { e1234: 1. };
  /// NaN
  pub const NAN: Self = Self { e1234: F::NAN };
}
