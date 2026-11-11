use super::beam::BeamSolution;
use super::breakpoints::breakpoints;
use super::extremum::extremum;

impl BeamSolution {
    /// Bending moment of largest magnitude along the span (sign retained).
    pub fn max_moment(&self) -> f64 {
        extremum(|x| self.moment(x), &breakpoints(&self.beam))
    }
}
