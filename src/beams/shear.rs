use super::beam::BeamSolution;
use super::state::SHEAR;

impl BeamSolution {
    /// Vertical shear `V` at station `x` (`V(0)` is the upward left reaction).
    pub fn shear(&self, x: f64) -> f64 {
        self.state_at(x)[SHEAR]
    }
}
