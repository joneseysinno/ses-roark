use super::assemble_system::assemble_system;
use super::axial::critical_load;
use super::beam::{Beam, BeamSolution};
use super::error::BeamError;
use super::kernel::Resolved;
use super::solve_scaled::solve_scaled;
use super::validate::validate;

impl Beam {
    /// Solve the left-end state (reactions and integration constants).
    pub fn solve(&self) -> Result<BeamSolution, BeamError> {
        validate(self)?;
        let kernel = Resolved::from_beam(self)?;
        if let Resolved::Compression { force, .. } = kernel {
            let critical = critical_load(self)?;
            if force >= critical {
                return Err(BeamError::AtOrAboveCriticalLoad { force, critical });
            }
        }
        let (matrix, rhs) = assemble_system(&kernel, self);
        let initial = solve_scaled(matrix, rhs).ok_or(BeamError::UnstableSupports)?;
        Ok(BeamSolution::from_initial_state(
            self.clone(),
            kernel,
            initial,
        ))
    }
}
