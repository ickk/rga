use crate::{
  algebra::values::{
    Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
    Scalar, Trivector, Vector,
  },
  helpers::{
    impl_binary_operation_implicit_output, return_antiscalar_zero_binary,
  },
};

/// a ∘ b
#[inline]
pub fn antidot<Lhs, Rhs>(a: Lhs, b: Rhs) -> Antiscalar
where
  Lhs: AntidotProduct<Rhs>,
{
  a.antidot(b)
}

/// a ∘ b
pub trait AntidotProduct<Rhs> {
  /// a ∘ b
  #[doc(alias = "inner product", alias = "product")]
  fn antidot(self, b: Rhs) -> Antiscalar;
}

impl_binary_operation_implicit_output!(AntidotProduct::antidot {
  Multivector, Multivector => Antiscalar: multivector_antidot_multivector;
  Multivector, Scalar => Antiscalar: return_antiscalar_zero_binary;
  Multivector, Vector => Antiscalar: multivector_antidot_vector;
  Multivector, Bivector => Antiscalar: multivector_antidot_bivector;
  Multivector, Trivector => Antiscalar: multivector_antidot_trivector;
  Multivector, Antiscalar => Antiscalar: multivector_antidot_antiscalar;
  Multivector, DualNumber => Antiscalar: multivector_antidot_dual_number;
  Multivector, OddGrade => Antiscalar: multivector_antidot_odd_grade;
  Multivector, EvenGrade => Antiscalar: multivector_antidot_even_grade;

  Scalar, Multivector => Antiscalar: return_antiscalar_zero_binary;
  Scalar, Scalar => Antiscalar: return_antiscalar_zero_binary;
  Scalar, Vector => Antiscalar: return_antiscalar_zero_binary;
  Scalar, Bivector => Antiscalar: return_antiscalar_zero_binary;
  Scalar, Trivector => Antiscalar: return_antiscalar_zero_binary;
  Scalar, Antiscalar => Antiscalar: return_antiscalar_zero_binary;
  Scalar, DualNumber => Antiscalar: return_antiscalar_zero_binary;
  Scalar, OddGrade => Antiscalar: return_antiscalar_zero_binary;
  Scalar, EvenGrade => Antiscalar: return_antiscalar_zero_binary;

  Vector, Multivector => Antiscalar: vector_antidot_multivector;
  Vector, Scalar => Antiscalar: return_antiscalar_zero_binary;
  Vector, Vector => Antiscalar: vector_antidot_vector;
  Vector, Bivector => Antiscalar: return_antiscalar_zero_binary;
  Vector, Trivector => Antiscalar: return_antiscalar_zero_binary;
  Vector, Antiscalar => Antiscalar: return_antiscalar_zero_binary;
  Vector, DualNumber => Antiscalar: return_antiscalar_zero_binary;
  Vector, OddGrade => Antiscalar: vector_antidot_odd_grade;
  Vector, EvenGrade => Antiscalar: return_antiscalar_zero_binary;

  Bivector, Multivector => Antiscalar: bivector_antidot_multivector;
  Bivector, Scalar => Antiscalar: return_antiscalar_zero_binary;
  Bivector, Vector => Antiscalar: return_antiscalar_zero_binary;
  Bivector, Bivector => Antiscalar: bivector_antidot_bivector;
  Bivector, Trivector => Antiscalar: return_antiscalar_zero_binary;
  Bivector, Antiscalar => Antiscalar: return_antiscalar_zero_binary;
  Bivector, DualNumber => Antiscalar: return_antiscalar_zero_binary;
  Bivector, OddGrade => Antiscalar: return_antiscalar_zero_binary;
  Bivector, EvenGrade => Antiscalar: bivector_antidot_even_grade;

  Trivector, Multivector => Antiscalar: trivector_antidot_multivector;
  Trivector, Scalar => Antiscalar: return_antiscalar_zero_binary;
  Trivector, Vector => Antiscalar: return_antiscalar_zero_binary;
  Trivector, Bivector => Antiscalar: return_antiscalar_zero_binary;
  Trivector, Trivector => Antiscalar: trivector_antidot_trivector;
  Trivector, Antiscalar => Antiscalar: return_antiscalar_zero_binary;
  Trivector, DualNumber => Antiscalar: return_antiscalar_zero_binary;
  Trivector, OddGrade => Antiscalar: trivector_antidot_odd_grade;
  Trivector, EvenGrade => Antiscalar: return_antiscalar_zero_binary;

  Antiscalar, Multivector => Antiscalar: antiscalar_antidot_multivector;
  Antiscalar, Scalar => Antiscalar: return_antiscalar_zero_binary;
  Antiscalar, Vector => Antiscalar: return_antiscalar_zero_binary;
  Antiscalar, Bivector => Antiscalar: return_antiscalar_zero_binary;
  Antiscalar, Trivector => Antiscalar: return_antiscalar_zero_binary;
  Antiscalar, Antiscalar => Antiscalar: antiscalar_antidot_antiscalar;
  Antiscalar, DualNumber => Antiscalar: antiscalar_antidot_dual_number;
  Antiscalar, OddGrade => Antiscalar: return_antiscalar_zero_binary;
  Antiscalar, EvenGrade => Antiscalar: antiscalar_antidot_even_grade;

  DualNumber, Multivector => Antiscalar: dual_number_antidot_multivector;
  DualNumber, Scalar => Antiscalar: return_antiscalar_zero_binary;
  DualNumber, Vector => Antiscalar: return_antiscalar_zero_binary;
  DualNumber, Bivector => Antiscalar: return_antiscalar_zero_binary;
  DualNumber, Trivector => Antiscalar: return_antiscalar_zero_binary;
  DualNumber, Antiscalar => Antiscalar: dual_number_antidot_antiscalar;
  DualNumber, DualNumber => Antiscalar: dual_number_antidot_dual_number;
  DualNumber, OddGrade => Antiscalar: return_antiscalar_zero_binary;
  DualNumber, EvenGrade => Antiscalar: dual_number_antidot_even_grade;

  OddGrade, Multivector => Antiscalar: odd_grade_antidot_multivector;
  OddGrade, Scalar => Antiscalar: return_antiscalar_zero_binary;
  OddGrade, Vector => Antiscalar: odd_grade_antidot_vector;
  OddGrade, Bivector => Antiscalar: return_antiscalar_zero_binary;
  OddGrade, Trivector => Antiscalar: odd_grade_antidot_trivector;
  OddGrade, Antiscalar => Antiscalar: return_antiscalar_zero_binary;
  OddGrade, DualNumber => Antiscalar: return_antiscalar_zero_binary;
  OddGrade, OddGrade => Antiscalar: odd_grade_antidot_odd_grade;
  OddGrade, EvenGrade => Antiscalar: return_antiscalar_zero_binary;

  EvenGrade, Multivector => Antiscalar: even_grade_antidot_multivector;
  EvenGrade, Scalar => Antiscalar: return_antiscalar_zero_binary;
  EvenGrade, Vector => Antiscalar: return_antiscalar_zero_binary;
  EvenGrade, Bivector => Antiscalar: even_grade_antidot_bivector;
  EvenGrade, Trivector => Antiscalar: return_antiscalar_zero_binary;
  EvenGrade, Antiscalar => Antiscalar: even_grade_antidot_antiscalar;
  EvenGrade, DualNumber => Antiscalar: even_grade_antidot_dual_number;
  EvenGrade, OddGrade => Antiscalar: return_antiscalar_zero_binary;
  EvenGrade, EvenGrade => Antiscalar: even_grade_antidot_even_grade;
});

#[rustfmt::skip]
#[inline]
fn multivector_antidot_multivector(
  Multivector {
    e4: l4,
    e41: l41, e42: l42, e43: l43,
    e423: l423, e431: l431, e412: l412,
    e1234: l1234,
    ..
  }: Multivector,
  Multivector {
    e4: r4,
    e41: r41, e42: r42, e43: r43,
    e423: r423, e431: r431, e412: r412,
    e1234: r1234,
    ..
  }: Multivector,
) -> Antiscalar {
  Antiscalar { e1234: l4*r4 + l41*r41 + l42*r42 + l43*r43
    + l423*r423 + l431*r431 + l412*r412 + l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn multivector_antidot_vector(
  Multivector { e4: l4, .. }: Multivector,
  Vector { e4: r4, .. }: Vector,
) -> Antiscalar {
  Antiscalar { e1234: l4*r4 }
}

#[rustfmt::skip]
#[inline]
fn multivector_antidot_bivector(
  Multivector { e41: l41, e42: l42, e43: l43, .. }: Multivector,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Antiscalar {
  Antiscalar { e1234: l41*r41 + l42*r42 + l43*r43 }
}

#[rustfmt::skip]
#[inline]
fn multivector_antidot_trivector(
  Multivector { e423: l423, e431: l431, e412: l412, .. }: Multivector,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Antiscalar {
  Antiscalar { e1234: l423*r423 + l431*r431 + l412*r412 }
}

#[rustfmt::skip]
#[inline]
fn multivector_antidot_antiscalar(
  Multivector { e1234: l1234, .. }: Multivector,
  Antiscalar { e1234: r1234, .. }: Antiscalar,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn multivector_antidot_dual_number(
  Multivector { e1234: l1234, .. }: Multivector,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn multivector_antidot_odd_grade(
  Multivector {
    e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: Multivector,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> Antiscalar {
  Antiscalar { e1234: l4*r4 + l423*r423 + l431*r431 + l412*r412 }
}

#[rustfmt::skip]
#[inline]
fn multivector_antidot_even_grade(
  Multivector {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: Multivector,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> Antiscalar {
  Antiscalar { e1234: l41*r41 + l42*r42 + l43*r43 + l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn vector_antidot_multivector(
  Vector { e4: l4, .. }: Vector,
  Multivector { e4: r4, .. }: Multivector,
) -> Antiscalar {

  Antiscalar { e1234: l4 * r4 }
}

#[rustfmt::skip]
#[inline]
fn vector_antidot_vector(
  Vector { e4: l4, .. }: Vector,
  Vector { e4: r4, .. }: Vector,
) -> Antiscalar {
  Antiscalar { e1234: l4 * r4 }
}

#[rustfmt::skip]
#[inline]
fn vector_antidot_odd_grade(
  Vector { e4: l4, .. }: Vector,
  OddGrade { e4: r4, .. }: OddGrade,
) -> Antiscalar {
  Antiscalar { e1234: l4 * r4 }
}

#[rustfmt::skip]
#[inline]
fn bivector_antidot_multivector(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  Multivector { e41: r41, e42: r42, e43: r43, .. }: Multivector,
) -> Antiscalar {
  Antiscalar { e1234: l41*r41 + l42*r42 + l43*r43 }
}

#[rustfmt::skip]
#[inline]
fn bivector_antidot_bivector(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Antiscalar {
  Antiscalar { e1234: l41*r41 + l42*r42 + l43*r43 }
}

#[rustfmt::skip]
#[inline]
fn bivector_antidot_even_grade(
  Bivector { e41: l41, e42: l42, e43: l43, .. }: Bivector,
  EvenGrade { e41: r41, e42: r42, e43: r43, .. }: EvenGrade,
) -> Antiscalar {
  Antiscalar { e1234: l41*r41 + l42*r42 + l43*r43 }
}

#[rustfmt::skip]
#[inline]
fn trivector_antidot_multivector(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  Multivector { e423: r423, e431: r431, e412: r412, .. }: Multivector,
) -> Antiscalar {
  Antiscalar { e1234: l423*r423 + l431*r431 + l412*r412 }
}

#[rustfmt::skip]
#[inline]
fn trivector_antidot_trivector(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Antiscalar {
  Antiscalar { e1234: l423*r423 + l431*r431 + l412*r412 }
}

#[rustfmt::skip]
#[inline]
fn trivector_antidot_odd_grade(
  Trivector { e423: l423, e431: l431, e412: l412, .. }: Trivector,
  OddGrade { e423: r423, e431: r431, e412: r412, .. }: OddGrade,
) -> Antiscalar {
  Antiscalar { e1234: l423*r423 + l431*r431 + l412*r412 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antidot_multivector(
  Antiscalar { e1234: l1234, .. }: Antiscalar,
  Multivector { e1234: r1234, .. }: Multivector,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antidot_antiscalar(
  Antiscalar { e1234: l1234, .. }: Antiscalar,
  Antiscalar { e1234: r1234, .. }: Antiscalar,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antidot_dual_number(
  Antiscalar { e1234: l1234, .. }: Antiscalar,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn antiscalar_antidot_even_grade(
  Antiscalar { e1234: l1234, .. }: Antiscalar,
  EvenGrade { e1234: r1234, .. }: EvenGrade,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antidot_multivector(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Multivector { e1234: r1234, .. }: Multivector,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antidot_antiscalar(
  DualNumber { e1234: l1234, .. }: DualNumber,
  Antiscalar { e1234: r1234, .. }: Antiscalar,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antidot_dual_number(
  DualNumber { e1234: l1234, .. }: DualNumber,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn dual_number_antidot_even_grade(
  DualNumber { e1234: l1234, .. }: DualNumber,
  EvenGrade { e1234: r1234, .. }: EvenGrade,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antidot_multivector(
  OddGrade {
    e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: OddGrade,
  Multivector {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: Multivector,
) -> Antiscalar {
  Antiscalar { e1234: l4*r4 + l423*r423 + l431*r431 + l412*r412 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antidot_vector(
  OddGrade { e4: l4, .. }: OddGrade,
  Vector { e4: r4, .. }: Vector,
) -> Antiscalar {
  Antiscalar { e1234: l4*r4 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antidot_trivector(
  OddGrade { e423: l423, e431: l431, e412: l412, .. }: OddGrade,
  Trivector { e423: r423, e431: r431, e412: r412, .. }: Trivector,
) -> Antiscalar {
  Antiscalar { e1234: l423*r423 + l431*r431 + l412*r412 }
}

#[rustfmt::skip]
#[inline]
fn odd_grade_antidot_odd_grade(
  OddGrade {
    e4: l4,
    e423: l423, e431: l431, e412: l412,
    ..
  }: OddGrade,
  OddGrade {
    e4: r4,
    e423: r423, e431: r431, e412: r412,
    ..
  }: OddGrade,
) -> Antiscalar {
  Antiscalar { e1234: l4*r4 + l423*r423 + l431*r431 + l412*r412 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antidot_multivector(
  EvenGrade {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: EvenGrade,
  Multivector {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: Multivector,
) -> Antiscalar {
  Antiscalar { e1234: l41*r41 + l42*r42 + l43*r43 + l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antidot_bivector(
  EvenGrade { e41: l41, e42: l42, e43: l43, .. }: EvenGrade,
  Bivector { e41: r41, e42: r42, e43: r43, .. }: Bivector,
) -> Antiscalar {
  Antiscalar { e1234: l41*r41 + l42*r42 + l43*r43 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antidot_antiscalar(
  EvenGrade { e1234: l1234, .. }: EvenGrade,
  Antiscalar { e1234: r1234, .. }: Antiscalar,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antidot_dual_number(
  EvenGrade { e1234: l1234, .. }: EvenGrade,
  DualNumber { e1234: r1234, .. }: DualNumber,
) -> Antiscalar {
  Antiscalar { e1234: l1234*r1234 }
}

#[rustfmt::skip]
#[inline]
fn even_grade_antidot_even_grade(
  EvenGrade {
    e41: l41, e42: l42, e43: l43,
    e1234: l1234,
    ..
  }: EvenGrade,
  EvenGrade {
    e41: r41, e42: r42, e43: r43,
    e1234: r1234,
    ..
  }: EvenGrade,
) -> Antiscalar {
  Antiscalar { e1234: l41*r41 + l42*r42 + l43*r43 + l1234*r1234 }
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
      fn sparse_multivector_a_*() {
        let multivector: Multivector = MULTIVECTOR_A;
        assert_eq!(
          Multivector::from(antidot(multivector, variant)),
          Multivector::from(antidot(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antidot(scalar, variant)),
          Multivector::from(antidot(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antidot(vector, variant)),
          Multivector::from(antidot(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antidot(bivector, variant)),
          Multivector::from(antidot(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antidot(trivector, variant)),
          Multivector::from(antidot(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antidot(antiscalar, variant)),
          Multivector::from(antidot(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antidot(dual_number, variant)),
          Multivector::from(antidot(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antidot(odd_grade, variant)),
          Multivector::from(antidot(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(antidot(even_grade, variant)),
          Multivector::from(antidot(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
