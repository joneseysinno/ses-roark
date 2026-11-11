use super::beam::BeamSolution;
use super::evaluation_points::evaluation_points;
use super::extremum::extremum;
use super::peak::Peak;

impl BeamSolution {
    /// Bending moment of largest magnitude, and where it occurs.
    pub fn peak_moment(&self) -> Peak {
        extremum(|x| self.moment(x), &evaluation_points(self))
    }
}
