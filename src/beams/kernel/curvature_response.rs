use super::{Resolved, axial_compression, axial_tension, elastic_foundation, plain};
use crate::beams::state::State;

impl Resolved {
    /// `∫₀ˣ T(u)·e_θ du`: response to a unit imposed curvature that started
    /// `x` behind the station.
    pub(crate) fn curvature_response(&self, x: f64) -> State {
        match self {
            Resolved::Plain { .. } => plain::curvature_response::curvature_response(x),
            Resolved::Foundation { ei, beta, .. } => {
                elastic_foundation::curvature_response::curvature_response(*ei, *beta, x)
            }
            Resolved::Compression { force, k, .. } => {
                axial_compression::curvature_response::curvature_response(*force, *k, x)
            }
            Resolved::Tension { force, k, .. } => {
                axial_tension::curvature_response::curvature_response(*force, *k, x)
            }
            Resolved::Variable { .. } => {
                unreachable!("the variable-section kernel is not shift-invariant")
            }
        }
    }
}
