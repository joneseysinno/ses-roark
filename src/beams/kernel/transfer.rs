use super::{
    Resolved, axial_compression, axial_tension, elastic_foundation, plain, variable_section,
};
use crate::beams::state::StateMatrix;

impl Resolved {
    /// Transfer matrix carrying the left-end state to station `x`.
    pub(crate) fn transfer(&self, x: f64) -> StateMatrix {
        match self {
            Resolved::Plain {
                ei,
                shear_flexibility,
            } => plain::transfer::transfer(*ei, *shear_flexibility, x),
            Resolved::Foundation { ei, beta, .. } => {
                elastic_foundation::transfer::transfer(*ei, *beta, x)
            }
            Resolved::Compression { ei, force, k } => {
                axial_compression::transfer::transfer(*ei, *force, *k, x)
            }
            Resolved::Tension { ei, force, k } => {
                axial_tension::transfer::transfer(*ei, *force, *k, x)
            }
            Resolved::Variable {
                stiffness,
                length,
                shear_flexibility,
                breakpoints,
            } => variable_section::transfer::transfer(
                stiffness,
                *length,
                *shear_flexibility,
                breakpoints,
                x,
            ),
        }
    }
}
