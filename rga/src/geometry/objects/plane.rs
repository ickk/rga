#[cfg(feature = "_math")]
use crate::algebra::operators::WeightNorm;
use {
  crate::{
    algebra::{
      operators::{Unitize, WeightNormSquared},
      values::{unit::impl_from_unit, Antiscalar, Trivector, Unit},
    },
    F,
  },
  ::core::ops::Mul,
};

/// A 3D plane represented by a normal and position
///
/// The normal & position are the 4 coordinates of a trivector in projective
/// 3-space. The position is proportional to the magnitude of the normal
/// trivector. A unitized plane has a normal with magnitude 1, and the position
/// is the signed distance to the origin.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Plane(pub Trivector);

impl Plane {
  /// A plane at the horizon
  pub const HORIZON: Self = Plane(Trivector::E321);

  /// Create a `Plane` from a normal and signed distance to the origin, where
  /// the distance is proportional to the magnitude of the normal.
  #[inline]
  pub const fn new(nx: F, ny: F, nz: F, d: F) -> Self {
    Plane(Trivector {
      e423: nx,
      e431: ny,
      e412: nz,
      e321: d,
    })
  }

  /// Create a `Unit<Plane>` from a normal and signed distance from the origin,
  /// unitizing the normal without scaling the distance.
  #[cfg(feature = "_math")]
  #[inline]
  pub fn new_unitize_normal(nx: F, ny: F, nz: F, d: F) -> Unit<Plane> {
    let mut p = Plane(Trivector {
      e423: nx,
      e431: ny,
      e412: nz,
      e321: 0.,
    })
    .unitize();
    p.0 .0.e431 = d;
    p
  }
}

impl Unit<Plane> {
  /// A unit plane at the horizon
  pub const HORIZON: Self = Unit::new_assume_unit(Plane::HORIZON);
}

impl From<(F, F, F, F)> for Plane {
  #[inline]
  fn from((nx, ny, nz, d): (F, F, F, F)) -> Self {
    Plane::new(nx, ny, nz, d)
  }
}

impl From<[F; 4]> for Plane {
  #[inline]
  fn from([nx, ny, nz, d]: [F; 4]) -> Self {
    Plane::new(nx, ny, nz, d)
  }
}

impl Unitize for Plane {}

impl_from_unit!(impl From<Unit<Plane>> for Plane);

impl WeightNormSquared for Plane {
  #[inline]
  fn weight_norm_squared(self) -> Antiscalar {
    self.0.weight_norm_squared()
  }
}
#[cfg(feature = "_math")]
impl WeightNorm for Plane {
  #[inline]
  fn weight_norm(self) -> Antiscalar {
    self.0.weight_norm()
  }
}

impl<Rhs> Mul<Rhs> for Plane
where
  Trivector: Mul<Rhs, Output = Trivector>,
{
  type Output = Self;
  #[inline]
  fn mul(self, rhs: Rhs) -> Self::Output {
    Self(self.0 * rhs)
  }
}
