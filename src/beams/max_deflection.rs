use super::beam::BeamSolution;
use super::breakpoints::breakpoints;
use super::extremum::extremum;

impl BeamSolution {
    /// Deflection of largest magnitude along the span (sign retained).
    pub fn max_deflection(&self) -> f64 {
        extremum(|x| self.deflection(x), &breakpoints(&self.beam))
    }
}
