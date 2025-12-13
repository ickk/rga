use {
  crate::algebra::values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Unit, Vector, Zero,
  },
  ::core::cmp::PartialEq,
};

macro_rules! impl_partial_eq_when_zero {
  ($($lhs:ty, $rhs:ty;)*) => {
    $(impl PartialEq<$rhs> for $lhs {
      #[inline]
      fn eq(&self, rhs: &$rhs) -> bool {
        *self == <$lhs as Zero>::zero()
        && *rhs == <$rhs as Zero>::zero()
      }
    })*
  };
}

impl_partial_eq_when_zero! {
  Scalar, Vector;
  Scalar, Bivector;
  Scalar, Trivector;
  Scalar, Antiscalar;
  Scalar, OddGrade;

  Vector, Scalar;
  Vector, Bivector;
  Vector, Trivector;
  Vector, Antiscalar;
  Vector, DualNumber;
  Vector, EvenGrade;

  Bivector, Scalar;
  Bivector, Vector;
  Bivector, Trivector;
  Bivector, Antiscalar;
  Bivector, DualNumber;
  Bivector, OddGrade;

  Trivector, Scalar;
  Trivector, Vector;
  Trivector, Bivector;
  Trivector, Antiscalar;
  Trivector, DualNumber;
  Trivector, EvenGrade;

  Antiscalar, Scalar;
  Antiscalar, Vector;
  Antiscalar, Bivector;
  Antiscalar, Trivector;
  Antiscalar, OddGrade;

  DualNumber, Vector;
  DualNumber, Bivector;
  DualNumber, Trivector;
  DualNumber, OddGrade;

  OddGrade, Scalar;
  OddGrade, Bivector;
  OddGrade, Antiscalar;
  OddGrade, DualNumber;
  OddGrade, EvenGrade;

  EvenGrade, Vector;
  EvenGrade, Trivector;
  EvenGrade, OddGrade;
}

macro_rules! impl_partial_eq_rhs_into_lhs {
  ($($a:ty, $b:ty;)*) => {
    $(
      impl PartialEq<$a> for $b {
        #[inline]
        fn eq(&self, rhs: &$a) -> bool {
          let lhs = <$a>::from(*self);
          lhs == *rhs
        }
      }
      impl PartialEq<$b> for $a {
        #[inline]
        fn eq(&self, rhs: &$b) -> bool {
          let rhs = <$a>::from(*rhs);
          *self == rhs
        }
      }
    )*
  };
}

impl_partial_eq_rhs_into_lhs! {
  Multivector, Scalar;
  Multivector, Vector;
  Multivector, Bivector;
  Multivector, Trivector;
  Multivector, Antiscalar;
  Multivector, DualNumber;
  Multivector, OddGrade;
  Multivector, EvenGrade;

  DualNumber, Scalar;
  DualNumber, Antiscalar;

  OddGrade, Vector;
  OddGrade, Trivector;

  EvenGrade, Scalar;
  EvenGrade, Bivector;
  EvenGrade, Antiscalar;
  EvenGrade, DualNumber;
}

macro_rules! impl_partial_eq_unit {
  ($(Unit<$lhs:ty>, Unit<$rhs:ty>;)*) => {$(
    impl PartialEq<Unit<$rhs>> for Unit<$lhs> {
      #[inline]
      fn eq(&self, rhs: &Unit<$rhs>) -> bool {
        self.inner() == *rhs
      }
    }
  )*};
  ($(Unit<$lhs:ty>, $rhs:ty;)*) => {$(
    impl PartialEq<$rhs> for Unit<$lhs> {
      #[inline]
      fn eq(&self, rhs: &$rhs) -> bool {
        &self.inner() == rhs
      }
    }
  )*};
  ($($lhs:ty, Unit<$rhs:ty>;)*) => {$(
    impl PartialEq<Unit<$rhs>> for $lhs {
      #[inline]
      fn eq(&self, rhs: &Unit<$rhs>) -> bool {
        self == &rhs.inner()
      }
    }
  )*};
}

impl_partial_eq_unit!(
  Multivector, Unit<Multivector>;
  Multivector, Unit<Vector>;
  Multivector, Unit<Bivector>;
  Multivector, Unit<Trivector>;
  Multivector, Unit<Antiscalar>;
  Multivector, Unit<DualNumber>;
  Multivector, Unit<OddGrade>;
  Multivector, Unit<EvenGrade>;

  Scalar, Unit<Multivector>;
  Scalar, Unit<Vector>;
  Scalar, Unit<Bivector>;
  Scalar, Unit<Trivector>;
  Scalar, Unit<Antiscalar>;
  Scalar, Unit<DualNumber>;
  Scalar, Unit<OddGrade>;
  Scalar, Unit<EvenGrade>;

  Vector, Unit<Multivector>;
  Vector, Unit<Vector>;
  Vector, Unit<Bivector>;
  Vector, Unit<Trivector>;
  Vector, Unit<Antiscalar>;
  Vector, Unit<DualNumber>;
  Vector, Unit<OddGrade>;
  Vector, Unit<EvenGrade>;

  Bivector, Unit<Multivector>;
  Bivector, Unit<Vector>;
  Bivector, Unit<Bivector>;
  Bivector, Unit<Trivector>;
  Bivector, Unit<Antiscalar>;
  Bivector, Unit<DualNumber>;
  Bivector, Unit<OddGrade>;
  Bivector, Unit<EvenGrade>;

  Trivector, Unit<Multivector>;
  Trivector, Unit<Vector>;
  Trivector, Unit<Bivector>;
  Trivector, Unit<Trivector>;
  Trivector, Unit<Antiscalar>;
  Trivector, Unit<DualNumber>;
  Trivector, Unit<OddGrade>;
  Trivector, Unit<EvenGrade>;

  Antiscalar, Unit<Multivector>;
  Antiscalar, Unit<Vector>;
  Antiscalar, Unit<Bivector>;
  Antiscalar, Unit<Trivector>;
  Antiscalar, Unit<Antiscalar>;
  Antiscalar, Unit<DualNumber>;
  Antiscalar, Unit<OddGrade>;
  Antiscalar, Unit<EvenGrade>;

  DualNumber, Unit<Multivector>;
  DualNumber, Unit<Vector>;
  DualNumber, Unit<Bivector>;
  DualNumber, Unit<Trivector>;
  DualNumber, Unit<Antiscalar>;
  DualNumber, Unit<DualNumber>;
  DualNumber, Unit<OddGrade>;
  DualNumber, Unit<EvenGrade>;

  OddGrade, Unit<Multivector>;
  OddGrade, Unit<Vector>;
  OddGrade, Unit<Bivector>;
  OddGrade, Unit<Trivector>;
  OddGrade, Unit<Antiscalar>;
  OddGrade, Unit<DualNumber>;
  OddGrade, Unit<OddGrade>;
  OddGrade, Unit<EvenGrade>;

  EvenGrade, Unit<Multivector>;
  EvenGrade, Unit<Vector>;
  EvenGrade, Unit<Bivector>;
  EvenGrade, Unit<Trivector>;
  EvenGrade, Unit<Antiscalar>;
  EvenGrade, Unit<DualNumber>;
  EvenGrade, Unit<OddGrade>;
  EvenGrade, Unit<EvenGrade>;
);

impl_partial_eq_unit!(
  Unit<Multivector>, Scalar;
  Unit<Multivector>, Vector;
  Unit<Multivector>, Bivector;
  Unit<Multivector>, Trivector;
  Unit<Multivector>, Antiscalar;
  Unit<Multivector>, DualNumber;
  Unit<Multivector>, OddGrade;
  Unit<Multivector>, EvenGrade;

  Unit<Vector>, Multivector;
  Unit<Vector>, Scalar;
  Unit<Vector>, Vector;
  Unit<Vector>, Bivector;
  Unit<Vector>, Trivector;
  Unit<Vector>, Antiscalar;
  Unit<Vector>, DualNumber;
  Unit<Vector>, OddGrade;
  Unit<Vector>, EvenGrade;

  Unit<Bivector>, Multivector;
  Unit<Bivector>, Scalar;
  Unit<Bivector>, Vector;
  Unit<Bivector>, Bivector;
  Unit<Bivector>, Trivector;
  Unit<Bivector>, Antiscalar;
  Unit<Bivector>, DualNumber;
  Unit<Bivector>, OddGrade;
  Unit<Bivector>, EvenGrade;

  Unit<Trivector>, Multivector;
  Unit<Trivector>, Scalar;
  Unit<Trivector>, Vector;
  Unit<Trivector>, Bivector;
  Unit<Trivector>, Trivector;
  Unit<Trivector>, Antiscalar;
  Unit<Trivector>, DualNumber;
  Unit<Trivector>, OddGrade;
  Unit<Trivector>, EvenGrade;

  Unit<Antiscalar>, Multivector;
  Unit<Antiscalar>, Scalar;
  Unit<Antiscalar>, Vector;
  Unit<Antiscalar>, Bivector;
  Unit<Antiscalar>, Trivector;
  Unit<Antiscalar>, Antiscalar;
  Unit<Antiscalar>, DualNumber;
  Unit<Antiscalar>, OddGrade;
  Unit<Antiscalar>, EvenGrade;

  Unit<DualNumber>, Multivector;
  Unit<DualNumber>, Scalar;
  Unit<DualNumber>, Vector;
  Unit<DualNumber>, Bivector;
  Unit<DualNumber>, Trivector;
  Unit<DualNumber>, Antiscalar;
  Unit<DualNumber>, DualNumber;
  Unit<DualNumber>, OddGrade;
  Unit<DualNumber>, EvenGrade;

  Unit<OddGrade>, Multivector;
  Unit<OddGrade>, Scalar;
  Unit<OddGrade>, Vector;
  Unit<OddGrade>, Bivector;
  Unit<OddGrade>, Trivector;
  Unit<OddGrade>, Antiscalar;
  Unit<OddGrade>, DualNumber;
  Unit<OddGrade>, OddGrade;
  Unit<OddGrade>, EvenGrade;

  Unit<EvenGrade>, Multivector;
  Unit<EvenGrade>, Scalar;
  Unit<EvenGrade>, Vector;
  Unit<EvenGrade>, Bivector;
  Unit<EvenGrade>, Trivector;
  Unit<EvenGrade>, Antiscalar;
  Unit<EvenGrade>, DualNumber;
  Unit<EvenGrade>, OddGrade;
  Unit<EvenGrade>, EvenGrade;
);

impl_partial_eq_unit!(
  Unit<Multivector>, Unit<Vector>;
  Unit<Multivector>, Unit<Bivector>;
  Unit<Multivector>, Unit<Trivector>;
  Unit<Multivector>, Unit<Antiscalar>;
  Unit<Multivector>, Unit<DualNumber>;
  Unit<Multivector>, Unit<OddGrade>;
  Unit<Multivector>, Unit<EvenGrade>;

  Unit<Vector>, Unit<Multivector>;
  Unit<Vector>, Unit<Bivector>;
  Unit<Vector>, Unit<Trivector>;
  Unit<Vector>, Unit<Antiscalar>;
  Unit<Vector>, Unit<DualNumber>;
  Unit<Vector>, Unit<OddGrade>;
  Unit<Vector>, Unit<EvenGrade>;

  Unit<Bivector>, Unit<Multivector>;
  Unit<Bivector>, Unit<Vector>;
  Unit<Bivector>, Unit<Trivector>;
  Unit<Bivector>, Unit<Antiscalar>;
  Unit<Bivector>, Unit<DualNumber>;
  Unit<Bivector>, Unit<OddGrade>;
  Unit<Bivector>, Unit<EvenGrade>;

  Unit<Trivector>, Unit<Multivector>;
  Unit<Trivector>, Unit<Vector>;
  Unit<Trivector>, Unit<Bivector>;
  Unit<Trivector>, Unit<Antiscalar>;
  Unit<Trivector>, Unit<DualNumber>;
  Unit<Trivector>, Unit<OddGrade>;
  Unit<Trivector>, Unit<EvenGrade>;

  Unit<Antiscalar>, Unit<Multivector>;
  Unit<Antiscalar>, Unit<Vector>;
  Unit<Antiscalar>, Unit<Bivector>;
  Unit<Antiscalar>, Unit<Trivector>;
  Unit<Antiscalar>, Unit<DualNumber>;
  Unit<Antiscalar>, Unit<OddGrade>;
  Unit<Antiscalar>, Unit<EvenGrade>;

  Unit<DualNumber>, Unit<Multivector>;
  Unit<DualNumber>, Unit<Vector>;
  Unit<DualNumber>, Unit<Bivector>;
  Unit<DualNumber>, Unit<Trivector>;
  Unit<DualNumber>, Unit<Antiscalar>;
  Unit<DualNumber>, Unit<OddGrade>;
  Unit<DualNumber>, Unit<EvenGrade>;

  Unit<OddGrade>, Unit<Multivector>;
  Unit<OddGrade>, Unit<Vector>;
  Unit<OddGrade>, Unit<Bivector>;
  Unit<OddGrade>, Unit<Trivector>;
  Unit<OddGrade>, Unit<Antiscalar>;
  Unit<OddGrade>, Unit<DualNumber>;
  Unit<OddGrade>, Unit<EvenGrade>;

  Unit<EvenGrade>, Unit<Multivector>;
  Unit<EvenGrade>, Unit<Vector>;
  Unit<EvenGrade>, Unit<Bivector>;
  Unit<EvenGrade>, Unit<Trivector>;
  Unit<EvenGrade>, Unit<Antiscalar>;
  Unit<EvenGrade>, Unit<DualNumber>;
  Unit<EvenGrade>, Unit<OddGrade>;
);
