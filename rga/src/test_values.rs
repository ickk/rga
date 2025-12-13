use crate::algebra::values::{Matrix4, Multivector, Scalar};

#[rustfmt::skip]
pub const MULTIVECTOR_A: Multivector = Multivector {
  s: 2.,
  e1: 3., e2: 5., e3: 7., e4: 11.,
  e41: 13., e42: 17., e43: 19., e23: 23., e31: 29., e12: 31.,
  e423: 37., e431: 41., e412: 43., e321: 47.,
  e1234: 53.,
};

#[rustfmt::skip]
pub const MULTIVECTOR_B: Multivector = Multivector {
  s: -59.,
  e1: -61., e2: -67., e3: -71., e4: -73.,
  e41: -79., e42: -83., e43: -89., e23: -97., e31: -101., e12: -103.,
  e423: -107., e431: -109., e412: -113., e321: -127.,
  e1234: -131.,
};

#[rustfmt::skip]
pub const MULTIVECTOR_C: Multivector = Multivector {
  s: 137.,
  e1: -139., e2: 149., e3: -151., e4: 157.,
  e41: -163., e42: 167., e43: -173., e23: 179., e31: -181., e12: 191.,
  e423: -193., e431: 197., e412: -199., e321: 211.,
  e1234: -223.,
};

#[rustfmt::skip]
pub const SCALAR_A: Scalar = Scalar {
  s: 137.,
};

#[rustfmt::skip]
pub const MATRIX4_A: Matrix4 = Matrix4 {
  m11:  2., m12:  3., m13:  5., m14:  7.,
  m21: 11., m22: 13., m23: 17., m24: 19.,
  m31: 23., m32: 29., m33: 31., m34: 37.,
  m41: 41., m42: 43., m43: 47., m44: 53.,
};
