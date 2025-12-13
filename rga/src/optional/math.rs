/// Math backend for primitive operators such as floating point `sqrt`.
///
/// Automatically enabled when either the `std` or `libm` features are enabled,
/// preferring the `libm` backend when both are enabled.
pub(crate) trait Math {
  #[allow(unused)]
  fn backend_id() -> BackendId;

  fn sqrt(self) -> Self;
}

#[allow(unused)]
pub(crate) enum BackendId {
  Std,
  Libm,
}

#[cfg(feature = "libm")]
impl Math for crate::F {
  fn backend_id() -> BackendId {
    BackendId::Libm
  }

  #[inline]
  fn sqrt(self) -> Self {
    ::libm::Libm::<crate::F>::sqrt(self)
  }
}

#[cfg(all(feature = "std", not(feature = "libm")))]
impl Math for crate::F {
  fn backend_id() -> BackendId {
    BackendId::Std
  }

  #[inline]
  fn sqrt(self) -> Self {
    crate::F::sqrt(self)
  }
}

#[cfg(test)]
mod tests {
  #[cfg(feature = "libm")]
  #[test]
  fn ensure_libm_backend_preferred() {
    use super::{BackendId, Math};
    assert!(matches!(<crate::F as Math>::backend_id(), BackendId::Libm));
  }
}
