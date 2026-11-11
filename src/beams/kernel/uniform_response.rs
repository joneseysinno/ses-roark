use super::{Resolved, axial_compression, axial_tension, elastic_foundation, plain};
use crate::beams::state::State;

impl Resolved {
    /// `∫₀ˣ T(u)·e_V du`: response at distance `x` past the start of a unit
    /// upward distributed shear (shift-invariant kernels only).
    pub(crate) fn uniform_response(&self, x: f64) -> State {
        match self {
            Resolved::Plain {
                ei,
                shear_flexibility,
            } => plain::uniform_response::uniform_response(*ei, *shear_flexibility, x),
            Resolved::Foundation { ei, modulus, beta } => {
                elastic_foundation::uniform_response::uniform_response(*ei, *modulus, *beta, x)
            }
            Resolved::Compression { ei, k, .. } => {
                axial_compression::uniform_response::uniform_response(*ei, *k, x)
            }
            Resolved::Tension { ei, k, .. } => {
                axial_tension::uniform_response::uniform_response(*ei, *k, x)
            }
            Resolved::Variable { .. } => {
                unreachable!("the variable-section kernel is not shift-invariant")
            }
        }
    }
}
