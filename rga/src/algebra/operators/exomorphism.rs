use crate::{
  algebra::values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Matrix4, Multivector,
    OddGrade, Scalar, Trivector, Vector,
  },
  helpers::impl_binary_operation_implicit_output,
};

/// Exomorphism: A linear map A: R<sup>4</sup> → R<sup>4</sup> extended to
/// multivectors in Cl<sub>3,0,1</sub>(R), such that it distributes over the
/// [wedge product](crate::WedgeProduct)
///
/// ---
///
/// A: R<sup>4</sup> → R<sup>4</sup>  is a linear map if for any
/// <i>u</i>, <i>v</i> ∈ R<sup>4</sup> and c ∈ R, it satisfies
/// both:
///
/// - A(<i>u</i> + <i>v</i>) = A(<i>u</i>) + A(<i>v</i>)
///
/// - A(c·<i>u</i>) = c·A(<i>u</i>)
///
/// Then the linear map A is extended as an exomorphism over multivectors,
/// A: Cl<sub>3,0,1</sub> → Cl<sub>3,0,1</sub>, where for any
/// <i>u</i> ∧ <i>v</i> = <i>b</i> ∈  Cl<sub>3,0,1</sub>, then
/// A(<i>b</i>) = A(<i>u</i> ∧ <i>v</i>) = A(<i>u</i>) ∧ A(<i>v</i>).
pub fn morph<Lhs, Rhs>(a: Lhs, b: Rhs) -> Rhs
where
  Lhs: Exomorphism<Rhs>,
{
  a.morph(b)
}

/// A linear map A: R<sup>4</sup> → R<sup>4</sup> extended to
/// multivectors in Cl<sub>3,0,1</sub>(R), such that it distributes over the
/// [wedge product](crate::WedgeProduct)
///
/// ---
///
/// A: R<sup>4</sup> → R<sup>4</sup>  is a linear map if for any
/// <i>u</i>, <i>v</i> ∈ R<sup>4</sup> and c ∈ R, it satisfies
/// both:
///
/// - A(<i>u</i> + <i>v</i>) = A(<i>u</i>) + A(<i>v</i>)
///
/// - A(c·<i>u</i>) = c·A(<i>u</i>)
///
/// Then the linear map A is extended as an exomorphism over multivectors,
/// A: Cl<sub>3,0,1</sub> → Cl<sub>3,0,1</sub>, where for any
/// <i>u</i> ∧ <i>v</i> = <i>b</i> ∈  Cl<sub>3,0,1</sub>, then
/// A(<i>b</i>) = A(<i>u</i> ∧ <i>v</i>) = A(<i>u</i>) ∧ A(<i>v</i>).
pub trait Exomorphism<Arg> {
  /// A linear map A: R<sup>4</sup> → R<sup>4</sup> extended to
  /// multivectors in Cl<sub>3,0,1</sub>(R), such that it distributes over the
  /// [wedge product](crate::WedgeProduct)
  ///
  /// ---
  ///
  /// A: R<sup>4</sup> → R<sup>4</sup>  is a linear map if for any
  /// <i>u</i>, <i>v</i> ∈ R<sup>4</sup> and c ∈ R, it satisfies
  /// both:
  ///
  /// - A(<i>u</i> + <i>v</i>) = A(<i>u</i>) + A(<i>v</i>)
  ///
  /// - A(c·<i>u</i>) = c·A(<i>u</i>)
  ///
  /// Then the linear map A is extended as an exomorphism over multivectors,
  /// A: Cl<sub>3,0,1</sub> → Cl<sub>3,0,1</sub>, where for any
  /// <i>u</i> ∧ <i>v</i> = <i>b</i> ∈  Cl<sub>3,0,1</sub>, then
  /// A(<i>b</i>) = A(<i>u</i> ∧ <i>v</i>) = A(<i>u</i>) ∧ A(<i>v</i>).
  #[doc(alias = "exomorphism", alias = "outermorphism", alias = "product")]
  fn morph(self, arg: Arg) -> Arg;
}

impl_binary_operation_implicit_output!(Exomorphism::morph {
  Matrix4, Scalar => Scalar: morph_scalar;
  Matrix4, Vector => Vector: morph_vector;
  Matrix4, Bivector => Bivector: morph_bivector;
  Matrix4, Trivector => Trivector: morph_trivector;
  Matrix4, Antiscalar => Antiscalar: morph_antiscalar;
  Matrix4, Multivector => Multivector: morph_multivector;
  Matrix4, DualNumber => DualNumber: morph_dual_number;
  Matrix4, OddGrade => OddGrade: morph_odd_grade;
  Matrix4, EvenGrade => EvenGrade: morph_even_grade;
});

#[inline]
fn morph_scalar(_: Matrix4, scalar: Scalar) -> Scalar {
  scalar
}

#[rustfmt::skip]
#[inline]
pub(super) fn morph_vector(
  Matrix4 {
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44,
  }: Matrix4,
  Vector { e1: v1, e2: v2, e3: v3, e4: v4 }: Vector
) -> Vector {
  let e1 = m11*v1 + m12*v2 + m13*v3 + m14*v4;
  let e2 = m21*v1 + m22*v2 + m23*v3 + m24*v4;
  let e3 = m31*v1 + m32*v2 + m33*v3 + m34*v4;
  let e4 = m41*v1 + m42*v2 + m43*v3 + m44*v4;

  Vector { e1, e2, e3, e4 }
}

#[rustfmt::skip]
#[inline]
pub(super) fn morph_bivector(
  Matrix4 {
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44,
  }: Matrix4,
  Bivector {
    e41: b41, e42: b42, e43: b43,
    e23: b23, e31: b31, e12: b12,
  }: Bivector
) -> Bivector {
  let x1 = m14*b41 - m12*b12 + m13*b31;
  let y1 = m14*b42 - m13*b23 + m11*b12;
  let z1 = m14*b43 - m11*b31 + m12*b23;
  let w1 = m11*b41 + m12*b42 + m13*b43;

  let x2 = m24*b41 - m22*b12 + m23*b31;
  let y2 = m24*b42 - m23*b23 + m21*b12;
  let z2 = m24*b43 - m21*b31 + m22*b23;
  let w2 = m21*b41 + m22*b42 + m23*b43;

  let x3 = m34*b41 - m32*b12 + m33*b31;
  let y3 = m34*b42 - m33*b23 + m31*b12;
  let z3 = m34*b43 - m31*b31 + m32*b23;
  let w3 = m31*b41 + m32*b42 + m33*b43;

  let e41 = m44*w1 - m41*x1 - m42*y1 - m43*z1;
  let e42 = m44*w2 - m41*x2 - m42*y2 - m43*z2;
  let e43 = m44*w3 - m41*x3 - m42*y3 - m43*z3;

  let e23 = m24*w3 - m21*x3 - m22*y3 - m23*z3;
  let e31 = m34*w1 - m31*x1 - m32*y1 - m33*z1;
  let e12 = m14*w2 - m11*x2 - m12*y2 - m13*z2;

  Bivector { e41, e42, e43, e23, e31, e12 }
}

#[rustfmt::skip]
#[inline]
pub(super) fn morph_trivector(
  Matrix4 {
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44,
  }: Matrix4,
  Trivector { e321: t321, e423: t423, e431: t431, e412: t412 }: Trivector
) -> Trivector {
  let a1 = m22*m33 - m23*m32;
  let b2 = m23*m31 - m21*m33;
  let c3 = m21*m32 - m22*m31;
  let d4 = m23*m34 - m24*m33;
  let e5 = m22*m34 - m24*m32;
  let f6 = m21*m34 - m24*m31;

  let u1 = m42*m13 - m43*m12;
  let v2 = m43*m11 - m41*m13;
  let w3 = m41*m12 - m42*m11;
  let x4 = m43*m14 - m44*m13;
  let y5 = m42*m14 - m44*m12;
  let z6 = m41*m14 - m44*m11;

  let e423 = -(
      t321*(m41*a1 + m42*b2 + m43*c3)
    - t423*(m44*a1 + m42*d4 - m43*e5)
    - t431*(m44*b2 + m43*f6 - m41*d4)
    - t412*(m44*c3 + m41*e5 - m42*f6)
    );
  let e431 =
      t321*(m31*u1 + m32*v2 + m33*w3)
    - t423*(m34*u1 + m32*x4 - m33*y5)
    - t431*(m34*v2 + m33*z6 - m31*x4)
    - t412*(m34*w3 + m31*y5 - m32*z6);
  let e412 = -(
      t321*(m21*u1 + m22*v2 + m23*w3)
    - t423*(m24*u1 + m22*x4 - m23*y5)
    - t431*(m24*v2 + m23*z6 - m21*x4)
    - t412*(m24*w3 + m21*y5 - m22*z6)
    );
  let e321 =
      t321*(m11*a1 + m12*b2 + m13*c3)
    - t423*(m14*a1 + m12*d4 - m13*e5)
    - t431*(m14*b2 + m13*f6 - m11*d4)
    - t412*(m14*c3 + m11*e5 - m12*f6);

  Trivector { e423, e431, e412, e321 }
}

#[rustfmt::skip]
#[inline]
pub(super) fn morph_antiscalar(
  Matrix4 {
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44,
  }: Matrix4,
  Antiscalar { e1234: p1234 }: Antiscalar,
) -> Antiscalar {
  let a1 = m22*m33 - m23*m32;
  let b2 = m23*m31 - m21*m33;
  let c3 = m21*m32 - m22*m31;
  let d4 = m23*m34 - m24*m33;
  let e5 = m22*m34 - m24*m32;
  let f6 = m21*m34 - m24*m31;

  let u1 = m42*m13 - m43*m12;
  let v2 = m43*m11 - m41*m13;
  let w3 = m41*m12 - m42*m11;
  let x4 = m43*m14 - m44*m13;
  let y5 = m42*m14 - m44*m12;
  let z6 = m41*m14 - m44*m11;

  let e1234 = -p1234*(a1*z6 + b2*y5 + c3*x4 + d4*w3 + e5*v2 + f6*u1);

  Antiscalar { e1234 }
}

#[rustfmt::skip]
#[inline]
pub(super) fn morph_multivector(
  matrix: Matrix4,
  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }: Multivector,
) -> Multivector {
  let Scalar { s } = morph_scalar(
    matrix, Scalar { s }
  );
  let Vector { e1, e2, e3, e4 } = morph_vector(
    matrix, Vector { e1, e2, e3, e4 }
  );
  let Bivector { e41, e42, e43, e23, e31, e12 } = morph_bivector(
    matrix, Bivector { e41, e42, e43, e23, e31, e12 },
  );
  let Trivector { e423, e431, e412, e321 } = morph_trivector(
    matrix, Trivector { e423, e431, e412, e321 }
  );
  let Antiscalar { e1234 } = morph_antiscalar(
    matrix, Antiscalar { e1234 }
  );

  Multivector {
    s,
    e1, e2, e3, e4,
    e41, e42, e43, e23, e31, e12,
    e423, e431, e412, e321,
    e1234,
  }
}

#[rustfmt::skip]
#[inline]
fn morph_dual_number(
  matrix: Matrix4,
  DualNumber { s, e1234 }: DualNumber,
) -> DualNumber {
  let Scalar { s } = morph_scalar(
    matrix, Scalar { s }
  );
  let Antiscalar { e1234 } = morph_antiscalar(
    matrix, Antiscalar { e1234 }
  );

  DualNumber { s, e1234 }
}

#[rustfmt::skip]
#[inline]
fn morph_odd_grade(
  matrix: Matrix4,
  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }: OddGrade,
) -> OddGrade {
  let Vector { e1, e2, e3, e4 } = morph_vector(
    matrix, Vector { e1, e2, e3, e4 }
  );
  let Trivector { e423, e431, e412, e321 } = morph_trivector(
    matrix, Trivector { e423, e431, e412, e321 }
  );

  OddGrade {
    e1, e2, e3, e4,
    e423, e431, e412, e321,
  }
}

#[rustfmt::skip]
#[inline]
fn morph_even_grade(
  matrix: Matrix4,
  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }: EvenGrade,
) -> EvenGrade {
  let Scalar { s } = morph_scalar(
    matrix, Scalar { s }
  );
  let Bivector { e41, e42, e43, e23, e31, e12 } = morph_bivector(
    matrix, Bivector { e41, e42, e43, e23, e31, e12 },
  );
  let Antiscalar { e1234 } = morph_antiscalar(
    matrix, Antiscalar { e1234 }
  );

  EvenGrade {
    s,
    e41, e42, e43, e23, e31, e12,
    e1234,
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    algebra::{operators::*, values::*},
    helpers::def_for_each,
    test_values::*,
  };

  def_for_each! {
    for variant in [
      multivector_a: MULTIVECTOR_A,
      multivector_b: MULTIVECTOR_B,
      multivector_c: MULTIVECTOR_C,
      scalar_a: grade_0(MULTIVECTOR_A),
      scalar_b: grade_0(MULTIVECTOR_B),
      scalar_c: grade_0(MULTIVECTOR_C),
      vector_a: grade_1(MULTIVECTOR_A),
      vector_b: grade_1(MULTIVECTOR_B),
      vector_c: grade_1(MULTIVECTOR_C),
      bivector_a: grade_2(MULTIVECTOR_A),
      bivector_b: grade_2(MULTIVECTOR_B),
      bivector_c: grade_2(MULTIVECTOR_C),
      trivector_a: grade_3(MULTIVECTOR_A),
      trivector_b: grade_3(MULTIVECTOR_B),
      trivector_c: grade_3(MULTIVECTOR_C),
      antiscalar_a: grade_4(MULTIVECTOR_A),
      antiscalar_b: grade_4(MULTIVECTOR_B),
      antiscalar_c: grade_4(MULTIVECTOR_C),
      dual_number_a: grade_0_4(MULTIVECTOR_A),
      dual_number_b: grade_0_4(MULTIVECTOR_B),
      dual_number_c: grade_0_4(MULTIVECTOR_C),
      odd_grade_a: grade_1_3(MULTIVECTOR_A),
      odd_grade_b: grade_1_3(MULTIVECTOR_B),
      odd_grade_c: grade_1_3(MULTIVECTOR_C),
      even_grade_a: grade_0_2_4(MULTIVECTOR_A),
      even_grade_b: grade_0_2_4(MULTIVECTOR_B),
      even_grade_c: grade_0_2_4(MULTIVECTOR_C),
    ] {
      #[test]
      fn identity_matrix4_*() {
        assert_eq!(dbg!(Matrix4::IDENTITY.morph(variant)), dbg!(variant));
      }
    }
  }

  // test Vector, and then just check the linearity property of the others
  // types below
  #[rustfmt::skip]
  #[test]
  fn matrix_a_vector_a() {
    let vector: Vector = grade_1(MULTIVECTOR_A);

    assert_eq!(
      dbg!(MATRIX4_A.morph(vector)),
      dbg!(Vector { e1: 133., e2: 426., e3: 838.,e4: 1250. })
    );
  }

  #[test]
  fn matrix_a_scalar_a() {
    let scalar: Scalar = grade_0(MULTIVECTOR_A);

    assert_eq!(MATRIX4_A.morph(scalar), scalar);
  }

  #[test]
  fn matrix_a_bivector() {
    let vector_a: Vector = grade_1(MULTIVECTOR_A);
    let vector_b: Vector = grade_1(MULTIVECTOR_B);

    assert_eq!(
      wedge(MATRIX4_A.morph(vector_a), MATRIX4_A.morph(vector_b)),
      MATRIX4_A.morph(wedge(vector_a, vector_b))
    );
  }

  #[test]
  fn matrix_a_trivector() {
    let vector: Vector = grade_1(MULTIVECTOR_A);
    let bivector: Bivector = grade_2(MULTIVECTOR_B);

    assert_eq!(
      wedge(MATRIX4_A.morph(vector), MATRIX4_A.morph(bivector)),
      MATRIX4_A.morph(wedge(vector, bivector))
    );
  }

  #[test]
  fn matrix_a_antiscalar() {
    let vector: Vector = grade_1(MULTIVECTOR_A);
    let trivector: Trivector = grade_3(MULTIVECTOR_B);

    assert_eq!(
      wedge(MATRIX4_A.morph(vector), MATRIX4_A.morph(trivector)),
      MATRIX4_A.morph(wedge(vector, trivector))
    );
  }

  #[test]
  fn matrix_a_multivector() {
    assert_eq!(
      dbg!(wedge(
        MATRIX4_A.morph(MULTIVECTOR_A),
        MATRIX4_A.morph(MULTIVECTOR_B),
      )),
      dbg!(MATRIX4_A.morph(wedge(MULTIVECTOR_A, MULTIVECTOR_B)))
    );
  }
}
