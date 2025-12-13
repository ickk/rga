//! Collection of geometry object types; `Magnitude`, `Point`, `Line`, `Plane`,
//! ..

mod line;
mod magnitude;
mod plane;
mod point;

pub use self::{line::Line, magnitude::Magnitude, plane::Plane, point::Point};
