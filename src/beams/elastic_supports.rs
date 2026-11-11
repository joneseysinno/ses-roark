//! Deformation due to the elasticity of fixed supports (Roark 8.6).
//!
//! Elastic restraint is modeled with [`EndCondition::Elastic`](super::EndCondition::Elastic);
//! this module supplies spring constants for common support idealizations.

mod wall_rotational_stiffness;

pub use wall_rotational_stiffness::wall_rotational_stiffness;
