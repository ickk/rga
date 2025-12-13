use crate::{
  algebra::values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Matrix4, Multivector,
    OddGrade, Scalar, Trivector, Vector,
  },
  helpers::impl_binary_operation,
};

pub use ::core::ops::Sub;

/// a - b
#[inline]
pub fn sub<Lhs, Rhs>(a: Lhs, b: Rhs) -> <Lhs as ::core::ops::Add<Rhs>>::Output
where
  Lhs: ::core::ops::Add<Rhs>,
  Rhs: ::core::ops::Neg<Output = Rhs>,
{
  a + (-b)
}

impl_binary_operation!(Sub::sub {
  Multivector, Multivector => Multivector: sub;
  Multivector, Scalar => Multivector: sub;
  Multivector, Vector => Multivector: sub;
  Multivector, Bivector => Multivector: sub;
  Multivector, Trivector => Multivector: sub;
  Multivector, Antiscalar => Multivector: sub;
  Multivector, DualNumber => Multivector: sub;
  Multivector, OddGrade => Multivector: sub;
  Multivector, EvenGrade => Multivector: sub;

  Scalar, Multivector => Multivector: sub;
  Scalar, Scalar => Scalar: sub;
  Scalar, Vector => Multivector: sub;
  Scalar, Bivector => EvenGrade: sub;
  Scalar, Trivector => Multivector: sub;
  Scalar, Antiscalar => DualNumber: sub;
  Scalar, DualNumber => DualNumber: sub;
  Scalar, OddGrade => Multivector: sub;
  Scalar, EvenGrade => EvenGrade: sub;

  Vector, Multivector => Multivector: sub;
  Vector, Scalar => Multivector: sub;
  Vector, Vector => Vector: sub;
  Vector, Bivector => Multivector: sub;
  Vector, Trivector => OddGrade: sub;
  Vector, Antiscalar => Multivector: sub;
  Vector, DualNumber => Multivector: sub;
  Vector, OddGrade => OddGrade: sub;
  Vector, EvenGrade => Multivector: sub;

  Bivector, Multivector => Multivector: sub;
  Bivector, Scalar => EvenGrade: sub;
  Bivector, Vector => Multivector: sub;
  Bivector, Bivector => Bivector: sub;
  Bivector, Trivector => Multivector: sub;
  Bivector, Antiscalar => EvenGrade: sub;
  Bivector, DualNumber => EvenGrade: sub;
  Bivector, OddGrade => Multivector: sub;
  Bivector, EvenGrade => EvenGrade: sub;

  Trivector, Multivector => Multivector: sub;
  Trivector, Scalar => Multivector: sub;
  Trivector, Vector => OddGrade: sub;
  Trivector, Bivector => Multivector: sub;
  Trivector, Trivector => Trivector: sub;
  Trivector, Antiscalar => Multivector: sub;
  Trivector, DualNumber => Multivector: sub;
  Trivector, OddGrade => OddGrade: sub;
  Trivector, EvenGrade => Multivector: sub;

  Antiscalar, Multivector => Multivector: sub;
  Antiscalar, Scalar => DualNumber: sub;
  Antiscalar, Vector => Multivector: sub;
  Antiscalar, Bivector => EvenGrade: sub;
  Antiscalar, Trivector => Multivector: sub;
  Antiscalar, Antiscalar => Antiscalar: sub;
  Antiscalar, DualNumber => DualNumber: sub;
  Antiscalar, OddGrade => Multivector: sub;
  Antiscalar, EvenGrade => EvenGrade: sub;

  DualNumber, Multivector => Multivector: sub;
  DualNumber, Scalar => DualNumber: sub;
  DualNumber, Vector => Multivector: sub;
  DualNumber, Bivector => EvenGrade: sub;
  DualNumber, Trivector => Multivector: sub;
  DualNumber, Antiscalar => DualNumber: sub;
  DualNumber, DualNumber => DualNumber: sub;
  DualNumber, OddGrade => Multivector: sub;
  DualNumber, EvenGrade => EvenGrade: sub;

  OddGrade, Multivector => Multivector: sub;
  OddGrade, Scalar => Multivector: sub;
  OddGrade, Vector => OddGrade: sub;
  OddGrade, Bivector => Multivector: sub;
  OddGrade, Trivector => OddGrade: sub;
  OddGrade, Antiscalar => Multivector: sub;
  OddGrade, DualNumber => Multivector: sub;
  OddGrade, OddGrade => OddGrade: sub;
  OddGrade, EvenGrade => Multivector: sub;

  EvenGrade, Multivector => Multivector: sub;
  EvenGrade, Scalar => EvenGrade: sub;
  EvenGrade, Vector => Multivector: sub;
  EvenGrade, Bivector => EvenGrade: sub;
  EvenGrade, Trivector => Multivector: sub;
  EvenGrade, Antiscalar => EvenGrade: sub;
  EvenGrade, DualNumber => EvenGrade: sub;
  EvenGrade, OddGrade => Multivector: sub;
  EvenGrade, EvenGrade => EvenGrade: sub;

  Matrix4, Matrix4 => Matrix4: sub;
});

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
      fn sparse_multivector_a_*() {
        let multivector: Multivector = MULTIVECTOR_A;
        assert_eq!(
          Multivector::from(multivector - variant),
          Multivector::from(multivector) - Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(scalar - variant),
          Multivector::from(scalar) - Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(vector - variant),
          Multivector::from(vector) - Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(bivector - variant),
          Multivector::from(bivector) - Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(trivector - variant),
          Multivector::from(trivector) - Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antiscalar - variant),
          Multivector::from(antiscalar) - Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dual_number - variant),
          Multivector::from(dual_number) - Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(odd_grade - variant),
          Multivector::from(odd_grade) - Multivector::from(variant)
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(even_grade - variant),
          Multivector::from(even_grade) - Multivector::from(variant)
        );
      }
    }
  }
}
