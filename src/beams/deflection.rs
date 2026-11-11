use super::beam::BeamSolution;
use super::state::DEFLECTION;

impl BeamSolution {
    /// Deflection `y` at station `x` (positive up).
    pub fn deflection(&self, x: f64) -> f64 {
        self.state_at(x)[DEFLECTION]
    }
}
