use super::beam::BeamSolution;

impl BeamSolution {
    /// Bending moment of largest magnitude along the span (sign retained).
    pub fn max_moment(&self) -> f64 {
        self.peak_moment().value
    }
}
