use super::beam::BeamSolution;

impl BeamSolution {
    /// Deflection of largest magnitude along the span (sign retained).
    pub fn max_deflection(&self) -> f64 {
        self.peak_deflection().value
    }
}
