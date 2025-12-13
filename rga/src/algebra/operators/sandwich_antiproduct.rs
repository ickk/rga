use crate::{
  algebra::{
    operators::{LeftComplement, RightComplement, SandwichProduct},
    values::{
      Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
      Scalar, Trivector, Vector,
    },
  },
  helpers::{impl_binary_operation, return_scalar_nan_binary},
};

/// a ⟇ b ⟇ a⁻¹
#[inline]
pub fn antisandwich<Operator, Argument>(
  a: Operator,
  b: Argument,
) -> <Operator as SandwichAntiproduct<Argument>>::Output
where
  Operator: SandwichAntiproduct<Argument>,
{
  a.antisandwich(b)
}

/// a ⟇ b ⟇ a⁻¹
pub trait SandwichAntiproduct<Arg> {
  type Output;

  /// a ⟇ b ⟇ a⁻¹
  ///
  /// Geometric sandwich antiproduct with the inverse applied on the right
  #[doc(alias = "product")]
  fn antisandwich(self, b: Arg) -> Self::Output;
}

#[inline]
fn general_impl<Lhs, Rhs>(
  a: Lhs,
  b: Rhs,
) -> <<<Lhs as LeftComplement>::Output as SandwichProduct<
  <Rhs as LeftComplement>::Output,
>>::Output as RightComplement>::Output
where
  Lhs: Copy + LeftComplement,
  <Lhs as LeftComplement>::Output:
    SandwichProduct<<Rhs as LeftComplement>::Output>,
  Rhs: Copy + LeftComplement,
  <<Lhs as LeftComplement>::Output as SandwichProduct<
    <Rhs as LeftComplement>::Output,
  >>::Output: RightComplement,
{
  a.left_complement()
    .sandwich(b.left_complement())
    .right_complement()
}

impl_binary_operation!(SandwichAntiproduct::antisandwich {
  Multivector, Multivector => Multivector: general_impl;
  Multivector, Scalar => Multivector: general_impl;
  Multivector, Vector => Multivector: general_impl;
  Multivector, Bivector => Multivector: general_impl;
  Multivector, Trivector => Multivector: general_impl;
  Multivector, Antiscalar => Antiscalar: general_impl;
  Multivector, DualNumber => Multivector: general_impl;
  Multivector, EvenGrade => Multivector: general_impl;
  Multivector, OddGrade => Multivector: general_impl;

  Scalar, Multivector => Scalar: return_scalar_nan_binary;
  Scalar, Scalar => Scalar: return_scalar_nan_binary;
  Scalar, Vector => Scalar: return_scalar_nan_binary;
  Scalar, Bivector => Scalar: return_scalar_nan_binary;
  Scalar, Trivector => Scalar: return_scalar_nan_binary;
  Scalar, Antiscalar => Scalar: return_scalar_nan_binary;
  Scalar, DualNumber => Scalar: return_scalar_nan_binary;
  Scalar, EvenGrade => Scalar: return_scalar_nan_binary;
  Scalar, OddGrade => Scalar: return_scalar_nan_binary;

  Vector, Multivector => Multivector: general_impl;
  Vector, Scalar => Scalar: general_impl;
  Vector, Vector => Vector: general_impl;
  Vector, Bivector => Bivector: general_impl;
  Vector, Trivector => Trivector: general_impl;
  Vector, Antiscalar => Antiscalar: general_impl;
  Vector, DualNumber => DualNumber: general_impl;
  Vector, EvenGrade => EvenGrade: general_impl;
  Vector, OddGrade => OddGrade: general_impl;

  Bivector, Multivector => Multivector: general_impl;
  Bivector, Scalar => Scalar: general_impl;
  Bivector, Vector => OddGrade: general_impl;
  Bivector, Bivector => Bivector: general_impl;
  Bivector, Trivector => OddGrade: general_impl;
  Bivector, Antiscalar => Antiscalar: general_impl;
  Bivector, DualNumber => DualNumber: general_impl;
  Bivector, EvenGrade => EvenGrade: general_impl;
  Bivector, OddGrade => OddGrade: general_impl;

  Trivector, Multivector => Multivector: general_impl;
  Trivector, Scalar => Scalar: general_impl;
  Trivector, Vector => Vector: general_impl;
  Trivector, Bivector => Bivector: general_impl;
  Trivector, Trivector => Trivector: general_impl;
  Trivector, Antiscalar => Antiscalar: general_impl;
  Trivector, DualNumber => DualNumber: general_impl;
  Trivector, EvenGrade => EvenGrade: general_impl;
  Trivector, OddGrade => OddGrade: general_impl;

  Antiscalar, Multivector => Multivector: general_impl;
  Antiscalar, Scalar => Scalar: general_impl;
  Antiscalar, Vector => Vector: general_impl;
  Antiscalar, Bivector => Bivector: general_impl;
  Antiscalar, Trivector => Trivector: general_impl;
  Antiscalar, Antiscalar => Antiscalar: general_impl;
  Antiscalar, DualNumber => DualNumber: general_impl;
  Antiscalar, EvenGrade => EvenGrade: general_impl;
  Antiscalar, OddGrade => OddGrade: general_impl;

  DualNumber, Multivector => Multivector: general_impl;
  DualNumber, Scalar => Scalar: general_impl;
  DualNumber, Vector => OddGrade: general_impl;
  DualNumber, Bivector => Bivector: general_impl;
  DualNumber, Trivector => OddGrade: general_impl;
  DualNumber, Antiscalar => Antiscalar: general_impl;
  DualNumber, DualNumber => DualNumber: general_impl;
  DualNumber, EvenGrade => EvenGrade: general_impl;
  DualNumber, OddGrade => OddGrade: general_impl;

  EvenGrade, Multivector => Multivector: general_impl;
  EvenGrade, Scalar => Scalar: general_impl;
  EvenGrade, Vector => OddGrade: general_impl;
  EvenGrade, Bivector => Bivector: general_impl;
  EvenGrade, Trivector => OddGrade: general_impl;
  EvenGrade, Antiscalar => Antiscalar: general_impl;
  EvenGrade, DualNumber => DualNumber: general_impl;
  EvenGrade, EvenGrade => EvenGrade: general_impl;
  EvenGrade, OddGrade => OddGrade: general_impl;

  OddGrade, Multivector => Multivector: general_impl;
  OddGrade, Scalar => Scalar: general_impl;
  OddGrade, Vector => OddGrade: general_impl;
  OddGrade, Bivector => Bivector: general_impl;
  OddGrade, Trivector => OddGrade: general_impl;
  OddGrade, Antiscalar => Antiscalar: general_impl;
  OddGrade, DualNumber => DualNumber: general_impl;
  OddGrade, EvenGrade => EvenGrade: general_impl;
  OddGrade, OddGrade => OddGrade: general_impl;
});

#[cfg(test)]
mod tests {
  use {
    crate::{
      algebra::{operators::*, values::*},
      helpers::def_for_each,
      test_values::*,
    },
    ::approx::assert_ulps_eq,
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
        assert_ulps_eq!(
          dbg!(geometric_antiproduct(
            geometric_antiproduct(multivector, variant),
            antiinverse(multivector)
          )),
          dbg!(multivector.antisandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert!(dbg!(scalar.antisandwich(variant)).is_nan());
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_antiproduct(
            geometric_antiproduct(vector, variant),
            antiinverse(vector)
          )),
          dbg!(vector.antisandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_antiproduct(
            geometric_antiproduct(bivector, variant),
            antiinverse(bivector)
          )),
          dbg!(bivector.antisandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_antiproduct(
            geometric_antiproduct(trivector, variant),
            antiinverse(trivector)
          )),
          dbg!(trivector.antisandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_antiproduct(
            geometric_antiproduct(antiscalar, variant),
            antiinverse(antiscalar)
          )),
          dbg!(antiscalar.antisandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_antiproduct(
            geometric_antiproduct(dual_number, variant),
            antiinverse(dual_number)
          )),
          dbg!(dual_number.antisandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_antiproduct(
            geometric_antiproduct(odd_grade, variant),
            antiinverse(odd_grade)
          )),
          dbg!(odd_grade.antisandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_ulps_eq!(
          dbg!(geometric_antiproduct(
            geometric_antiproduct(even_grade, variant),
            antiinverse(even_grade)
          )),
          dbg!(even_grade.antisandwich(variant)).into(),
          epsilon = 0.00_000_000_000_1
        );
      }
    }
  }
}
