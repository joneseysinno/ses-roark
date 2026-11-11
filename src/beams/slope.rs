use super::beam::BeamSolution;
use super::state::SLOPE;

impl BeamSolution {
    /// Slope `θ = dy/dx` at station `x`. With shear rigidity set, this is the
    /// bending rotation of the section; the total slope is `θ − V/(κGA)`.
    pub fn slope(&self, x: f64) -> f64 {
        self.state_at(x)[SLOPE]
    }
}
