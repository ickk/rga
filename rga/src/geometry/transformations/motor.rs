use crate::algebra::values::EvenGrade;

/// A proper isometry composed of a rotation and translation
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Motor(pub EvenGrade);
