//! Operator functions; `wedge`, `dot`, `inverse`, `sandwich`,
//! `geometric_product`..

mod add;
mod antidot_product;
mod antiinverse;
mod antireverse;
mod antiwedge_product;
mod attitude;
mod bulk;
mod bulk_contraction;
mod bulk_expansion;
mod bulk_norm;
mod div;
mod dot_product;
mod exomorphism;
mod geometric_antiproduct;
mod geometric_norm;
mod geometric_product;
mod grade_select;
mod inverse;
mod is_geometric;
mod is_inf;
mod is_nan;
mod left_bulk_dual;
mod left_complement;
mod left_weight_dual;
mod matrix_product;
mod mul;
mod neg;
mod partial_eq;
mod reverse;
mod right_bulk_dual;
mod right_complement;
mod right_weight_dual;
mod rsandwich_product;
mod sandwich_antiproduct;
mod sandwich_product;
mod sub;
mod unitize;
mod wedge_product;
mod weight;
mod weight_contraction;
mod weight_expansion;
mod weight_norm;

#[doc(inline)]
pub use self::{functions::*, traits::*};

/// Collection of operators as traits
pub mod traits {
  pub use super::{
    add::Add,
    antidot_product::AntidotProduct,
    antiinverse::Antiinverse,
    antireverse::Antireverse,
    antiwedge_product::AntiwedgeProduct,
    attitude::Attitude,
    bulk::Bulk,
    bulk_contraction::BulkContraction,
    bulk_expansion::BulkExpansion,
    bulk_norm::{BulkNorm, BulkNormSquared},
    div::Div,
    dot_product::DotProduct,
    exomorphism::Exomorphism,
    geometric_antiproduct::GeometricAntiproduct,
    geometric_norm::GeometricNorm,
    geometric_product::GeometricProduct,
    grade_select::GradeSelect,
    inverse::Inverse,
    is_geometric::IsGeometric,
    is_inf::IsInf,
    is_nan::IsNan,
    left_bulk_dual::LeftBulkDual,
    left_complement::LeftComplement,
    left_weight_dual::LeftWeightDual,
    matrix_product::MatrixProduct,
    neg::Neg,
    reverse::Reverse,
    right_bulk_dual::RightBulkDual,
    right_complement::RightComplement,
    right_weight_dual::RightWeightDual,
    rsandwich_product::RSandwichProduct,
    sandwich_antiproduct::SandwichAntiproduct,
    sandwich_product::SandwichProduct,
    sub::Sub,
    unitize::Unitize,
    wedge_product::WedgeProduct,
    weight::Weight,
    weight_contraction::WeightContraction,
    weight_expansion::WeightExpansion,
    weight_norm::{WeightNorm, WeightNormSquared},
  };
}

/// Collection of geometric operators as free-functions
pub mod functions {
  pub use super::{
    add::add,
    antidot_product::antidot,
    antiinverse::antiinverse,
    antireverse::antireverse,
    antiwedge_product::antiwedge,
    attitude::attitude,
    bulk::bulk,
    bulk_contraction::bulk_contraction,
    bulk_expansion::bulk_expansion,
    bulk_norm::{bulk_norm, bulk_norm_squared},
    div::div,
    dot_product::dot,
    exomorphism::morph,
    geometric_antiproduct::geometric_antiproduct,
    geometric_norm::geometric_norm,
    geometric_product::geometric_product,
    grade_select::{
      grade_0, grade_0_2_4, grade_0_4, grade_1, grade_1_3, grade_2, grade_3,
      grade_4,
    },
    inverse::inverse,
    left_bulk_dual::left_bulk_dual,
    left_complement::left_complement,
    left_weight_dual::left_weight_dual,
    matrix_product::matrix_product,
    reverse::reverse,
    right_bulk_dual::bulk_dual,
    right_complement::right_complement,
    right_weight_dual::weight_dual,
    rsandwich_product::rsandwich,
    sandwich_antiproduct::antisandwich,
    sandwich_product::sandwich,
    sub::sub,
    unitize::unitize,
    wedge_product::wedge,
    weight::weight,
    weight_contraction::weight_contraction,
    weight_expansion::weight_expansion,
    weight_norm::{weight_norm, weight_norm_squared},
  };
}
