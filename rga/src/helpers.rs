use crate::algebra::values::{Antiscalar, DualNumber, Scalar};

/// Unconditionally return unmodified s
#[inline(always)]
pub(crate) fn identity<S: Copy>(s: S) -> S {
  s
}

/// Unconditionally return rhs
#[inline(always)]
pub(crate) fn return_rhs<Lhs, Rhs>(_: Lhs, rhs: Rhs) -> Rhs {
  rhs
}

/// Unconditionally return Scalar::ZERO
#[inline(always)]
pub(crate) fn return_scalar_zero_unary<S>(_: S) -> Scalar {
  Scalar::ZERO
}

/// Unconditionally return Scalar::ZERO
#[inline(always)]
pub(crate) fn return_scalar_zero_binary<Lhs, Rhs>(_: Lhs, _: Rhs) -> Scalar {
  Scalar::ZERO
}

/// Unconditionally return Antiscalar::ZERO
#[inline(always)]
pub(crate) fn return_antiscalar_zero_binary<Lhs, Rhs>(
  _: Lhs,
  _: Rhs,
) -> Antiscalar {
  Antiscalar::ZERO
}

/// Unconditionally return Scalar::NAN
#[inline(always)]
pub(crate) fn return_scalar_nan_binary<Lhs, Rhs>(_: Lhs, _: Rhs) -> Scalar {
  Scalar::NAN
}

/// Unconditionally return Antiscalar::NAN
#[inline(always)]
pub(crate) fn _return_antiscalar_nan_binary<Lhs, Rhs>(
  _: Lhs,
  _: Rhs,
) -> Antiscalar {
  Antiscalar::NAN
}

/// Unconditionally return the negative of rhs
#[inline(always)]
pub(crate) fn return_neg_rhs<Lhs, Rhs: ::core::ops::Neg>(
  _: Lhs,
  rhs: Rhs,
) -> <Rhs as ::core::ops::Neg>::Output {
  rhs.neg()
}

/// Unconditionally return the conjugate of the rhs dual number
#[inline(always)]
pub(crate) fn return_conjugate_rhs<Lhs>(
  _: Lhs,
  DualNumber { s, e1234 }: DualNumber,
) -> DualNumber {
  DualNumber { s, e1234: -e1234 }
}

/// Automate the boilerplate to impl many unary operator trait definitions
macro_rules! impl_unary_operation {
  (
    $trait:ident::$trait_fn:ident {
      $($input:ty => $output:ty: $implementation_fn:path;)*
    }
  ) => {
    $(impl $trait for $input {
      type Output = $output;

      #[inline]
      fn $trait_fn(self) -> $output {
        $implementation_fn(self)
      }
    })*
  };
}
pub(crate) use impl_unary_operation;

/// Automate the boilerplate to impl many binary operator trait definitions
macro_rules! impl_binary_operation {
  (
    $trait:ident::$trait_fn:ident {
      $($lhs:ty, $rhs:ty => $output:ty: $implementation_fn:path;)*
    }
  ) => {
    $(impl $trait<$rhs> for $lhs {
      type Output = $output;

      #[inline]
      fn $trait_fn(self, other: $rhs) -> $output {
        $implementation_fn(self, other)
      }
    })*
  };
}
pub(crate) use impl_binary_operation;

macro_rules! impl_binary_operation_implicit_output {
  (
    $trait:ident::$trait_fn:ident {
      $($lhs:ty, $rhs:ty => $output:ty: $implementation_fn:path;)*
    }
  ) => {
    $(impl $trait<$rhs> for $lhs {
      #[inline]
      fn $trait_fn(self, other: $rhs) -> $output {
        $implementation_fn(self, other)
      }
    })*
  };
}
pub(crate) use impl_binary_operation_implicit_output;

/// Automate many function definitions differing only name and a binding to
/// an expression at the beginning of the function's scope
#[cfg(test)]
macro_rules! def_for_each {
  (
    for $binding:pat in [
      $($variant:ident: $expr:expr),*$(,)?
    ] {
      $(
        $(#[$fn_attr:meta])*
        $fn_vis:vis fn $fn_name_prefix:ident*(
          $($fn_param:ident: $fn_param_ty:ty),*$(,)?
        ) $(-> $fn_ret:ty)? {
          $($fn_body_tokens:tt)*
        }
      )*
    }
  ) => {
    def_for_each!(@for-variant {
      binding: $binding,
      variant-list: [$({
        variant: $variant,
        expr: $expr,
      }),*],
      fn-list: [$({
        attrs: [$($fn_attr)*],
        vis: $fn_vis,
        name-prefix: $fn_name_prefix,
        params: [$({
          name: $fn_param,
          ty: $fn_param_ty,
        }),*],
        ret: $($fn_ret)?,
        body: {$($fn_body_tokens)*},
      }),*],
    });
  };
  (@for-variant {
    binding: $binding:pat,
    variant-list: [$({
      variant: $variant:ident,
      expr: $expr:expr,
    }),*],
    fn-list: $fn_list:tt,
  }) => {
    $(def_for_each!(@for-body {
      binding: $binding,
      variant: $variant,
      expr: $expr,
      fn-list: $fn_list,
    });)*
  };
  (@for-body {
    binding: $binding:pat,
    variant: $variant:ident,
    expr: $expr:expr,
    fn-list: [$({
      attrs: [$($fn_attrs:meta)*],
      vis: $fn_vis:vis,
      name-prefix: $fn_name_prefix:ident,
      params: [$({
        name: $fn_param:ident,
        ty: $fn_param_ty:ty,
      }),*],
      ret: $($fn_ret:ty)?,
      body: {$($fn_body_tt:tt)*},
    }),*],
  }) => {
    $(
      ::paste::paste!{
        $(#[$fn_attrs])*
        $fn_vis fn [<$fn_name_prefix $variant>](
          $($fn_param: $fn_param_ty),*
        ) $(-> $fn_ret)? {
          let $binding = $expr;
          $($fn_body_tt)*
        }
      }
    )*
  };
}
#[cfg(test)]
pub(crate) use def_for_each;
