use super::shift_invariant_load_terms::shift_invariant_load_terms;
use super::{Resolved, variable_section};
use crate::beams::load::Load;
use crate::beams::state::State;

impl Resolved {
    /// Load term `LT(load, x)` added to `T(x)·s_A`.
    pub(crate) fn load_terms(&self, load: &Load, x: f64) -> State {
        match self {
            Resolved::Variable {
                stiffness,
                length,
                shear_flexibility,
                breakpoints,
            } => variable_section::load_terms::load_terms(
                stiffness,
                *length,
                *shear_flexibility,
                breakpoints,
                load,
                x,
            ),
            _ => shift_invariant_load_terms(self, load, x),
        }
    }
}
