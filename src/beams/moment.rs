use super::beam::BeamSolution;
use super::state::MOMENT;

impl BeamSolution {
    /// Bending moment `M` at station `x` (positive sagging).
    pub fn moment(&self, x: f64) -> f64 {
        self.state_at(x)[MOMENT]
    }
}
