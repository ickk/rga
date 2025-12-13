use crate::F;

/// s𝟏
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(transparent)]
pub struct Scalar {
  /// s𝟏
  pub s: F,
}

/// [`struct@Scalar`] constructor
#[allow(non_snake_case)]
pub const fn Scalar(s: F) -> Scalar {
  Scalar { s }
}

impl Scalar {
  /// 𝟎
  pub const ZERO: Self = Self { s: 0. };
  /// The unit scalar element, 𝟏
  pub const ONE: Self = Self { s: 1. };
  /// NaN
  pub const NAN: Self = Self { s: F::NAN };
}

impl From<F> for Scalar {
  #[inline]
  fn from(f: F) -> Scalar {
    Scalar { s: f }
  }
}

impl ::core::ops::Deref for Scalar {
  type Target = F;
  fn deref(&self) -> &Self::Target {
    &self.s
  }
}
impl ::core::ops::DerefMut for Scalar {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.s
  }
}
