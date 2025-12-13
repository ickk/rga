use crate::algebra::values::{Matrix4, Vector};

/// (AB)<sub>i,j</sub> = ∑<sub><sub>k=1</sub></sub> a<sub>i,k</sub> b<sub>k,j</sub>
#[inline]
pub fn matrix_product<Lhs, Rhs>(a: Lhs, b: Rhs) -> Lhs::Output
where
  Lhs: MatrixProduct<Rhs>,
{
  a.matrix_product(b)
}

/// (AB)<sub>i,j</sub> = ∑<sub><sub>k=1</sub></sub> a<sub>i,k</sub> b<sub>k,j</sub>
pub trait MatrixProduct<Rhs> {
  type Output;

  /// (AB)<sub>i,j</sub> = ∑<sub><sub>k=1</sub></sub> a<sub>i,k</sub> b<sub>k,j</sub>
  #[doc(alias = "product")]
  fn matrix_product(self, b: Rhs) -> Self::Output;
}

impl MatrixProduct<Matrix4> for Matrix4 {
  type Output = Matrix4;

  #[rustfmt::skip]
  #[inline]
  fn matrix_product(self, b: Matrix4) -> Self::Output {
    let Matrix4 {
      m11: a11, m12: a12, m13: a13, m14: a14,
      m21: a21, m22: a22, m23: a23, m24: a24,
      m31: a31, m32: a32, m33: a33, m34: a34,
      m41: a41, m42: a42, m43: a43, m44: a44,
    } = self;
    let Matrix4 {
      m11: b11, m12: b12, m13: b13, m14: b14,
      m21: b21, m22: b22, m23: b23, m24: b24,
      m31: b31, m32: b32, m33: b33, m34: b34,
      m41: b41, m42: b42, m43: b43, m44: b44,
    } = b;

    let m11 = a11*b11 + a12*b21 + a13*b31 + a14*b41;
    let m21 = a21*b11 + a22*b21 + a23*b31 + a24*b41;
    let m31 = a31*b11 + a32*b21 + a33*b31 + a34*b41;
    let m41 = a41*b11 + a42*b21 + a43*b31 + a44*b41;

    let m12 = a11*b12 + a12*b22 + a13*b32 + a14*b42;
    let m22 = a21*b12 + a22*b22 + a23*b32 + a24*b42;
    let m32 = a31*b12 + a32*b22 + a33*b32 + a34*b42;
    let m42 = a41*b12 + a42*b22 + a43*b32 + a44*b42;

    let m13 = a11*b13 + a12*b23 + a13*b33 + a14*b43;
    let m23 = a21*b13 + a22*b23 + a23*b33 + a24*b43;
    let m33 = a31*b13 + a32*b23 + a33*b33 + a34*b43;
    let m43 = a41*b13 + a42*b23 + a43*b33 + a44*b43;

    let m14 = a11*b14 + a12*b24 + a13*b34 + a14*b44;
    let m24 = a21*b14 + a22*b24 + a23*b34 + a24*b44;
    let m34 = a31*b14 + a32*b24 + a33*b34 + a34*b44;
    let m44 = a41*b14 + a42*b24 + a43*b34 + a44*b44;

    Matrix4 {
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44,
    }
  }
}

impl MatrixProduct<Vector> for Matrix4 {
  type Output = Vector;

  #[rustfmt::skip]
  #[inline]
  fn matrix_product(self, b: Vector) -> Self::Output {
    let Matrix4 {
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44,
    } = self;
    let Vector { e1: b1, e2: b2, e3: b3, e4: b4 } = b;

    let e1 = m11*b1 + m12*b2 + m13*b3 + m14*b4;
    let e2 = m21*b1 + m22*b2 + m23*b3 + m24*b4;
    let e3 = m31*b1 + m32*b2 + m33*b3 + m34*b4;
    let e4 = m41*b1 + m42*b2 + m43*b3 + m44*b4;

    Vector { e1, e2, e3, e4 }
  }
}
