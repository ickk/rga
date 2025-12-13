use crate::{
  algebra::operators::GeometricAntiproduct,
  geometry::transformations::{Flector, Inversion, Motor, Reflection},
};

/// Compose transformations
#[inline]
pub fn compose<Lhs, Rhs>(a: Lhs, b: Rhs) -> <Lhs as Compose<Rhs>>::Output
where
  Lhs: Compose<Rhs>,
{
  a.compose(b)
}

/// Compose transformations
pub trait Compose<Transform> {
  type Output;

  /// Compose transformations
  fn compose(self, rhs: Transform) -> Self::Output;
}

impl Compose<Reflection> for Reflection {
  type Output = Motor;

  fn compose(self, rhs: Reflection) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Inversion> for Reflection {
  type Output = Motor;

  fn compose(self, rhs: Inversion) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Motor> for Reflection {
  type Output = Flector;

  fn compose(self, rhs: Motor) -> Self::Output {
    Flector(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Flector> for Reflection {
  type Output = Motor;

  fn compose(self, rhs: Flector) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Reflection> for Inversion {
  type Output = Motor;

  fn compose(self, rhs: Reflection) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Inversion> for Inversion {
  type Output = Motor; // translation

  fn compose(self, rhs: Inversion) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Motor> for Inversion {
  type Output = Flector;

  fn compose(self, rhs: Motor) -> Self::Output {
    Flector(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Flector> for Inversion {
  type Output = Motor;

  fn compose(self, rhs: Flector) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Reflection> for Motor {
  type Output = Flector;

  fn compose(self, rhs: Reflection) -> Self::Output {
    Flector(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Inversion> for Motor {
  type Output = Flector;

  fn compose(self, rhs: Inversion) -> Self::Output {
    Flector(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Motor> for Motor {
  type Output = Motor;

  fn compose(self, rhs: Motor) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Flector> for Motor {
  type Output = Flector;

  fn compose(self, rhs: Flector) -> Self::Output {
    Flector(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Reflection> for Flector {
  type Output = Motor;

  fn compose(self, rhs: Reflection) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Inversion> for Flector {
  type Output = Motor;

  fn compose(self, rhs: Inversion) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Motor> for Flector {
  type Output = Flector;

  fn compose(self, rhs: Motor) -> Self::Output {
    Flector(self.0.geometric_antiproduct(rhs.0))
  }
}

impl Compose<Flector> for Flector {
  type Output = Motor;

  fn compose(self, rhs: Flector) -> Self::Output {
    Motor(self.0.geometric_antiproduct(rhs.0))
  }
}
