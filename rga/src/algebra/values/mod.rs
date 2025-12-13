//! Collection of number structures; `Vector`, `Bivector`, `DualNumber`,
//! `Matrix4`, ..

mod antiscalar;
mod bivector;
mod dual_number;
mod even_grade;
mod matrix4;
mod multivector;
mod odd_grade;
mod scalar;
mod trivector;
mod vector;

pub use self::{
  antiscalar::Antiscalar,
  bivector::Bivector,
  dual_number::DualNumber,
  even_grade::EvenGrade,
  matrix4::Matrix4,
  multivector::Multivector,
  odd_grade::OddGrade,
  scalar::Scalar,
  trivector::Trivector,
  unit::Unit,
  vector::Vector,
  zero::{zero, Zero},
};

mod zero {
  use super::*;

  /// 𝟎
  #[inline]
  pub fn zero<T: Zero>() -> T {
    <T as Zero>::zero()
  }

  /// 𝟎
  pub trait Zero {
    /// 𝟎
    fn zero() -> Self;
  }

  macro_rules! impl_zero {
    ($($struct:ty;)*) => {
      $(impl Zero for $struct {
        #[inline]
        fn zero() -> Self {
          Self::ZERO
        }
      })*
    };
  }

  impl_zero! {
    Multivector;
    Scalar;
    Vector;
    Bivector;
    Trivector;
    Antiscalar;
    DualNumber;
    OddGrade;
    EvenGrade;
    Matrix4;
  }
}

pub(crate) mod unit {
  use crate::algebra::{
    operators::{Unitize, WeightNorm},
    values::*,
  };

  /// û
  ///
  /// A weight-normalised value, where ||u||<sub>○</sub> = 𝟙.
  ///
  /// See also: [`WeightNorm`], [`Unitize`]
  #[derive(Clone, Copy, PartialEq, Debug)]
  pub struct Unit<T>(pub(crate) T);

  impl<T> Unit<T>
  where
    T: Unitize,
  {
    /// û
    ///
    /// Weight Normalisation.
    ///
    /// Scale the value so that the weight is normalised to have a unit magnitude
    /// of 𝟙.
    #[inline]
    pub fn new_unitize(value: T) -> Self
    where
      T: WeightNorm,
    {
      Unitize::unitize(value)
    }

    /// Wraps the value in `Unit` without performing unitization
    #[inline(always)]
    pub const fn new_assume_unit(value: T) -> Self {
      Unit(value)
    }

    /// Reunitize the value
    ///
    /// This may be needed if the inner value becomes denormalised due to
    /// numerical imprecision in repeated computations.
    ///
    /// See also [`Self::reunitize_fast`].
    #[inline]
    pub fn reunitize(&mut self)
    where
      T: WeightNorm,
    {
      Unitize::reunitize(self);
    }

    /// Perform a fast approximate reunitization using a single step of Newton's
    /// method
    ///
    /// This can be a relatively cheap way to keep values unitized when they
    /// are subject to denormalisation due to numerical imprecision in repeated
    /// computations.
    #[inline]
    pub fn reunitize_fast(&mut self) {
      Unitize::reunitize_fast(self);
    }
  }

  impl<T> Unit<T> {
    /// Get the underlying value
    #[inline(always)]
    pub fn inner(self) -> T {
      self.0
    }
  }

  impl<T> ::core::ops::Deref for Unit<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }

  impl<T> ::core::borrow::Borrow<T> for Unit<T> {
    #[inline]
    fn borrow(&self) -> &T {
      &self.0
    }
  }

  macro_rules! impl_from_unit (
    ($(impl From<Unit<$unit:ty>> for $ty:ty);*$(;)?) => {$(
      impl From<Unit<$unit>> for $ty {
        #[inline]
        fn from(value: Unit<$unit>) -> Self {
          Self::from(value.inner())
        }
      }
    )*};
  );
  pub(crate) use impl_from_unit;

  impl_from_unit!(
    impl From<Unit<Multivector>> for Multivector;
    impl From<Unit<Vector>> for Vector;
    impl From<Unit<Bivector>> for Bivector;
    impl From<Unit<Trivector>> for Trivector;
    impl From<Unit<Antiscalar>> for Antiscalar;
    impl From<Unit<DualNumber>> for DualNumber;
    impl From<Unit<OddGrade>> for OddGrade;
    impl From<Unit<EvenGrade>> for EvenGrade;

    impl From<Unit<Vector>> for Multivector;
    impl From<Unit<Bivector>> for Multivector;
    impl From<Unit<Trivector>> for Multivector;
    impl From<Unit<Antiscalar>> for Multivector;
    impl From<Unit<DualNumber>> for Multivector;
    impl From<Unit<OddGrade>> for Multivector;
    impl From<Unit<EvenGrade>> for Multivector;

    impl From<Unit<Antiscalar>> for DualNumber;

    impl From<Unit<Vector>> for OddGrade;
    impl From<Unit<Trivector>> for OddGrade;

    impl From<Unit<Bivector>> for EvenGrade;
    impl From<Unit<Antiscalar>> for EvenGrade;
    impl From<Unit<DualNumber>> for EvenGrade;
  );
}
