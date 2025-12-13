//! Optional features enabled by corresponding cargo-feature

#[cfg(feature = "_math")]
pub(crate) mod math;

#[cfg(any(feature = "approx", test))]
mod approx;
