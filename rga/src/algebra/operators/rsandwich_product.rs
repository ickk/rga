use crate::{
  algebra::values::{Bivector, Vector},
  helpers::impl_binary_operation,
};

/// a ⟑ b ⟑ ã
#[inline]
pub fn rsandwich<Operator, Argument>(
  a: Operator,
  b: Argument,
) -> <Operator as RSandwichProduct<Argument>>::Output
where
  Operator: RSandwichProduct<Argument>,
{
  a.rsandwich(b)
}

/// a ⟑ b ⟑ ã
pub trait RSandwichProduct<Arg> {
  type Output;

  /// a ⟑ b ⟑ ã
  ///
  /// Geometric sandwich product with the reverse applied on the right
  #[doc(alias = "product")]
  fn rsandwich(self, b: Arg) -> Self::Output;
}

impl_binary_operation!(RSandwichProduct::rsandwich {
  Bivector, Vector => Vector: bivector_rsandwich_vector;
});

#[rustfmt::skip]
#[inline]
fn bivector_rsandwich_vector(
  Bivector {
    e41: o41, e42: o42, e43: o43, e23: o23, e31: o31, e12: o12,
  }: Bivector,
  Vector { e1: a1, e2: a2, e3: a3, e4: a4 }: Vector,
) -> Vector {
  let c =   o23*o23 + o31*o31 + o12*o12;
  let d =   o23*o23 - o31*o31 - o12*o12;
  let e = - o23*o23 + o31*o31 - o12*o12;
  let f = - o23*o23 - o31*o31 + o12*o12;

  let m = - o42*o12 + o43*o31;
  let n =   o41*o12 - o43*o23;
  let o = - o41*o31 + o42*o23;

  let j = o31*o12;
  let k = o12*o23;
  let l = o23*o31;

  let e1 = d*a1 + 2.*(l*a2 + k*a3);
  let e2 = e*a2 + 2.*(j*a3 + l*a1);
  let e3 = f*a3 + 2.*(k*a1 + j*a2);
  let e4 = c*a4 + 2.*(m*a1 + n*a2 + o*a3);

  Vector { e1, e2, e3, e4 }
}

#[cfg(test)]
mod tests {
  use {
    crate::{
      algebra::{operators::*, values::*},
      test_values::*,
    },
    ::approx::assert_ulps_eq,
  };

  #[test]
  fn bivector_rsandwich_vector() {
    let bivector: Bivector = grade_2(MULTIVECTOR_A);
    let vector: Vector = grade_1(MULTIVECTOR_B);

    assert_ulps_eq!(
      dbg!(geometric_product(
        geometric_product(bivector, vector),
        reverse(bivector)
      )),
      dbg!(bivector.rsandwich(vector)).into(),
      epsilon = 0.00_000_000_000_1
    );
  }
}
