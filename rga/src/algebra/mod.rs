//! Algebra over the Cl<sub>3,0,1</sub> 4D projective space corresponding to
//! the 3D euclidian vector space

pub mod operators;
pub mod values;

#[doc(inline)]
pub use self::{
  operators::{functions::*, traits::*},
  values::*,
};
