use super::{Resolved, axial_compression, axial_tension, elastic_foundation, plain};
use crate::beams::state::State;

impl Resolved {
    /// `∫₀ˣ (x − u)·T(u)·e_V du`: response to a unit-slope ramp of upward
    /// distributed shear starting `x` behind the station.
    pub(crate) fn ramp_response(&self, x: f64) -> State {
        match self {
            Resolved::Plain {
                ei,
                shear_flexibility,
            } => plain::ramp_response::ramp_response(*ei, *shear_flexibility, x),
            Resolved::Foundation { modulus, beta, .. } => {
                elastic_foundation::ramp_response::ramp_response(*modulus, *beta, x)
            }
            Resolved::Compression { ei, k, .. } => {
                axial_compression::ramp_response::ramp_response(*ei, *k, x)
            }
            Resolved::Tension { ei, k, .. } => {
                axial_tension::ramp_response::ramp_response(*ei, *k, x)
            }
            Resolved::Variable { .. } => {
                unreachable!("the variable-section kernel is not shift-invariant")
            }
        }
    }
}
