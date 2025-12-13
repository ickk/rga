use {
  crate::{
    algebra::values::{
      Antiscalar, Bivector, DualNumber, EvenGrade, Multivector, OddGrade,
      Scalar, Trivector, Vector,
    },
    helpers::{
      impl_binary_operation_implicit_output, return_scalar_zero_binary,
    },
  },
  ::core::ops::Mul,
};

/// a • b
#[inline]
pub fn dot<Lhs, Rhs>(a: Lhs, b: Rhs) -> Scalar
where
  Lhs: DotProduct<Rhs>,
{
  a.dot(b)
}

/// a • b
pub trait DotProduct<Rhs> {
  /// a • b
  #[doc(alias = "inner product", alias = "product")]
  fn dot(self, b: Rhs) -> Scalar;
}

impl_binary_operation_implicit_output!(DotProduct::dot {
  Multivector, Multivector => Scalar: multivector_dot_multivector;
  Multivector, Scalar => Scalar: multivector_dot_scalar;
  Multivector, Vector => Scalar: multivector_dot_vector;
  Multivector, Bivector => Scalar: multivector_dot_bivector;
  Multivector, Trivector => Scalar: multivector_dot_trivector;
  Multivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Multivector, DualNumber => Scalar: multivector_dot_dual_number;
  Multivector, OddGrade => Scalar: multivector_dot_odd_grade;
  Multivector, EvenGrade => Scalar: multivector_dot_even_grade;

  Scalar, Multivector => Scalar: scalar_dot_multivector;
  Scalar, Scalar => Scalar: Scalar::mul;
  Scalar, Vector => Scalar: return_scalar_zero_binary;
  Scalar, Bivector => Scalar: return_scalar_zero_binary;
  Scalar, Trivector => Scalar: return_scalar_zero_binary;
  Scalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Scalar, DualNumber => Scalar: scalar_dot_dual_number;
  Scalar, OddGrade => Scalar: return_scalar_zero_binary;
  Scalar, EvenGrade => Scalar: scalar_dot_even_grade;

  Vector, Multivector => Scalar: vector_dot_multivector;
  Vector, Scalar => Scalar: return_scalar_zero_binary;
  Vector, Vector => Scalar: vector_dot_vector;
  Vector, Bivector => Scalar: return_scalar_zero_binary;
  Vector, Trivector => Scalar: return_scalar_zero_binary;
  Vector, Antiscalar => Scalar: return_scalar_zero_binary;
  Vector, DualNumber => Scalar: return_scalar_zero_binary;
  Vector, OddGrade => Scalar: vector_dot_odd_grade;
  Vector, EvenGrade => Scalar: return_scalar_zero_binary;

  Bivector, Multivector => Scalar: bivector_dot_multivector;
  Bivector, Scalar => Scalar: return_scalar_zero_binary;
  Bivector, Vector => Scalar: return_scalar_zero_binary;
  Bivector, Bivector => Scalar: bivector_dot_bivector;
  Bivector, Trivector => Scalar: return_scalar_zero_binary;
  Bivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Bivector, DualNumber => Scalar: return_scalar_zero_binary;
  Bivector, OddGrade => Scalar: return_scalar_zero_binary;
  Bivector, EvenGrade => Scalar: bivector_dot_even_grade;

  Trivector, Multivector => Scalar: trivector_dot_multivector;
  Trivector, Scalar => Scalar: return_scalar_zero_binary;
  Trivector, Vector => Scalar: return_scalar_zero_binary;
  Trivector, Bivector => Scalar: return_scalar_zero_binary;
  Trivector, Trivector => Scalar: trivector_dot_trivector;
  Trivector, Antiscalar => Scalar: return_scalar_zero_binary;
  Trivector, DualNumber => Scalar: return_scalar_zero_binary;
  Trivector, OddGrade => Scalar: trivector_dot_odd_grade;
  Trivector, EvenGrade => Scalar: return_scalar_zero_binary;

  Antiscalar, Multivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Scalar => Scalar: return_scalar_zero_binary;
  Antiscalar, Vector => Scalar: return_scalar_zero_binary;
  Antiscalar, Bivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Trivector => Scalar: return_scalar_zero_binary;
  Antiscalar, Antiscalar => Scalar: return_scalar_zero_binary;
  Antiscalar, DualNumber => Scalar: return_scalar_zero_binary;
  Antiscalar, OddGrade => Scalar: return_scalar_zero_binary;
  Antiscalar, EvenGrade => Scalar: return_scalar_zero_binary;

  DualNumber, Multivector => Scalar: dual_number_dot_multivector;
  DualNumber, Scalar => Scalar: dual_number_dot_scalar;
  DualNumber, Vector => Scalar: return_scalar_zero_binary;
  DualNumber, Bivector => Scalar: return_scalar_zero_binary;
  DualNumber, Trivector => Scalar: return_scalar_zero_binary;
  DualNumber, Antiscalar => Scalar: return_scalar_zero_binary;
  DualNumber, DualNumber => Scalar: dual_number_dot_dual_number;
  DualNumber, OddGrade => Scalar: return_scalar_zero_binary;
  DualNumber, EvenGrade => Scalar: dual_number_dot_even_grade;

  OddGrade, Multivector => Scalar: odd_grade_dot_multivector;
  OddGrade, Scalar => Scalar: return_scalar_zero_binary;
  OddGrade, Vector => Scalar: odd_grade_dot_vector;
  OddGrade, Bivector => Scalar: return_scalar_zero_binary;
  OddGrade, Trivector => Scalar: odd_grade_dot_trivector;
  OddGrade, Antiscalar => Scalar: return_scalar_zero_binary;
  OddGrade, DualNumber => Scalar: return_scalar_zero_binary;
  OddGrade, OddGrade => Scalar: odd_grade_dot_odd_grade;
  OddGrade, EvenGrade => Scalar: return_scalar_zero_binary;

  EvenGrade, Multivector => Scalar: even_grade_dot_multivector;
  EvenGrade, Scalar => Scalar: even_grade_dot_scalar;
  EvenGrade, Vector => Scalar: return_scalar_zero_binary;
  EvenGrade, Bivector => Scalar: even_grade_dot_bivector;
  EvenGrade, Trivector => Scalar: return_scalar_zero_binary;
  EvenGrade, Antiscalar => Scalar: return_scalar_zero_binary;
  EvenGrade, DualNumber => Scalar: even_grade_dot_dual_number;
  EvenGrade, OddGrade => Scalar: return_scalar_zero_binary;
  EvenGrade, EvenGrade => Scalar: even_grade_dot_even_grade;
});

#[inline]
fn multivector_dot_multivector(
  Multivector {
    s: ls,
    e1: l1,
    e2: l2,
    e3: l3,
    e23: l23,
    e31: l31,
    e12: l12,
    e321: l321,
    ..
  }: Multivector,
  Multivector {
    s: rs,
    e1: r1,
    e2: r2,
    e3: r3,
    e23: r23,
    e31: r31,
    e12: r12,
    e321: r321,
    ..
  }: Multivector,
) -> Scalar {
  Scalar {
    s: ls * rs
      + l1 * r1
      + l2 * r2
      + l3 * r3
      + l23 * r23
      + l31 * r31
      + l12 * r12
      + l321 * r321,
  }
}

#[inline]
fn multivector_dot_scalar(
  Multivector { s: ls, .. }: Multivector,
  Scalar { s: rs }: Scalar,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn multivector_dot_vector(
  Multivector {
    e1: l1,
    e2: l2,
    e3: l3,
    ..
  }: Multivector,
  Vector {
    e1: r1,
    e2: r2,
    e3: r3,
    ..
  }: Vector,
) -> Scalar {
  Scalar {
    s: l1 * r1 + l2 * r2 + l3 * r3,
  }
}

#[inline]
fn multivector_dot_bivector(
  Multivector {
    e23: l23,
    e31: l31,
    e12: l12,
    ..
  }: Multivector,
  Bivector {
    e23: r23,
    e31: r31,
    e12: r12,
    ..
  }: Bivector,
) -> Scalar {
  Scalar {
    s: l23 * r23 + l31 * r31 + l12 * r12,
  }
}

#[inline]
fn multivector_dot_trivector(
  Multivector { e321: l321, .. }: Multivector,
  Trivector { e321: r321, .. }: Trivector,
) -> Scalar {
  Scalar { s: l321 * r321 }
}

#[inline]
fn multivector_dot_dual_number(
  Multivector { s: ls, .. }: Multivector,
  DualNumber { s: rs, .. }: DualNumber,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn multivector_dot_odd_grade(
  Multivector {
    e1: l1,
    e2: l2,
    e3: l3,
    e321: l321,
    ..
  }: Multivector,
  OddGrade {
    e1: r1,
    e2: r2,
    e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> Scalar {
  Scalar {
    s: l1 * r1 + l2 * r2 + l3 * r3 + l321 * r321,
  }
}

#[inline]
fn multivector_dot_even_grade(
  Multivector {
    s: ls,
    e23: l23,
    e31: l31,
    e12: l12,
    ..
  }: Multivector,
  EvenGrade {
    s: rs,
    e23: r23,
    e31: r31,
    e12: r12,
    ..
  }: EvenGrade,
) -> Scalar {
  Scalar {
    s: ls * rs + l23 * r23 + l31 * r31 + l12 * r12,
  }
}

#[inline]
fn scalar_dot_multivector(
  Scalar { s: ls }: Scalar,
  Multivector { s: rs, .. }: Multivector,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn scalar_dot_dual_number(
  Scalar { s: ls }: Scalar,
  DualNumber { s: rs, .. }: DualNumber,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn scalar_dot_even_grade(
  Scalar { s: ls }: Scalar,
  EvenGrade { s: rs, .. }: EvenGrade,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn vector_dot_multivector(
  Vector {
    e1: l1,
    e2: l2,
    e3: l3,
    ..
  }: Vector,
  Multivector {
    e1: r1,
    e2: r2,
    e3: r3,
    ..
  }: Multivector,
) -> Scalar {
  Scalar {
    s: l1 * r1 + l2 * r2 + l3 * r3,
  }
}

#[inline]
fn vector_dot_vector(
  Vector {
    e1: l1,
    e2: l2,
    e3: l3,
    ..
  }: Vector,
  Vector {
    e1: r1,
    e2: r2,
    e3: r3,
    ..
  }: Vector,
) -> Scalar {
  Scalar {
    s: l1 * r1 + l2 * r2 + l3 * r3,
  }
}

#[inline]
fn vector_dot_odd_grade(
  Vector {
    e1: l1,
    e2: l2,
    e3: l3,
    ..
  }: Vector,
  OddGrade {
    e1: r1,
    e2: r2,
    e3: r3,
    ..
  }: OddGrade,
) -> Scalar {
  Scalar {
    s: l1 * r1 + l2 * r2 + l3 * r3,
  }
}

#[inline]
fn bivector_dot_multivector(
  Bivector {
    e23: l23,
    e31: l31,
    e12: l12,
    ..
  }: Bivector,
  Multivector {
    e23: r23,
    e31: r31,
    e12: r12,
    ..
  }: Multivector,
) -> Scalar {
  Scalar {
    s: l23 * r23 + l31 * r31 + l12 * r12,
  }
}

#[inline]
fn bivector_dot_bivector(
  Bivector {
    e23: l23,
    e31: l31,
    e12: l12,
    ..
  }: Bivector,
  Bivector {
    e23: r23,
    e31: r31,
    e12: r12,
    ..
  }: Bivector,
) -> Scalar {
  Scalar {
    s: l23 * r23 + l31 * r31 + l12 * r12,
  }
}

#[inline]
fn bivector_dot_even_grade(
  Bivector {
    e23: l23,
    e31: l31,
    e12: l12,
    ..
  }: Bivector,
  EvenGrade {
    e23: r23,
    e31: r31,
    e12: r12,
    ..
  }: EvenGrade,
) -> Scalar {
  Scalar {
    s: l23 * r23 + l31 * r31 + l12 * r12,
  }
}

#[inline]
fn trivector_dot_multivector(
  Trivector { e321: l321, .. }: Trivector,
  Multivector { e321: r321, .. }: Multivector,
) -> Scalar {
  Scalar { s: l321 * r321 }
}

#[inline]
fn trivector_dot_trivector(
  Trivector { e321: l321, .. }: Trivector,
  Trivector { e321: r321, .. }: Trivector,
) -> Scalar {
  Scalar { s: l321 * r321 }
}

#[inline]
fn trivector_dot_odd_grade(
  Trivector { e321: l321, .. }: Trivector,
  OddGrade { e321: r321, .. }: OddGrade,
) -> Scalar {
  Scalar { s: l321 * r321 }
}

#[inline]
fn dual_number_dot_multivector(
  DualNumber { s: ls, .. }: DualNumber,
  Multivector { s: rs, .. }: Multivector,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn dual_number_dot_scalar(
  DualNumber { s: ls, .. }: DualNumber,
  Scalar { s: rs, .. }: Scalar,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn dual_number_dot_dual_number(
  DualNumber { s: ls, .. }: DualNumber,
  DualNumber { s: rs, .. }: DualNumber,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn dual_number_dot_even_grade(
  DualNumber { s: ls, .. }: DualNumber,
  EvenGrade { s: rs, .. }: EvenGrade,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn odd_grade_dot_multivector(
  OddGrade {
    e1: l1,
    e2: l2,
    e3: l3,
    e321: l321,
    ..
  }: OddGrade,
  Multivector {
    e1: r1,
    e2: r2,
    e3: r3,
    e321: r321,
    ..
  }: Multivector,
) -> Scalar {
  Scalar {
    s: l1 * r1 + l2 * r2 + l3 * r3 + l321 * r321,
  }
}

#[inline]
fn odd_grade_dot_vector(
  OddGrade {
    e1: l1,
    e2: l2,
    e3: l3,
    ..
  }: OddGrade,
  Vector {
    e1: r1,
    e2: r2,
    e3: r3,
    ..
  }: Vector,
) -> Scalar {
  Scalar {
    s: l1 * r1 + l2 * r2 + l3 * r3,
  }
}

#[inline]
fn odd_grade_dot_trivector(
  OddGrade { e321: l321, .. }: OddGrade,
  Trivector { e321: r321, .. }: Trivector,
) -> Scalar {
  Scalar { s: l321 * r321 }
}

#[inline]
fn odd_grade_dot_odd_grade(
  OddGrade {
    e1: l1,
    e2: l2,
    e3: l3,
    e321: l321,
    ..
  }: OddGrade,
  OddGrade {
    e1: r1,
    e2: r2,
    e3: r3,
    e321: r321,
    ..
  }: OddGrade,
) -> Scalar {
  Scalar {
    s: l1 * r1 + l2 * r2 + l3 * r3 + l321 * r321,
  }
}

#[inline]
fn even_grade_dot_multivector(
  EvenGrade {
    s: ls,
    e23: l23,
    e31: l31,
    e12: l12,
    ..
  }: EvenGrade,
  Multivector {
    s: rs,
    e23: r23,
    e31: r31,
    e12: r12,
    ..
  }: Multivector,
) -> Scalar {
  Scalar {
    s: ls * rs + l23 * r23 + l31 * r31 + l12 * r12,
  }
}

#[inline]
fn even_grade_dot_scalar(
  EvenGrade { s: ls, .. }: EvenGrade,
  Scalar { s: rs }: Scalar,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn even_grade_dot_bivector(
  EvenGrade {
    e23: l23,
    e31: l31,
    e12: l12,
    ..
  }: EvenGrade,
  Bivector {
    e23: r23,
    e31: r31,
    e12: r12,
    ..
  }: Bivector,
) -> Scalar {
  Scalar {
    s: l23 * r23 + l31 * r31 + l12 * r12,
  }
}

#[inline]
fn even_grade_dot_dual_number(
  EvenGrade { s: ls, .. }: EvenGrade,
  DualNumber { s: rs, .. }: DualNumber,
) -> Scalar {
  Scalar { s: ls * rs }
}

#[inline]
fn even_grade_dot_even_grade(
  EvenGrade {
    s: ls,
    e23: l23,
    e31: l31,
    e12: l12,
    ..
  }: EvenGrade,
  EvenGrade {
    s: rs,
    e23: r23,
    e31: r31,
    e12: r12,
    ..
  }: EvenGrade,
) -> Scalar {
  Scalar {
    s: ls * rs + l23 * r23 + l31 * r31 + l12 * r12,
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    algebra::{operators::*, values::*},
    helpers::def_for_each,
    test_values::*,
  };

  #[test]
  fn linearity() {
    assert_eq!(
      dot(MULTIVECTOR_A + MULTIVECTOR_B, MULTIVECTOR_C),
      (dot(MULTIVECTOR_A, MULTIVECTOR_C) + dot(MULTIVECTOR_B, MULTIVECTOR_C))
    );

    assert_eq!(
      dot(SCALAR_A * MULTIVECTOR_A, MULTIVECTOR_B),
      dot(MULTIVECTOR_A, SCALAR_A * MULTIVECTOR_B)
    );
    assert_eq!(
      dot(MULTIVECTOR_A, SCALAR_A * MULTIVECTOR_B),
      SCALAR_A * dot(MULTIVECTOR_A, MULTIVECTOR_B)
    );

    assert_eq!(
      dot(MULTIVECTOR_A, MULTIVECTOR_B),
      dot(MULTIVECTOR_B, MULTIVECTOR_A)
    );
  }

  #[test]
  fn de_morgans_laws() {
    assert_eq!(
      antidot(MULTIVECTOR_A, MULTIVECTOR_B),
      right_complement(dot(
        left_complement(MULTIVECTOR_A),
        left_complement(MULTIVECTOR_B)
      ))
    );
  }

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
          Multivector::from(dot(multivector, variant)),
          Multivector::from(dot(
            Multivector::from(multivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_scalar_a_*() {
        let scalar: Scalar = grade_0(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dot(scalar, variant)),
          Multivector::from(dot(
            Multivector::from(scalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_vector_a_*() {
        let vector: Vector = grade_1(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dot(vector, variant)),
          Multivector::from(dot(
            Multivector::from(vector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_bivector_a_*() {
        let bivector: Bivector = grade_2(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dot(bivector, variant)),
          Multivector::from(dot(
            Multivector::from(bivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_trivector_a_*() {
        let trivector: Trivector = grade_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dot(trivector, variant)),
          Multivector::from(dot(
            Multivector::from(trivector),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_antiscalar_a_*() {
        let antiscalar: Antiscalar = grade_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dot(antiscalar, variant)),
          Multivector::from(dot(
            Multivector::from(antiscalar),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_dual_number_a_*() {
        let dual_number: DualNumber = grade_0_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dot(dual_number, variant)),
          Multivector::from(dot(
            Multivector::from(dual_number),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_odd_grade_a_*() {
        let odd_grade: OddGrade = grade_1_3(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dot(odd_grade, variant)),
          Multivector::from(dot(
            Multivector::from(odd_grade),
            Multivector::from(variant)
          ))
        );
      }
      #[test]
      fn sparse_even_grade_a_*() {
        let even_grade: EvenGrade = grade_0_2_4(MULTIVECTOR_A);
        assert_eq!(
          Multivector::from(dot(even_grade, variant)),
          Multivector::from(dot(
            Multivector::from(even_grade),
            Multivector::from(variant)
          ))
        );
      }
    }
  }
}
