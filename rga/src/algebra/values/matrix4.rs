use {
  crate::{
    algebra::values::{Scalar, Vector},
    F,
  },
  ::core::{
    fmt,
    ops::{Index, IndexMut},
  },
};

/// A 4×4 matrix
///
/// Fields can be accessed directly, or the matrix may be indexed as
/// `a[(i, j)]`, where `i`, `j` specify the row, column respectively. Element
/// indices are 1-based.
///
/// ```txt
///  ⎡  m11, m12, m13, m14  ⎤
///  ⎢  m21, m22, m23, m24  ⎥
///  ⎢  m31, m32, m33, m34  ⎥
///  ⎣  m41, m42, m43, m44  ⎦
/// ```
///
/// Memory layout is column-major.
#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq)]
#[repr(C)]
pub struct Matrix4 {
  // col 1
  pub m11: F,
  pub m21: F,
  pub m31: F,
  pub m41: F,
  // col 2
  pub m12: F,
  pub m22: F,
  pub m32: F,
  pub m42: F,
  // col 3
  pub m13: F,
  pub m23: F,
  pub m33: F,
  pub m43: F,
  // col 4
  pub m14: F,
  pub m24: F,
  pub m34: F,
  pub m44: F,
}

impl Matrix4 {
  /// The matrix containing all zeros
  /// ```txt
  ///  ⎡ 0, 0, 0, 0 ⎤
  ///  ⎢ 0, 0, 0, 0 ⎥
  ///  ⎢ 0, 0, 0, 0 ⎥
  ///  ⎣ 0, 0, 0, 0 ⎦
  /// ```
  #[rustfmt::skip]
  pub const ZERO: Self = Self {
    m11: 0., m12: 0., m13: 0., m14: 0.,
    m21: 0., m22: 0., m23: 0., m24: 0.,
    m31: 0., m32: 0., m33: 0., m34: 0.,
    m41: 0., m42: 0., m43: 0., m44: 0.,
  };
  /// The Identity matrix
  /// ```txt
  ///  ⎡ 1, 0, 0, 0 ⎤
  ///  ⎢ 0, 1, 0, 0 ⎥
  ///  ⎢ 0, 0, 1, 0 ⎥
  ///  ⎣ 0, 0, 0, 1 ⎦
  /// ```
  #[rustfmt::skip]
  pub const IDENTITY: Self = Self {
    m11: 1., m12: 0., m13: 0., m14: 0.,
    m21: 0., m22: 1., m23: 0., m24: 0.,
    m31: 0., m32: 0., m33: 1., m34: 0.,
    m41: 0., m42: 0., m43: 0., m44: 1.,
  };
  /// The matrix containing all NaNs
  /// ```txt
  ///  ⎡ NaN, NaN, NaN, NaN ⎤
  ///  ⎢ NaN, NaN, NaN, NaN ⎥
  ///  ⎢ NaN, NaN, NaN, NaN ⎥
  ///  ⎣ NaN, NaN, NaN, NaN ⎦
  /// ```
  #[rustfmt::skip]
  pub const NAN: Self = Self {
    m11: F::NAN, m12: F::NAN, m13: F::NAN, m14: F::NAN,
    m21: F::NAN, m22: F::NAN, m23: F::NAN, m24: F::NAN,
    m31: F::NAN, m32: F::NAN, m33: F::NAN, m34: F::NAN,
    m41: F::NAN, m42: F::NAN, m43: F::NAN, m44: F::NAN,
  };

  /// Get the j-th column from the matrix as a [`struct@Vector`]
  ///
  /// Column indices are 1-based.
  ///
  /// ## Panics
  ///
  /// Panics if index is not in the range 1..=4
  #[rustfmt::skip]
  #[inline(always)]
  pub fn col(self, j: usize) -> Vector {
    match j {
      1 => Vector { e1: self.m11, e2: self.m21, e3: self.m31, e4: self.m41 },
      2 => Vector { e1: self.m12, e2: self.m22, e3: self.m32, e4: self.m42 },
      3 => Vector { e1: self.m13, e2: self.m23, e3: self.m33, e4: self.m43 },
      4 => Vector { e1: self.m14, e2: self.m24, e3: self.m34, e4: self.m44 },
      _ => panic!("index out of bounds")
    }
  }

  /// Get the i-th row from the matrix as a [`struct@Vector`]
  ///
  /// Row indices are 1-based.
  ///
  /// ## Panics
  ///
  /// Panics if index is not in the range 1..=4
  #[rustfmt::skip]
  #[inline]
  pub fn row(self, i: usize) -> Vector {
    match i {
      1 => Vector { e1: self.m11, e2: self.m12, e3: self.m13, e4: self.m14 },
      2 => Vector { e1: self.m21, e2: self.m22, e3: self.m23, e4: self.m24 },
      3 => Vector { e1: self.m31, e2: self.m32, e3: self.m33, e4: self.m34 },
      4 => Vector { e1: self.m41, e2: self.m42, e3: self.m43, e4: self.m44 },
      _ => panic!("index out of bounds")
    }
  }

  /// Get the columns of the matrix as an array of [`struct@Vector`]s
  #[rustfmt::skip]
  #[inline(always)]
  pub fn to_cols(self) -> [Vector; 4] {
    [
      Vector { e1: self.m11, e2: self.m21, e3: self.m31, e4: self.m41 },
      Vector { e1: self.m12, e2: self.m22, e3: self.m32, e4: self.m42 },
      Vector { e1: self.m13, e2: self.m23, e3: self.m33, e4: self.m43 },
      Vector { e1: self.m14, e2: self.m24, e3: self.m34, e4: self.m44 },
    ]
  }

  /// Get the columns of the matrix as an array of `[F; 4]` arrays
  #[rustfmt::skip]
  #[inline(always)]
  pub fn to_cols_array_2d(self) -> [[F; 4]; 4] {
    let Matrix4 {
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44,
    } = self;
    [
      [m11, m21, m31, m41],
      [m12, m22, m32, m42],
      [m13, m23, m33, m43],
      [m14, m24, m34, m44],
    ]
  }

  /// Get the rows of the matrix as an array of `[F; 4]` arrays
  #[rustfmt::skip]
  #[inline]
  pub fn to_rows_array_2d(self) -> [[F; 4]; 4] {
    let Matrix4 {
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44,
    } = self;
    [
      [m11, m12, m13, m14],
      [m21, m22, m23, m24],
      [m31, m32, m33, m34],
      [m41, m42, m43, m44],
    ]
  }

  /// Create a matrix from an array containing the column vectors as arrays
  #[rustfmt::skip]
  #[inline(always)]
  pub fn from_cols_array_2d(cols: [[F; 4]; 4]) -> Self {
    let [
      [m11, m21, m31, m41],
      [m12, m22, m32, m42],
      [m13, m23, m33, m43],
      [m14, m24, m34, m44],
    ] = cols;
    Matrix4 {
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44,
    }
  }

  /// Create a matrix from an array containing the row vectors as arrays
  #[rustfmt::skip]
  #[inline]
  pub fn from_rows_array_2d(rows: [[F; 4]; 4]) -> Self {
    let [
      [m11, m12, m13, m14],
      [m21, m22, m23, m24],
      [m31, m32, m33, m34],
      [m41, m42, m43, m44],
    ] = rows;
    Matrix4 {
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44,
    }
  }

  /// A<sup>T</sup>, the transpose of the matrix
  ///
  /// ---
  ///
  /// (A<sup>T</sup> )<sub>i,j</sub> = (A)<sub>j,i</sub>
  #[inline]
  pub fn transpose(self) -> Matrix4 {
    self._transpose()
  }
  #[rustfmt::skip]
  #[inline(always)]
  pub(crate) fn _transpose(self) -> Matrix4 {
    let Matrix4 {
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44,
    } = self;

    Matrix4 {
      m11,      m12: m21, m13: m31, m14: m41,
      m21: m12, m22,      m23: m32, m24: m42,
      m31: m13, m32: m23, m33,      m34: m43,
      m41: m14, m42: m24, m43: m34, m44,
    }
  }

  /// Compute the minors M, of the matrix A.
  ///
  /// ---
  ///
  /// (M)<sub>i,j</sub> = det( \[a<sub>p,q</sub>\]<sub><sub>p≠i,q≠j</sub></sub> )
  #[inline]
  pub fn minors(self) -> Matrix4 {
    self._minors()
  }
  #[rustfmt::skip]
  #[inline(always)]
  pub(crate) fn _minors(self) -> Matrix4 {
    let Matrix4 {
      m11: a11, m12: a12, m13: a13, m14: a14,
      m21: a21, m22: a22, m23: a23, m24: a24,
      m31: a31, m32: a32, m33: a33, m34: a34,
      m41: a41, m42: a42, m43: a43, m44: a44,
    } = self;
    let a2 = a33*a44 - a34*a43;
    let b2 = a31*a42 - a32*a41;
    let a3 = a23*a44 - a24*a43;
    let b3 = a21*a42 - a22*a41;
    let a4 = a23*a34 - a24*a33;
    let b4 = a21*a32 - a22*a31;
    let c3 = a13*a44 - a14*a43;
    let d3 = a11*a42 - a12*a41;
    let c4 = a13*a34 - a14*a33;
    let d4 = a11*a32 - a12*a31;
    let e4 = a13*a24 - a14*a23;
    let f4 = a11*a22 - a12*a21;

    let m11 = a22*a2 - a32*a3 + a42*a4;
    let m12 = a21*a2 - a31*a3 + a41*a4;
    let m13 = a24*b2 - a34*b3 + a44*b4;
    let m14 = a23*b2 - a33*b3 + a43*b4;

    let m21 = a12*a2 - a32*c3 + a42*c4;
    let m22 = a11*a2 - a31*c3 + a41*c4;
    let m23 = a14*b2 - a34*d3 + a44*d4;
    let m24 = a13*b2 - a33*d3 + a43*d4;

    let m31 = a12*a3 - a22*c3 + a42*e4;
    let m32 = a11*a3 - a21*c3 + a41*e4;
    let m33 = a14*b3 - a24*d3 + a44*f4;
    let m34 = a13*b3 - a23*d3 + a43*f4;

    let m41 = a12*a4 - a22*c4 + a32*e4;
    let m42 = a11*a4 - a21*c4 + a31*e4;
    let m43 = a14*b4 - a24*d4 + a34*f4;
    let m44 = a13*b4 - a23*d4 + a33*f4;

    Matrix4 {
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44,
    }
  }

  /// Compute the cofactors C, of the matrix A.
  ///
  /// ---
  ///
  /// Where M are the minors of A, then
  /// (C)<sub>i,j</sub> = (-1)<sup>i+j </sup>(M)<sub>i,j</sub>
  #[inline]
  pub fn cofactors(self) -> Matrix4 {
    self._cofactors()
  }
  #[rustfmt::skip]
  #[inline(always)]
  pub(crate) fn _cofactors(self) -> Matrix4 {
    let Matrix4 {
      m11: a11, m12: a12, m13: a13, m14: a14,
      m21: a21, m22: a22, m23: a23, m24: a24,
      m31: a31, m32: a32, m33: a33, m34: a34,
      m41: a41, m42: a42, m43: a43, m44: a44,
    } = self._minors();

    Matrix4 {
      m11:  a11, m12: -a12, m13:  a13, m14: -a14,
      m21: -a21, m22:  a22, m23: -a23, m24:  a24,
      m31:  a31, m32: -a32, m33:  a33, m34: -a34,
      m41: -a41, m42:  a42, m43: -a43, m44:  a44,
    }
  }

  /// Compute the classical adjoint of the matrix, i.e. the transpose of the
  /// cofactor matrix
  #[doc(alias = "adjoint")]
  #[inline]
  pub fn adjugate(self) -> Matrix4 {
    self._adjugate()
  }
  #[inline(always)]
  pub(crate) fn _adjugate(self) -> Matrix4 {
    self._cofactors()._transpose()
  }

  /// Compute the determinant of the matrix
  #[inline]
  pub fn determinant(self) -> F {
    self._determinant()
  }
  #[inline(always)]
  pub(crate) fn _determinant(self) -> F {
    let [m11, m21, m31, m41] = self.to_cols_array_2d()[0];
    let [c11, c21, c31, c41] = self._cofactors().to_cols_array_2d()[0];
    m11 * c11 + m21 * c21 + m31 * c31 + m41 * c41
  }

  /// A⁻¹ the inverse of the matrix, if it exists
  ///
  /// The result will be [`IsNan`](crate::IsNan) if the matrix is not
  /// invertible.
  ///
  /// ---
  ///
  /// This is the inverse with respect to the
  /// [`MatrixProduct`](crate::MatrixProduct), such that
  ///
  /// A⁻¹A = A A⁻¹ = I
  #[inline]
  pub fn inverse(self) -> Matrix4 {
    let q = {
      let d = self._determinant();
      if d == 0.0 {
        Scalar::NAN
      } else {
        Scalar(1. / d)
      }
    };
    q * self._adjugate()
  }

  pub fn trace(self) -> F {
    let Matrix4 {
      m11, m22, m33, m44, ..
    } = self;
    m11 + m22 + m33 + m44
  }

  fn _row_echelon_form(self) {
    todo!()
  }

  fn _reduced_row_echelon_form(self) {
    todo!()
  }

  fn _eigenvalues(self) {
    todo!()
  }

  fn _eigenvectors(self) {
    todo!()
  }

  fn _solve(self) {
    todo!()
  }
}

impl Index<(usize, usize)> for Matrix4 {
  type Output = F;

  /// Get a reference to the element in the i-th row and j-th column
  ///
  /// Element indices are 1-based.
  ///
  /// ## Panics
  ///
  /// Panics if indices are not in the range 1..=4
  #[inline]
  fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
    match (i, j) {
      (1, 1) => &self.m11,
      (1, 2) => &self.m12,
      (1, 3) => &self.m13,
      (1, 4) => &self.m14,
      (2, 1) => &self.m21,
      (2, 2) => &self.m22,
      (2, 3) => &self.m23,
      (2, 4) => &self.m24,
      (3, 1) => &self.m31,
      (3, 2) => &self.m32,
      (3, 3) => &self.m33,
      (3, 4) => &self.m34,
      (4, 1) => &self.m41,
      (4, 2) => &self.m42,
      (4, 3) => &self.m43,
      (4, 4) => &self.m44,
      _ => panic!("Index out of bounds"),
    }
  }
}

impl IndexMut<(usize, usize)> for Matrix4 {
  /// Get a mutable reference to the element in the i-th row and j-th column
  ///
  /// Element indices are 1-based.
  ///
  /// ## Panics
  ///
  /// Panics if indices are not in the range 1..=4
  #[inline]
  fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
    match (i, j) {
      (1, 1) => &mut self.m11,
      (1, 2) => &mut self.m12,
      (1, 3) => &mut self.m13,
      (1, 4) => &mut self.m14,
      (2, 1) => &mut self.m21,
      (2, 2) => &mut self.m22,
      (2, 3) => &mut self.m23,
      (2, 4) => &mut self.m24,
      (3, 1) => &mut self.m31,
      (3, 2) => &mut self.m32,
      (3, 3) => &mut self.m33,
      (3, 4) => &mut self.m34,
      (4, 1) => &mut self.m41,
      (4, 2) => &mut self.m42,
      (4, 3) => &mut self.m43,
      (4, 4) => &mut self.m44,
      _ => panic!("Index out of bounds"),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::{algebra::values::*, test_values::*};

  #[test]
  fn index() {
    assert_eq!(MATRIX4_A[(1, 1)], 2.);
    assert_eq!(MATRIX4_A[(4, 1)], 41.);
    assert_eq!(MATRIX4_A[(2, 3)], 17.);
    assert_eq!(MATRIX4_A[(2, 4)], 19.);
  }

  #[test]
  fn index_mut() {
    let mut m = Matrix4::ZERO;
    m[(1, 2)] = 1.2;
    m[(4, 3)] = 4.3;
    m[(4, 4)] = 4.4;
    assert_eq!(
      m,
      Matrix4 {
        m12: 1.2,
        m43: 4.3,
        m44: 4.4,
        ..zero()
      }
    );
  }

  #[test]
  fn cofactors() {
    // verify that cofactor expansion along each column and each row is equivalent
    let (m, a) = (MATRIX4_A, MATRIX4_A.cofactors());
    let det_c1 = {
      let (a1, m1) = (a.col(1), m.col(1));
      a1.e1 * m1.e1 + a1.e2 * m1.e2 + a1.e3 * m1.e3 + a1.e4 * m1.e4
    };
    let det_c2 = {
      let (a2, m2) = (a.col(2), m.col(2));
      a2.e1 * m2.e1 + a2.e2 * m2.e2 + a2.e3 * m2.e3 + a2.e4 * m2.e4
    };
    let det_c3 = {
      let (a3, m3) = (a.col(3), m.col(3));
      a3.e1 * m3.e1 + a3.e2 * m3.e2 + a3.e3 * m3.e3 + a3.e4 * m3.e4
    };
    let det_c4 = {
      let (a4, m4) = (a.col(4), m.col(4));
      a4.e1 * m4.e1 + a4.e2 * m4.e2 + a4.e3 * m4.e3 + a4.e4 * m4.e4
    };
    let det_r1 = {
      let (a1, m1) = (a.row(1), m.row(1));
      a1.e1 * m1.e1 + a1.e2 * m1.e2 + a1.e3 * m1.e3 + a1.e4 * m1.e4
    };
    let det_r2 = {
      let (a2, m2) = (a.row(2), m.row(2));
      a2.e1 * m2.e1 + a2.e2 * m2.e2 + a2.e3 * m2.e3 + a2.e4 * m2.e4
    };
    let det_r3 = {
      let (a3, m3) = (a.row(3), m.row(3));
      a3.e1 * m3.e1 + a3.e2 * m3.e2 + a3.e3 * m3.e3 + a3.e4 * m3.e4
    };
    let det_r4 = {
      let (a4, m4) = (a.row(4), m.row(4));
      a4.e1 * m4.e1 + a4.e2 * m4.e2 + a4.e3 * m4.e3 + a4.e4 * m4.e4
    };
    assert!(
      det_c1 == det_c2
        && det_c1 == det_c3
        && det_c1 == det_c4
        && det_c1 == det_r1
        && det_c1 == det_r2
        && det_c1 == det_r3
        && det_c1 == det_r4,
      "cofactor expansion failed"
    );
  }
}

impl fmt::Debug for Matrix4 {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    if fmt.alternate() {
      let width = fmt.width().unwrap_or(8);
      let precision = fmt.precision().unwrap_or(2);
      // pretty print
      fmt.write_fmt(format_args!(
        "Matrix4 {{\n\
        \x20 {m11:width$.precision$},\
        \x20 {m12:width$.precision$},\
        \x20 {m13:width$.precision$},\
        \x20 {m14:width$.precision$},\n\
        \x20 {m21:width$.precision$},\
        \x20 {m22:width$.precision$},\
        \x20 {m23:width$.precision$},\
        \x20 {m24:width$.precision$},\n\
        \x20 {m31:width$.precision$},\
        \x20 {m32:width$.precision$},\
        \x20 {m33:width$.precision$},\
        \x20 {m34:width$.precision$},\n\
        \x20 {m41:width$.precision$},\
        \x20 {m42:width$.precision$},\
        \x20 {m43:width$.precision$},\
        \x20 {m44:width$.precision$},\n\
        }}",
        m11 = self.m11,
        m12 = self.m12,
        m13 = self.m13,
        m14 = self.m14,
        m21 = self.m21,
        m22 = self.m22,
        m23 = self.m23,
        m24 = self.m24,
        m31 = self.m31,
        m32 = self.m32,
        m33 = self.m33,
        m34 = self.m34,
        m41 = self.m41,
        m42 = self.m42,
        m43 = self.m43,
        m44 = self.m44,
      ))
    } else {
      fmt
        .debug_struct("Matrix4")
        .field("m11", &self.m11)
        .field("m12", &self.m12)
        .field("m13", &self.m13)
        .field("m14", &self.m14)
        .field("m21", &self.m21)
        .field("m22", &self.m22)
        .field("m23", &self.m23)
        .field("m24", &self.m24)
        .field("m31", &self.m31)
        .field("m32", &self.m32)
        .field("m33", &self.m33)
        .field("m34", &self.m34)
        .field("m41", &self.m41)
        .field("m42", &self.m42)
        .field("m43", &self.m43)
        .field("m44", &self.m44)
        .finish()
    }
  }
}

#[cfg(test)]
mod test {
  use {
    crate::{
      algebra::{
        operators::{matrix_product, IsNan},
        values::*,
      },
      test_values::*,
    },
    ::approx::assert_ulps_eq,
  };

  #[test]
  fn inverse_matrix4_a() {
    assert_ulps_eq!(
      dbg!(matrix_product(dbg!(MATRIX4_A.inverse()), MATRIX4_A)),
      Matrix4::IDENTITY,
      epsilon = 0.00_000_000_000_1
    );
  }

  #[test]
  fn no_inverse_matrix4() {
    #[rustfmt::skip]
    let m = Matrix4 {
      m11: 1., m12: 2., m13: 3.,
      m21: 2., m22: 4., m23: 6.,
      ..Matrix4::IDENTITY
    };
    dbg!(m, m.cofactors(), m.adjugate(), m.determinant());
    assert!(
      dbg!(m.inverse()).is_nan(),
      "Matrix has proportional rows, therefore it is not invertible"
    );
    assert!(Matrix4::ZERO.inverse().is_nan(),);
  }
}
