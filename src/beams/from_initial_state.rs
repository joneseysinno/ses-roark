use super::beam::{Beam, BeamSolution};
use super::kernel::Resolved;
use super::state::State;

impl BeamSolution {
    /// Wrap a known left-end state (used by the solver, continuous beams, and frames).
    pub(crate) fn from_initial_state(beam: Beam, kernel: Resolved, initial: State) -> BeamSolution {
        let step = beam.length;
        BeamSolution {
            beam,
            kernel,
            initial,
            nodes: Vec::new(),
            step,
        }
    }
}
