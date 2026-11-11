use super::assemble_segmented::assemble_segmented;
use super::assemble_system::assemble_system;
use super::axial::first_critical_below;
use super::beam::{Beam, BeamSolution};
use super::error::BeamError;
use super::kernel::Resolved;
use super::segment_count::segment_count;
use super::solve_scaled::solve_scaled;
use super::solve_segmented::solve_segmented;
use super::state::State;
use super::validate::validate;

impl Beam {
    /// Solve the left-end state (reactions and integration constants).
    pub fn solve(&self) -> Result<BeamSolution, BeamError> {
        validate(self)?;
        let kernel = Resolved::from_beam(self)?;
        if let Resolved::Compression { force, .. } = kernel
            && let Some(critical) = first_critical_below(self, force)?
        {
            return Err(BeamError::AtOrAboveCriticalLoad { force, critical });
        }
        let n = segment_count(&kernel, self.length)?;
        if n == 1 {
            let (matrix, rhs) = assemble_system(&kernel, self);
            let initial = solve_scaled(matrix, rhs).ok_or(BeamError::UnstableSupports)?;
            return Ok(BeamSolution::from_initial_state(
                self.clone(),
                kernel,
                initial,
            ));
        }
        let (matrix, rhs) = assemble_segmented(&kernel, self, n);
        let unknowns = solve_segmented(matrix, rhs).ok_or(BeamError::UnstableSupports)?;
        let nodes: Vec<State> = (0..n)
            .map(|i| State::from_fn(|p, _| unknowns[4 * i + p]))
            .collect();
        if nodes
            .iter()
            .any(|state| state.iter().any(|v| !v.is_finite()))
        {
            return Err(BeamError::UnstableSupports);
        }
        let initial = nodes[0];
        Ok(BeamSolution {
            beam: self.clone(),
            kernel,
            initial,
            nodes,
            step: self.length / n as f64,
        })
    }
}
