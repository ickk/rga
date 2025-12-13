//! Geometry operator functions; `meet`, `join`, `support`, `distance`, ..

mod antisupport;
mod central_antiprojection;
mod central_projection;
mod compose;
mod distance;
mod join;
mod meet;
mod orthogonal_antiprojection;
mod orthogonal_projection;
mod support;
mod transform;

#[doc(inline)]
pub use self::{functions::*, traits::*};

/// Collection of geometry operators as traits
pub mod traits {
  pub use super::{
    antisupport::Antisupport,
    central_antiprojection::CentralAntiprojection,
    central_projection::CentralProjection,
    compose::Compose,
    distance::{Distance, SignedDistance},
    join::Join,
    meet::Meet,
    orthogonal_antiprojection::OrthogonalAntiprojection,
    orthogonal_projection::OrthogonalProjection,
    support::Support,
    transform::Transform,
  };
}

/// Collection of geometry operators as free-functions
pub mod functions {
  pub use super::{
    antisupport::antisupport,
    central_antiprojection::central_antiprojection,
    central_projection::central_projection,
    compose::compose,
    distance::{distance, signed_distance},
    join::join,
    meet::meet,
    orthogonal_antiprojection::orthogonal_antiprojection,
    orthogonal_projection::orthogonal_projection,
    support::support,
    transform::transform,
  };
}
