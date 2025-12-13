#![cfg_attr(not(any(feature = "std", test, doctest)), no_std)]
#![deny(unsafe_code)]

pub mod algebra;
pub mod geometry;
mod optional;

pub(crate) type F = f64;
mod helpers;
#[cfg(test)]
mod test_values;

#[doc(inline)]
pub use crate::{
  algebra::{
    operators::{functions::*, traits::*},
    values::*,
  },
  geometry::{
    objects::*,
    operators::{functions::*, traits::*},
    transformations::*,
  },
};
