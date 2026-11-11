use super::beam::BeamSolution;
use super::evaluation_points::evaluation_points;
use super::extremum::extremum;
use super::peak::Peak;

impl BeamSolution {
    /// Deflection of largest magnitude, and where it occurs.
    pub fn peak_deflection(&self) -> Peak {
        extremum(|x| self.deflection(x), &evaluation_points(self))
    }
}
