use crate::algebra::values::OddGrade;

/// An improper isometry composed of a rotation and reflection
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Flector(pub OddGrade);
